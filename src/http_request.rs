use std::io;
use std::io::Read;
use hyper::Client;
use hyper::client::response::Response;
use hyper::error::Error as HyperError;
use hyper::header::{Connection, ContentType, ContentLength, Headers, Authorization, Basic};
use std::option::Option;

pub struct HTTPRequest {
    client: Client,
    user: String,
    password: Option<String>,
    auth: bool,
}

impl Default for HTTPRequest {
    fn default() -> HTTPRequest {
        HTTPRequest {
            client: Client::new(),
            user: "".to_string(),
            password: None,
            auth: false,
        }
    }
}

impl HTTPRequest {
    /// Create a HTTP client.
    pub fn new() -> HTTPRequest {
        HTTPRequest::default()
    }

    /// Set authentication information.
    ///
    /// Note that this method also sets `auth: true` to use basic authentication.
    ///
    /// And this method uses builder pattern and returns `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate ruroonga_client as groonga;
    ///
    /// groonga::HTTPRequest::new()
    ///   .authenticate("user".to_string(), "password".to_string());
    /// ```
    pub fn authenticate(mut self, user: String, password: String) -> HTTPRequest {
        self.user = user;
        self.password = Some(password);
        self.auth = true;
        self
    }

    /// Creating an outgoing request with HTTP.
    pub fn get(&mut self, url: String) -> Result<Response, HyperError> {
        let mut headers = Headers::new();
        if self.auth {
            headers.set(Authorization(Basic {
                username: self.user.clone(),
                password: self.password.clone(),
            }));
        }
        headers.set(Connection::close());
        self.client
            .get(&*url)
            .headers(headers)
            .send()
    }

    /// Creating an loading data request via POST.
    pub fn load(&mut self, url: String, body: String) -> Result<Response, HyperError> {
        let mut headers = Headers::new();
        if self.auth {
            headers.set(Authorization(Basic {
                username: self.user.clone(),
                password: self.password.clone(),
            }));
        }
        headers.set(ContentType::json());
        headers.set(ContentLength(body.len() as u64));
        self.client
            .post(&*url)
            .headers(headers)
            .body(&*body)
            .send()
    }

    /// Read the Response.
    pub fn receive(&mut self, res: &mut Response) -> Result<String, io::Error> {
        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_auth() {
        let req = HTTPRequest::new().authenticate("user".to_string(), "password".to_string());
        assert_eq!(true, req.auth)
    }

    #[test]
    fn dont_use_auth() {
        let req = HTTPRequest::new();
        assert_eq!(false, req.auth)
    }
}
