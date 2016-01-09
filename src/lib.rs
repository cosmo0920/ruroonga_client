extern crate hyper;
extern crate url;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use hyper::header::ContentType;
use url::form_urlencoded;

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
                   -> Result<hyper::client::response::Response, hyper::error::Error> {
        // Creating an outgoing request.
        let res = self.client.get(&*url)
            .header(Connection::close())
            .send();
        res
    }

    pub fn load(&mut self, url_base: String, body: &str)
                   -> Result<hyper::client::response::Response, hyper::error::Error> {
        // Creating an loading data request via POST.
        let url = url_base + "/d/load";
        let res = self.client.post(&*url)
            .header(ContentType::json())
            .body(body)
            .send();
        res
    }

    pub fn receive(&mut self, res: &mut hyper::client::response::Response) -> String {
        // Read the Response.
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        body
    }
}

pub type Query<'a> = Vec<(&'a str, &'a str)>;

#[derive(Clone, Debug)]
pub struct CommandQuery<'a> {
    command: String,
    arguments: Query<'a>
}

impl<'a> CommandQuery<'a> {
    pub fn new(command: &str) -> CommandQuery {
        let arguments: Query = vec![];
        CommandQuery{command: command.to_string(), arguments: arguments}
    }

    pub fn get_command(&mut self) -> String {
        self.command.clone()
    }

    pub fn set_argument(&mut self, arguments: Query<'a>) {
        self.arguments = arguments
    }

    pub fn make_query(&mut self) -> String {
        form_urlencoded::serialize(self.arguments.clone().into_iter())
    }

    pub fn encode(&mut self) -> String {
        format!("/d/{}?{}", self.get_command(), self.make_query())
    }
}
