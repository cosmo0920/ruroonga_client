extern crate ruroonga_client;

use ruroonga_client as groonga;

fn main() {
    let mut command = groonga::CommandQuery::new("select");
    command.set_argument(vec![("table", "Entry")]);
    let mut request = groonga::HTTPRequest::new();
    let url = format!("http://localhost:10041{}", command.encode());
    println!("url: {}", url);
    let res = request.get(url);
    let result = request.receive(&mut res.unwrap());
    println!("result: {}", result);
}
