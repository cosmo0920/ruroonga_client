extern crate hyper;
extern crate url;
extern crate json_flex;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use hyper::header::ContentType;
use url::form_urlencoded;
use json_flex::{JFObject, Unwrap};

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

#[derive(Clone, Debug)]
pub struct ResultParser {
    result: Box<JFObject>
}

impl ResultParser {
    pub fn new(json: String) -> ResultParser {
        ResultParser{result: json_flex::decode(json)}
    }

    pub fn get_raw_object(&mut self) -> Box<JFObject> {
        self.result.clone()
    }

    pub fn get_header(&mut self) -> JFObject {
        self.result[0].clone()
    }

    pub fn status(&mut self) -> Option<&i64> {
        self.result[0][0].into_i64().clone()
    }

    pub fn start_time(&mut self) -> Option<&f64> {
        self.result[0][1].into_f64()
    }

    pub fn elapsed_time(&mut self) -> Option<&f64> {
        self.result[0][2].into_f64()
    }

    #[inline]
    fn matched_columns_num(&mut self) -> i64 {
        self.result[1][0][0]
            .unwrap_vec().clone()
            .pop().unwrap()
            .unwrap_i64().clone()
    }

    pub fn matched_columns(&mut self) -> Option<i64> {
        match self.status() {
            Some(&0) => Some(self.matched_columns_num()),
            Some(_) => Some(-1),
            None    => None
        }
    }

    pub fn result(&mut self) -> Option<Vec<JFObject>> {
        match self.status() {
            Some(&0) => Some(vec![self.result[1][0].clone()]),
            Some(_) => Some(vec![self.result[0][3].clone()]),
            None    => None
        }
    }
}
