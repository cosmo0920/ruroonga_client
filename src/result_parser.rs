use json_flex;
use json_flex::{JFObject, Unwrap};

#[derive(Clone)]
pub struct Rows {
    data: Option<Vec<JFObject>>
}

impl Rows {
    pub fn new(data: Option<Vec<JFObject>>) -> Rows {
        Rows { data: data }
    }

    pub fn columns(&mut self) -> Option<Vec<JFObject>> {
        let popable = match self.data.clone() {
            Some(v) => v,
            None => return None
        };
        let pop = match popable.clone().pop() {
            Some(v) => v,
            None => return None
        };
        match pop.into_vec().clone() {
            Some(v) => return Some(v.clone()),
            None => return None
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResultParser {
    result: Box<JFObject>
}

impl ResultParser {
    pub fn new(json: String) -> ResultParser {
        ResultParser{result: json_flex::decode(json)}
    }

    pub fn get_raw_object(&mut self) -> Box<JFObject> {
        self.result.clone()
    }

    pub fn get_header(&mut self) -> JFObject {
        self.result[0].clone()
    }

    pub fn status(&mut self) -> Option<&i64> {
        self.result[0][0].into_i64()
    }

    pub fn start_time(&mut self) -> Option<&f64> {
        self.result[0][1].into_f64()
    }

    pub fn elapsed_time(&mut self) -> Option<&f64> {
        self.result[0][2].into_f64()
    }

    #[inline]
    fn matched_columns_num(&mut self) -> i64 {
        self.result[1][0][0]
            .unwrap_vec().clone()
            .pop().unwrap()
            .unwrap_i64().clone()
    }

    pub fn matched_columns(&mut self) -> Option<i64> {
        match self.status() {
            Some(&0) => Some(self.matched_columns_num()),
            Some(_) => None,
            None    => None
        }
    }

    pub fn result(&mut self) -> Option<Vec<JFObject>> {
        match self.status() {
            Some(&0) => Some(vec![self.result[1][0].clone()]),
            Some(_) => Some(vec![self.result[0][3].clone()]),
            None    => None
        }
    }

    pub fn into_row(&mut self) -> Rows {
        Rows::new(self.result())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_result() {
        let response = "
    [[0,1452348610.39281,0.000101566314697266],
    [[[9],[[\"_id\",\"UInt32\"],[\"_key\",\"ShortText\"],[\"title\",\"ShortText\"]],
    [1,\"http://example.org/\",\"This is test record 1!\"],
    [2,\"http://example.net/\",\"test record 2.\"],
    [3,\"http://example.com/\",\"test test record three.\"],
    [4,\"http://example.net/afr\",\"test record four.\"],
    [5,\"http://example.org/aba\",\"test test test record five.\"],
    [6,\"http://example.com/rab\",\"test test test test record six.\"],
    [7,\"http://example.net/atv\",\"test test test record seven.\"],
    [8,\"http://example.org/gat\",\"test test record eight.\"],
    [9,\"http://example.com/vdw\",\"test test record nine.\"]]]]";
        let mut decode = ResultParser::new(response.to_string());
        assert_eq!(&0, decode.status().unwrap());
        assert_eq!(&1452348610.39281, decode.start_time().unwrap());
        assert_eq!(&0.000101566314697266, decode.elapsed_time().unwrap());
        assert_eq!(9, decode.matched_columns().unwrap());
        let vec = decode.result().unwrap().pop().unwrap().unwrap_vec().clone();
        let expected = r#"Array([Integer(1), String("http://example.org/"), String("This is test record 1!")])"#.to_owned();
        assert_eq!(expected, format!("{:?}", vec[2]))
    }
}
