#[derive(Clone, Debug)]
pub struct URIBase {
    base_uri: String,
    port: u16
}

impl Default for URIBase {
    fn default() -> URIBase {
        URIBase {
            base_uri: "localhost".to_string(),
            port: 10041
        }
    }
}

impl URIBase {
    ///
    /// Create URIBase struct
    ///
    /// Default values are:
    ///
    ///   base_uri: "localhost"
    ///
    ///   port: 10041
    ///
    pub fn new() -> URIBase {
        let default: URIBase = Default::default();
        default
    }

    /// Set base_uri to replace default value with specified value
    pub fn set_uri(&mut self, base_uri: String) {
        self.base_uri = base_uri
    }

    /// Set port number to replace default value with specified value
    pub fn set_port(&mut self, port: u16) {
        self.port = port
    }

    /// make base uri
    /// Default value is: "localhost:10041"
    pub fn uri(&mut self) -> String {
        format!("http://{}:{}", self.base_uri, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_uri_default() {
        let mut uri_base = URIBase::new();
        let uri_default = uri_base.uri();
        assert_eq!("http://localhost:10041", uri_default)
    }

    #[test]
    fn construct_uri_accessors() {
        let mut uri_base = URIBase::new();
        uri_base.set_uri("127.0.0.1".to_string());
        assert_eq!("http://127.0.0.1:10041", uri_base.uri());
        uri_base.set_port(10042);
        assert_eq!("http://127.0.0.1:10042", uri_base.uri());
    }
}
