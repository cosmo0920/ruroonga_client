use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct URIBase<'a> {
    base_uri: Cow<'a, str>,
    port: u16,
}

impl<'a> Default for URIBase<'a> {
    fn default() -> URIBase<'a> {
        URIBase {
            base_uri: "localhost".into(),
            port: 10041,
        }
    }
}

impl<'a> URIBase<'a> {
    ///
    /// Create URIBase struct.
    ///
    /// Default values are:
    ///
    ///   base_uri: "localhost"
    ///
    ///   port: 10041
    ///
    pub fn new() -> URIBase<'a> {
        URIBase::default()
    }

    /// Set base to replace default value with specified value.
    pub fn base_uri<T>(mut self, base_uri: T) -> URIBase<'a>
        where T: Into<Cow<'a, str>>
    {
        self.base_uri = base_uri.into();
        self
    }

    /// Set port number to replace default value with specified value.
    pub fn port(mut self, port: u16) -> URIBase<'a> {
        self.port = port;
        self
    }

    /// Build and get base uri.
    pub fn build(self) -> String {
        format!("http://{}:{}", self.base_uri.into_owned(), self.port)
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
    fn build_only_uri_base() {
        let uri_base = URIBase::new().base_uri("127.0.0.1").build();
        assert_eq!("http://127.0.0.1:10041", uri_base);
    }

    #[test]
    fn build_only_uri_base_with_owned_str() {
        let uri_base = URIBase::new().base_uri("127.0.0.1".to_owned()).build();
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
