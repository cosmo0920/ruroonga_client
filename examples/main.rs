extern crate ruroonga_client;
extern crate json_flex;

use json_flex::Unwrap;

use ruroonga_client as groonga;

fn main() {
    let mut command = groonga::CommandQuery::new("select");
    command.set_argument(vec![("table", "Site")]);
    let mut request = groonga::HTTPRequest::new();
    let url = format!("http://localhost:10041{}", command.encode());
    println!("url: {}", url);
    let res = request.get(url);
    let result = request.receive(&mut res.unwrap());
    println!("result: {}", result);
    let mut decode = groonga::ResultParser::new(result);
    println!("status: {:?}", decode.status().unwrap());
    println!("start: {:?}", decode.start_time().unwrap());
    println!("elapsed: {:?}", decode.elapsed_time().unwrap());
    println!("matched columns: {:?}", decode.matched_columns().unwrap());
    let vec = decode.result().unwrap().pop().unwrap().unwrap_vec().clone();
    println!("index access: {:?}", vec[2]);
    // Read got response
    for v in decode.result().unwrap().pop().unwrap().unwrap_vec().iter() {
        println!("{:?}", v);
        for vv in v.unwrap_vec().iter() {
            if vv.is_integer() {
                println!("{:?}", vv.unwrap_i64())
            } else if vv.is_string() {
                println!("{:?}", vv.unwrap_string())
            } else {
                println!("{:?}", vv)
            }
        }
    }
}
