use url::form_urlencoded;
use std::borrow::Cow;

pub type Query<'a> = Vec<(&'a str, &'a str)>;

#[derive(Clone, Debug)]
pub struct CommandQuery<'a> {
    command: Cow<'a, str>,
    arguments: Query<'a>,
    prefix: Cow<'a, str>,
}

impl<'a> Default for CommandQuery<'a> {
    fn default() -> CommandQuery<'a> {
        CommandQuery {
            command: "".into(),
            arguments: vec![],
            prefix: "/d".into(),
        }
    }
}

impl<'a> CommandQuery<'a> {
    pub fn new<T>(command: T) -> CommandQuery<'a>
        where T: Into<Cow<'a, str>>
    {
        CommandQuery {
            command: command.into(),
            ..CommandQuery::default()
        }
    }

    /// Get vectorize `("key", "value")` pairs to construct url encoded query.
    pub fn get_command(&'a self) -> Cow<'a, str> {
        Cow::Borrowed(&self.command)
    }

    /// Set vectorize `("key", "value")` pairs to construct url encoded query.
    pub fn set_argument(&mut self, arguments: Query<'a>) {
        self.arguments = arguments.into()
    }

    #[doc(hidden)]
    // get HTTP URI prefix. default: /d
    // This function is mainly provided for internal usage.
    pub fn get_prefix(&'a self) -> Cow<'a, str> {
        Cow::Borrowed(&self.prefix)
    }

    #[doc(hidden)]
    // set HTTP URI prefix. This function is provided for advanced user.
    pub fn set_prefix<T>(&mut self, prefix: T)
        where T: Into<Cow<'a, str>>
    {
        self.prefix = prefix.into()
    }

    /// Create url encoded command query.
    ///
    /// `vec![("key","value")]` interprets to `"key=value"`.
    /// And two or more value pair are concatinate with `&`.
    pub fn make_query(&mut self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(self.arguments.clone())
            .finish()
    }

    ///
    /// Create Groonga HTTP server query URL.
    pub fn encode(&'a mut self) -> String {
        format!("{}/{}?{}",
                self.get_prefix().into_owned(),
                self.get_command().into_owned(),
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
