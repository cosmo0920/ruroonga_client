use std::io;
use std::io::Cursor;
use std::io::prelude::*;
use std::borrow::Cow;
use std::net::TcpStream;
use std::string::FromUtf8Error;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

const RECV_BUF_SIZE: usize = 8192;
const GQTP_HEADER_SIZE: usize = 24;

#[derive(Debug)]
pub enum GQTPError {
    InvalidProtocol,
    InvalidBodySize,
    StatusError(u16),
    IO(io::Error),
    EncodingError(FromUtf8Error),
}

impl From<io::Error> for GQTPError {
    fn from(err: io::Error) -> GQTPError {
        GQTPError::IO(err)
    }
}

impl From<FromUtf8Error> for GQTPError {
    fn from(err: FromUtf8Error) -> GQTPError {
        GQTPError::EncodingError(err)
    }
}

/// Request [GQTP protocol](http://groonga.org/docs/spec/gqtp.html) over TcpStream
pub struct GQTPRequest<'a> {
    addr: Cow<'a, str>,
}

impl<'a> Default for GQTPRequest<'a> {
    fn default() -> GQTPRequest<'a> {
        GQTPRequest { addr: Cow::Borrowed("127.0.0.1:10043") }
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
    pub fn with_addr<T>(mut self, addr: T) -> GQTPRequest<'a>
        where T: Into<Cow<'a, str>>
    {
        self.addr = addr.into();
        self
    }

    /// Send request and Receive response.
    pub fn call<C>(&self, command: C) -> Result<String, GQTPError>
        where C: AsRef<str>
    {
        // send
        let mut stream = try!(TcpStream::connect(self.addr.as_ref()));
        let mut send_buf = vec![];
        try!(send_buf.write_u8(0xc7));
        try!(send_buf.write_u8(0));
        try!(send_buf.write_i16::<BigEndian>(0));
        try!(send_buf.write_u8(0));
        try!(send_buf.write_u8(0x02)); // flags
        try!(send_buf.write_u16::<BigEndian>(0));
        try!(send_buf.write_u32::<BigEndian>(command.as_ref().len() as u32));
        try!(send_buf.write_u32::<BigEndian>(0));
        try!(send_buf.write_u64::<BigEndian>(0));
        send_buf.extend_from_slice(command.as_ref().as_bytes());
        let _ = stream.write_all(send_buf.as_slice());

        // receive and check protocol header value
        let mut read_buf = vec![0; RECV_BUF_SIZE];
        let _ = stream.read(&mut read_buf);
        let mut buf = Cursor::new(read_buf);

        let protocol = try!(buf.read_u8());
        let query_type = try!(buf.read_u8());
        if protocol != 0xc7 || query_type > 5 {
            return Err(GQTPError::InvalidProtocol);
        }
        let _ = try!(buf.read_i16::<BigEndian>());
        let _ = try!(buf.read_u8());

        let flags = try!(buf.read_u8());
        if !((flags & 0x01) == 0x01 || (flags & 0x02) == 0x02) {
            return Err(GQTPError::InvalidProtocol);
        }

        let status = try!(buf.read_u16::<BigEndian>());
        if status != 0 && status != 1 {
            return Err(GQTPError::StatusError(status));
        }
        let size = try!(buf.read_i32::<BigEndian>()) as usize;
        let _ = try!(buf.read_i32::<BigEndian>()); // opaque
        let _ = try!(buf.read_i64::<BigEndian>()); // cas

        // read body
        let mut msg_buf_len = if (size + GQTP_HEADER_SIZE) > RECV_BUF_SIZE {
            RECV_BUF_SIZE - GQTP_HEADER_SIZE
        } else {
            size
        };
        let mut msg = vec![0; msg_buf_len];
        let _ = try!(buf.read(&mut msg));
        if (size + GQTP_HEADER_SIZE) > RECV_BUF_SIZE {
            loop {
                let mut read_buf = vec![0; RECV_BUF_SIZE];
                let rsize = try!(stream.read(&mut read_buf));
                msg.extend_from_slice(read_buf.as_ref());
                msg_buf_len += rsize;
                if msg_buf_len >= size {
                    break;
                }
            }
        }

        Ok(try!(String::from_utf8(msg)))
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

    #[test]
    fn smoke_gqtp_with_addr_string() {
        let req = GQTPRequest::new().with_addr("127.0.0.1:20043".to_string());
        assert_eq!("127.0.0.1:20043", req.addr)
    }
}
