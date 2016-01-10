extern crate hyper;
extern crate url;
extern crate json_flex;

use std::io::Read;

use hyper::Client;
use hyper::client::response::Response;
use hyper::error::Error as HyperError;
use hyper::header::{Connection, ContentType};
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

#[cfg(test)]
mod command_query_test {
    use super::*;

    #[test]
    fn construct_query() {
        let mut command = CommandQuery::new("select");
        command.set_argument(vec![("table", "Site")]);
        let url_encoded = "/d/select?table=Site";
        assert_eq!(url_encoded, command.encode());
    }

    #[test]
    fn construct_complex_query() {
        let mut command = CommandQuery::new("select");
        command.set_argument(vec![("table", "Site"),
                                  ("--query","\'_key:\"http://example.org/\"\'")]);
        let url_encoded = "/d/select?table=Site&--query=%27_key%3A%22http%3A%2F%2Fexample.org%2F%22%27";
        assert_eq!(url_encoded, command.encode());
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
        self.result[0][0].into_i64()
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
            Some(_) => None,
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

#[cfg(test)]
mod result_parser_test {
    use super::*;

    #[test]
    fn parse_result() {
        let response = "
    [[0,1452348610.39281,0.000101566314697266],
    [[[9],[[\"_id\",\"UInt32\"],[\"_key\",\"ShortText\"],[\"title\",\"ShortText\"]],
    [1,\"http://example.org/\",\"This is test record 1!\"],
    [2,\"http://example.net/\",\"test record 2.\"],
    [3,\"http://example.com/\",\"test test record three.\"],
    [4,\"http://example.net/afr\",\"test record four.\"],
    [5,\"http://example.org/aba\",\"test test test record five.\"],
    [6,\"http://example.com/rab\",\"test test test test record six.\"],
    [7,\"http://example.net/atv\",\"test test test record seven.\"],
    [8,\"http://example.org/gat\",\"test test record eight.\"],
    [9,\"http://example.com/vdw\",\"test test record nine.\"]]]]";
        let mut decode = ResultParser::new(response.to_string());
        assert_eq!(&0, decode.status().unwrap());
        assert_eq!(&1452348610.39281, decode.start_time().unwrap());
        assert_eq!(&0.000101566314697266, decode.elapsed_time().unwrap());
        assert_eq!(9, decode.matched_columns().unwrap());
        let vec = decode.result().unwrap().pop().unwrap().unwrap_vec().clone();
        let expected = r#"Array([Integer(1), String("http://example.org/"), String("This is test record 1!")])"#.to_owned();
        assert_eq!(expected, format!("{:?}", vec[2]))
    }
}
