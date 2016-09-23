use std::io::Cursor;
use std::io::prelude::*;
use std::net::TcpStream;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

#[derive(Debug)]
pub enum GQTPError {
    InvalidProtocol,
    InvalidBodySize,
    StatusError(u16),
}

pub struct GQTPRequest<'a> {
    addr: &'a str,
}

impl<'a> Default for GQTPRequest<'a> {
    fn default() -> GQTPRequest<'a> {
        GQTPRequest { addr: "127.0.0.1:10043" }
    }
}

impl<'a> GQTPRequest<'a> {
    pub fn new() -> GQTPRequest<'a> {
        GQTPRequest::default()
    }

    pub fn with_addr(addr: &'a str) -> GQTPRequest<'a> {
        GQTPRequest { addr: addr }
    }

    pub fn call<C>(&self, command: C) -> Result<String, GQTPError>
        where C: AsRef<str>
    {
        // send
        let mut stream = TcpStream::connect(self.addr).unwrap();
        let mut send_buf = vec![];
        send_buf.write_u8(0xc7).unwrap();
        send_buf.write_u8(0).unwrap();
        send_buf.write_i16::<BigEndian>(0).unwrap();
        send_buf.write_u8(0).unwrap();
        send_buf.write_u8(0x02).unwrap();   // flags
        send_buf.write_u16::<BigEndian>(0).unwrap();
        send_buf.write_u32::<BigEndian>(command.as_ref().len() as u32).unwrap();
        send_buf.write_u32::<BigEndian>(0).unwrap();
        send_buf.write_u64::<BigEndian>(0).unwrap();
        send_buf.extend_from_slice(command.as_ref().as_bytes());
        let _ = stream.write_all(send_buf.as_slice());

        // recv and check protocol header value
        let mut read_buf = vec![0; 1024];
        let _ = stream.read(&mut read_buf);
        let mut buf = Cursor::new(read_buf);

        let protocol = buf.read_u8().unwrap();
        let query_type = buf.read_u8().unwrap();
        if protocol != 0xc7 || query_type > 5 {
            return Err(GQTPError::InvalidProtocol);
        }
        let _ = buf.read_i16::<BigEndian>().unwrap();
        let _ = buf.read_u8().unwrap();

        let flags = buf.read_u8().unwrap();
        if !((flags & 0x01) == 0x01 || (flags & 0x02) == 0x02) {
            return Err(GQTPError::InvalidProtocol);
        }

        let status = buf.read_u16::<BigEndian>().unwrap();
        if status != 0 && status != 1 {
            return Err(GQTPError::StatusError(status));
        }
        let size = buf.read_i32::<BigEndian>().unwrap();
        let _ = buf.read_i32::<BigEndian>().unwrap();    // opaque
        let _ = buf.read_i64::<BigEndian>().unwrap();    // cas
        let mut msg = vec![0; size as usize];
        let _ = buf.read_to_end(&mut msg);

        Ok(String::from_utf8(msg).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_gqtp() {
        let req = GQTPRequest::new();
        assert_eq!("127.0.0.1:10043", req.addr)
    }

    #[test]
    fn smoke_gqtp_with_addr() {
        let req = GQTPRequest::with_addr("127.0.0.1:20043");
        assert_eq!("127.0.0.1:20043", req.addr)
    }

    #[test]
    fn smoke_gqtp_call() {
        let req = GQTPRequest::new();
        // req.call("select --table Entries --filter \'content @ \"fast\"\'");
        let result_string = req.call("status").unwrap();
        assert_eq!(result_string.contains("version"), true);
    }
}
