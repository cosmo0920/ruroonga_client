use std::io;
use std::io::Read;
use hyper::Client;
use hyper::client::response::Response;
use hyper::error::Error as HyperError;
use hyper::header::{Connection, ContentType};

pub struct HTTPRequest {
    client: Client,
}

impl HTTPRequest {
    /// Create a HTTP client.
    pub fn new() -> HTTPRequest {
        let client = Client::new();
        HTTPRequest{client: client}
    }

    /// Creating an outgoing request with HTTP.
    pub fn get(&mut self, url: String)
                   -> Result<Response, HyperError> {
        let res = self.client.get(&*url)
            .header(Connection::close())
            .send();
        res
    }

    /// Creating an loading data request via POST.
    pub fn load(&mut self, url: String, body: String)
                   -> Result<Response, HyperError> {
        let res = self.client.post(&*url)
            .header(ContentType::json())
            .body(&*body)
            .send();
        res
    }

    /// Read the Response.
    pub fn receive(&mut self, res: &mut Response)
                   -> Result<String, io::Error> {
        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        Ok(body)
    }
}
