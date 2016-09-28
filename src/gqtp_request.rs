use std::io::Cursor;
use std::io::prelude::*;
use std::net::TcpStream;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

const RECV_BUF_SIZE: usize = 8192;
const GQTP_HEADER_SIZE: usize = 24;

#[derive(Debug)]
pub enum GQTPError {
    InvalidProtocol,
    InvalidBodySize,
    StatusError(u16),
}

/// Request [GQTP protocol](http://groonga.org/docs/spec/gqtp.html) over TcpStream
pub struct GQTPRequest<'a> {
    addr: &'a str,
}

impl<'a> Default for GQTPRequest<'a> {
    fn default() -> GQTPRequest<'a> {
        GQTPRequest { addr: "127.0.0.1:10043" }
    }
}

impl<'a> GQTPRequest<'a> {
    /// Create a GQTP client.
    pub fn new() -> GQTPRequest<'a> {
        GQTPRequest::default()
    }

    /// Set host address for GQTP server.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate ruroonga_client as groonga;
    ///
    /// groonga::GQTPRequest::new().with_addr("127.0.0.1:20043");
    /// ```
    pub fn with_addr(mut self, addr: &'a str) -> GQTPRequest<'a> {
        self.addr = addr;
        self
    }

    /// Send request and Receive response.
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

        // receive and check protocol header value
        let mut read_buf = vec![0; RECV_BUF_SIZE];
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

        // read body
        let mut msg_buf_len = if (size as usize + GQTP_HEADER_SIZE) > RECV_BUF_SIZE {
            RECV_BUF_SIZE - GQTP_HEADER_SIZE
        } else {
            size as usize
        };
        let mut msg = vec![0; msg_buf_len];
        let _ = buf.read(&mut msg).unwrap();
        if (size as usize + GQTP_HEADER_SIZE) > RECV_BUF_SIZE {
            loop {
                let mut read_buf = vec![0; RECV_BUF_SIZE];
                let rsize = stream.read(&mut read_buf).unwrap();
                msg.extend_from_slice(read_buf.as_ref());
                msg_buf_len += rsize;
                if msg_buf_len >= size as usize {
                    break;
                }
            }
        }

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
        let req = GQTPRequest::new().with_addr("127.0.0.1:20043");
        assert_eq!("127.0.0.1:20043", req.addr)
    }
}
