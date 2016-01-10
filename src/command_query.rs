use url::form_urlencoded;

pub type Query<'a> = Vec<(&'a str, &'a str)>;

#[derive(Clone, Debug)]
pub struct CommandQuery<'a> {
    command: String,
    arguments: Query<'a>
}

impl<'a> CommandQuery<'a> {
    pub fn new(command: &str) -> CommandQuery {
        let arguments: Query = vec![];
        CommandQuery{command: command.to_string(), arguments: arguments}
    }

    pub fn get_command(&mut self) -> String {
        self.command.clone()
    }

    pub fn set_argument(&mut self, arguments: Query<'a>) {
        self.arguments = arguments
    }

    pub fn make_query(&mut self) -> String {
        form_urlencoded::serialize(self.arguments.clone().into_iter())
    }

    pub fn encode(&mut self) -> String {
        format!("/d/{}?{}", self.get_command(), self.make_query())
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
                                  ("--query","\'_key:\"http://example.org/\"\'")]);
        let url_encoded = "/d/select?table=Site&--query=%27_key%3A%22http%3A%2F%2Fexample.org%2F%22%27";
        assert_eq!(url_encoded, command.encode());
    }
}
