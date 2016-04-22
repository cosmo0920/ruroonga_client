Ruroonga Client
===

[![Build Status](https://travis-ci.org/cosmo0920/ruroonga_client.svg?branch=master)](https://travis-ci.org/cosmo0920/ruroonga_client)
[![](http://meritbadge.herokuapp.com/ruroonga_client)](https://crates.io/crates/ruroonga_client)

[Documentation](http://cosmo0920.github.io/ruroonga_client/ruroonga_client/index.html)

A tiny Groonga client via HTTP written by Rust language.

## Usage

Add following lines to your Cargo.toml:

```toml
[dependencies]
ruroonga_client = "~0.4.0"
```

and following lines to your crate root:

```rust
extern crate ruroonga_client;

use ruroonga_client as groonga;
```

## Example

```rust
extern crate ruroonga_client as groonga;

use groonga::builtin::command_query::CommandQuery;


fn main() {
    let mut request = groonga::HTTPRequest::new();
    let mut command = CommandQuery::new("select");
    command.set_argument(vec![("table", "Sites")]);
    let uri_base = groonga::URIBase::new().build();
    let url = groonga::RequestURI::new(uri_base, command.encode()).url();
    println!("url: {}", url);
    let res = request.get(url);
    let result = request.receive(&mut res.unwrap()).unwrap();
    println!("result: {}", result);
}
```

### Testing

Execute `cargo test`.

#### Advanced

If you encountered building failure which is dependent openssl library,
you should define several environment variables.

##### For OS X

You can use homebrewed openssl like this:

```bash
$ export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include
$ export DEP_OPENSSL_INCLUDE=/usr/local/opt/openssl/include
```
And then, `cargo build`.

##### For Windows

In more detail, see: https://github.com/sfackler/rust-openssl#windows

## LICENSE

[MIT](LICENSE).
