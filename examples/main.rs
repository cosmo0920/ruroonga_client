extern crate ruroonga_client;
extern crate json_flex;

use json_flex::Unwrap;

use ruroonga_client as groonga;

fn load() {
    let data = r#"
[
{"_key":"http://example.org/","title":"This is test record 1!"},
{"_key":"http://example.net/","title":"test record 2."},
{"_key":"http://example.com/","title":"test test record three."},
{"_key":"http://example.net/afr","title":"test record four."},
{"_key":"http://example.org/aba","title":"test test test record five."},
{"_key":"http://example.com/rab","title":"test test test test record six."},
{"_key":"http://example.net/atv","title":"test test test record seven."},
{"_key":"http://example.org/gat","title":"test test record eight."},
{"_key":"http://example.com/vdw","title":"test test record nine."},
]"#;
    let mut request = groonga::HTTPRequest::new();
    let mut load_command = groonga::CommandQuery::new("load");
    load_command.set_argument(vec![("table", "Sites"),
                                   ("input_type","json"), ("values", data)]);
    let load_url = format!("http://localhost:10041{}", load_command.encode());
    println!("load url: {}", load_url);
    let load_res = request.load(load_url);
    let load_result = request.receive(&mut load_res.unwrap());
    println!("result: {}", load_result);
}

fn main() {
    load();
    let mut request = groonga::HTTPRequest::new();
    let mut command = groonga::CommandQuery::new("select");
    command.set_argument(vec![("table", "Site")]);
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
