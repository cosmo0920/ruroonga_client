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
    /// Create URIBase struct.
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

    /// Set base to replace default value with specified value.
    pub fn base_uri(mut self, base_uri: String) -> URIBase {
        self.base_uri = base_uri;
        self
    }

    /// Set port number to replace default value with specified value.
    pub fn port(mut self, port: u16) -> URIBase {
        self.port = port;
        self
    }

    /// Build and get base uri.
    pub fn build(self) -> String {
        format!("http://{}:{}", self.base_uri, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_uri_default() {
        let uri_base = URIBase::new().build();
        assert_eq!("http://localhost:10041", uri_base)
    }

    #[test]
    fn buikld_only_uri_base() {
        let uri_base = URIBase::new()
            .base_uri("127.0.0.1".to_string()).build();
        assert_eq!("http://127.0.0.1:10041", uri_base);
    }

    #[test]
    fn build_only_port() {
        let uri_base = URIBase::new().port(10042).build();
        assert_eq!("http://localhost:10042", uri_base);
    }

    #[test]
    fn uri_with_builder() {
        let uri_base = URIBase::new()
            .base_uri("127.0.1.1".to_string())
            .port(10043)
            .build();
        assert_eq!("http://127.0.1.1:10043", uri_base)
    }

}
