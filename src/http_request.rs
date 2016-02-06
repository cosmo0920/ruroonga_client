use std::io;
use std::io::Read;
use hyper::Client;
use hyper::client::response::Response;
use hyper::error::Error as HyperError;
use hyper::header::{Connection, ContentType, Headers, Authorization, Basic};
use std::option::Option;

pub struct HTTPRequest {
    client: Client,
    user: String,
    password: Option<String>,
    auth: bool
}

impl Default for HTTPRequest {
    fn default() -> HTTPRequest {
        HTTPRequest {
            client: Client::new(),
            user: "".to_string(),
            password: None,
            auth: false
        }
    }
}

impl HTTPRequest {
    /// Create a HTTP client.
    pub fn new() -> HTTPRequest {
        let default: HTTPRequest = Default::default();
        default
    }

    pub fn authenticate(mut self, user: String, password: String)
                        -> HTTPRequest{
        self.user = user;
        self.password = Some(password);
        self.auth = true;
        self
    }

    /// Creating an outgoing request with HTTP.
    pub fn get(&mut self, url: String)
               -> Result<Response, HyperError> {
        if self.auth {
            let mut headers = Headers::new();
            headers.set(Authorization(
                Basic { username: self.user.clone(),
                        password: self.password.clone() }));
            headers.set(Connection::close());
            let res = self.client.get(&*url).headers(headers).send();
            res
        } else {
            let res = self.client.get(&*url)
                .header(Connection::close())
                .send();
            res
        }
    }

    /// Creating an loading data request via POST.
    pub fn load(&mut self, url: String, body: String)
                -> Result<Response, HyperError> {
        if self.auth {
            let mut headers = Headers::new();
            headers.set(Authorization(
                Basic { username: self.user.clone(),
                        password: self.password.clone() }));
            headers.set(ContentType::json());
            let res = self.client.post(&*url)
                .headers(headers).body(&*body).send();
            res
        } else {
            let res = self.client.post(&*url)
                .header(ContentType::json())
                .body(&*body)
                .send();
            res
        }
    }

    /// Read the Response.
    pub fn receive(&mut self, res: &mut Response)
                   -> Result<String, io::Error> {
        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        Ok(body)
    }
}
