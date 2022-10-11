use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub checkbox_open: String,
    pub checkbox_mark: String,
    pub checkbox_shut: String,
    pub memo:  String,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{} {}", self.checkbox_open, self.checkbox_mark, self.checkbox_shut, self.memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        let item: Item = Item {
            checkbox_open: "[".into(),
            checkbox_mark: "x".into(),
            checkbox_shut: "]".into(),
            memo: "foo".into(),
        };
        let actual: String = item.to_string();
        let expect: String = "[x] foo".to_string();
        assert_eq!(actual, expect)
    }

    #[test]
    fn test_serde_json_from_str() {
        let input_json_as_str = r#"
            {
                "checkbox_open": "[",
                "checkbox_mark": "x",
                "checkbox_shut": "]",
                "memo": "foo"
            }
        "#;
        let actual: Item = serde_json::from_str(input_json_as_str).expect("actual");
        let expect = Item {
            checkbox_open: "[".into(),
            checkbox_mark: "x".into(),
            checkbox_shut: "]".into(),
            memo: "foo".into(),
        };
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_serde_json_to_string() {
        let item: Item = Item {
            checkbox_open: "[".into(),
            checkbox_mark: "x".into(),
            checkbox_shut: "]".into(),
            memo: "foo".into(),
        };
        let actual: String = serde_json::to_string(&item).expect("actual");
        let expect: String = r#"{"checkbox_open":"[","checkbox_mark":"x","checkbox_shut":"]","memo":"foo"}"#.to_string();
        assert_eq!(actual, expect);
    }

}
