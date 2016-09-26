extern crate ruroonga_client as groonga;
extern crate json_flex;

fn main() {
    let req = groonga::GQTPRequest::new();
    let result_string = req.call("status").unwrap();
    println!("{:?}", result_string);

    let data = json_flex::decode(result_string);
    println!("{:?}", data);
}
