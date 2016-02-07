#[derive(Debug, Clone)]
pub struct RequestURI {
    base: String,
    query: String,
}

impl RequestURI {
    pub fn new(base: String, query: String) -> RequestURI {
        RequestURI { base: base, query: query }
    }

    pub fn url(self) -> String {
        let url = format!("{}{}", self.base, self.query);
        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use command_query::CommandQuery;
    use uri_base::URIBase;

    #[test]
    fn construct_request_uri() {
        let url = RequestURI::new("http://localhost:10041".to_string(),
                                  "/d/status".to_string()).url();
        assert_eq!("http://localhost:10041/d/status", url)
    }

    #[test]
    fn construct_with_actual_usage() {
        let uri_base = URIBase::new().build();
        let mut command = CommandQuery::new("select");
        command.set_argument(vec![("table", "Sites")]);
        let url = RequestURI::new(uri_base, command.encode()).url();
        assert_eq!("http://localhost:10041/d/select?table=Sites", url)
    }
}
