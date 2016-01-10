use std::io::Read;
use hyper::Client;
use hyper::client::response::Response;
use hyper::error::Error as HyperError;
use hyper::header::{Connection, ContentType};

pub struct HTTPRequest {
    client: Client,
}

impl HTTPRequest {
    pub fn new() -> HTTPRequest {
        // Create a client.
        let client = Client::new();
        HTTPRequest{client: client}
    }

    pub fn get(&mut self, url: String)
                   -> Result<Response, HyperError> {
        // Creating an outgoing request.
        let res = self.client.get(&*url)
            .header(Connection::close())
            .send();
        res
    }

    pub fn load(&mut self, url: String, body: String)
                   -> Result<Response, HyperError> {
        // Creating an loading data request via POST.
        let res = self.client.post(&*url)
            .header(ContentType::json())
            .body(&*body)
            .send();
        res
    }

    pub fn receive(&mut self, res: &mut Response) -> String {
        // Read the Response.
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        body
    }
}
