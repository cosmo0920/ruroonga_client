Ruroonga Client
===

[![Build Status](https://travis-ci.org/cosmo0920/ruroonga_client.svg?branch=master)](https://travis-ci.org/cosmo0920/ruroonga_client)
[![](http://meritbadge.herokuapp.com/ruroonga_client)](https://crates.io/crates/ruroonga_client)
[![Build status](https://ci.appveyor.com/api/projects/status/2hrd5g937u7uw8nl/branch/master?svg=true)](https://ci.appveyor.com/project/cosmo0920/ruroonga-client/branch/master)

[Documentation](http://cosmo0920.github.io/ruroonga_client/ruroonga_client/index.html)

A tiny Groonga client mainly via HTTP written by Rust language.

GQTP protocol support is experimental for now.

## Usage

Add following lines to your Cargo.toml:

```toml
[dependencies]
ruroonga_client = "~0.5.0"
```

and following lines to your crate root:

```rust
extern crate ruroonga_client;

use ruroonga_client as groonga;
```

### Using GQTP protocol

If you want to use GQTP module, please specify the dependency as follows for now:

```toml
[dependencies.ruroonga_client]
version = "~0.5.0"
features = ["gqtp"]
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

## LICENSE

[MIT](LICENSE).
