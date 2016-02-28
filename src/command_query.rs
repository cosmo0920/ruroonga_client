use url::form_urlencoded;

pub type Query<'a> = Vec<(&'a str, &'a str)>;

#[derive(Clone, Debug)]
pub struct CommandQuery<'a> {
    command: String,
    arguments: Query<'a>,
    prefix: String,
}

impl<'a> Default for CommandQuery<'a> {
    fn default() -> CommandQuery<'a> {
        CommandQuery {
            command: "".to_string(),
            arguments: vec![],
            prefix: "/d".to_string(),
        }
    }
}

impl<'a> CommandQuery<'a> {
    pub fn new(command: &str) -> CommandQuery {
        let default: CommandQuery = Default::default();
        CommandQuery {
            command: command.to_string(),
            arguments: default.arguments,
            prefix: default.prefix,
        }
    }

    /// Get vectorize `("key", "value")` pairs to construct url encoded query.
    pub fn get_command(&mut self) -> String {
        self.command.clone()
    }

    /// Set vectorize `("key", "value")` pairs to construct url encoded query.
    pub fn set_argument(&mut self, arguments: Query<'a>) {
        self.arguments = arguments
    }

    #[doc(hidden)]
    // get HTTP URI prefix. default: /d
    // This function is mainly provided for internal usage.
    pub fn get_prefix(&mut self) -> String {
        self.prefix.clone()
    }

    #[doc(hidden)]
    // set HTTP URI prefix. This function is provided for advanced user.
    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix
    }

    /// Create url encoded command query.
    ///
    /// `vec![("key","value")]` interprets to `"key=value"`.
    /// And two or more value pair are concatinate with `&`.
    pub fn make_query(&mut self) -> String {
        form_urlencoded::serialize(self.arguments.clone().into_iter())
    }

    ///
    /// Create Groonga HTTP server query URL.
    pub fn encode(&mut self) -> String {
        format!("{}/{}?{}",
                self.get_prefix(),
                self.get_command(),
                self.make_query())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_query() {
        let mut command = CommandQuery::new("select");
        command.set_argument(vec![("table", "Site")]);
        let url_encoded = "/d/select?table=Site";
        assert_eq!(url_encoded, command.encode());
    }

    #[test]
    fn construct_complex_query() {
        let mut command = CommandQuery::new("select");
        command.set_argument(vec![("table", "Site"),
                                  ("query", "\'_key:\"http://example.org/\"\'")]);
        let url_encoded = "/d/select?table=Site&query=%27_key%3A%22http%3A%2F%2Fexample.\
                           org%2F%22%27";
        assert_eq!(url_encoded, command.encode());
    }
}
