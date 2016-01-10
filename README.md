Ruroonga Client
===

[![Build Status](https://travis-ci.org/cosmo0920/ruroonga_client.svg?branch=master)](https://travis-ci.org/cosmo0920/ruroonga_client)

A tiny Groonga client written by Rust language.

## Usage

Add this to your Cargo.toml:

```
[dependencies]
ruroonga_client = "*"
```

and this to your crate root:

```rust
extern crate ruroonga_client;

use ruroonga_client as groonga;
```

## Example

```rust
extern crate ruroonga_client;

use ruroonga_client as groonga;

fn main() {
    let mut request = groonga::HTTPRequest::new();
    let mut command = groonga::CommandQuery::new("select");
    command.set_argument(vec![("table", "Sites")]);
    let url = format!("http://localhost:10041{}", command.encode());
    println!("url: {}", url);
    let res = request.get(url);
    let result = request.receive(&mut res.unwrap()).unwrap();
    println!("result: {}", result);
}
```

## LICENSE

[MIT](LICENSE).
