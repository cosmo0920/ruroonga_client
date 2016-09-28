extern crate ruroonga_client as groonga;
extern crate json_flex;

#[cfg(feature="gqtp")]
fn inner() {
    let req = groonga::GQTPRequest::new();
    let result_string = req.call("status").unwrap();
    println!("{:?}", result_string);

    let data = json_flex::decode(result_string);
    println!("{:?}", data);
}
#[cfg(not(feature="gqtp"))]
fn inner() {
    println!("exec after that build to add option '--features=gqtp'")
}

fn main() {
    inner();
}
