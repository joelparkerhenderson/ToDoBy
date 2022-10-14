use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub nest: Option<u8>,
    pub mark: Option<String>,
    pub memo: Option<String>,
    pub label1s: Option<Vec<String>>,
    pub label2s: Option<Vec<(String, String)>>,
}

static NEST_DEFAULT: i8 = 0;
static LIST_ITEM_SYMBOL_DEFAULT: &str = "";
static LIST_ITEM_SYMBOL_SPACER_DEFAULT: &str = "";
static CHECKBOX_PREFIX_DEFAULT: &str = "";
static CHECKBOX_OPEN_DEFAULT: &str = "[";
static CHECKBOX_MARK_DEFAULT: &str = " ";
static CHECKBOX_SHUT_DEFAULT: &str = "]";
static CHECKBOX_SUFFIX_DEFAULT: &str = " ";
static MEMO_DEFAULT: &str = "?";

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}{}{}{}{}{}",
            LIST_ITEM_SYMBOL_DEFAULT,
            LIST_ITEM_SYMBOL_SPACER_DEFAULT,
            CHECKBOX_PREFIX_DEFAULT,
            CHECKBOX_OPEN_DEFAULT,
            match &self.mark { Some(x) => x, None => CHECKBOX_MARK_DEFAULT },
            CHECKBOX_SHUT_DEFAULT,
            CHECKBOX_SUFFIX_DEFAULT,
            match &self.memo { Some(x) => x, None => MEMO_DEFAULT },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_with_default_attributes() {
        let item: Item = Item {
            nest: None,
            mark: None,
            memo: None,
            label1s: None,
            label2s: None,
        };
        let actual: String = item.to_string();
        let expect: String = "[ ] ?".to_string();
        assert_eq!(actual, expect)
    }

    #[test]
    fn test_fmt_with_custom_attributes() {
        let item: Item = Item {
            nest: Some(0),
            mark: Some("x".into()),
            memo: Some("foo".into()),
            label1s: None,
            label2s: None,
        };
        let actual: String = item.to_string();
        let expect: String = "[x] foo".to_string();
        assert_eq!(actual, expect)
    }


    #[test]
    fn test_serde_json_from_str() {
        let input_json_as_str = r#"
            {
                "nest": 0,
                "mark": "x",
                "memo": "foo"
            }
        "#;
        let actual: Item = serde_json::from_str(input_json_as_str).expect("actual");
        let expect = Item {
            nest: Some(0),
            mark: Some("x".into()),
            memo: Some("foo".into()),
            label1s: None,
            label2s: None,
        };
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_serde_json_to_string() {
        let item: Item = Item {
            nest: Some(0),
            mark: Some("x".into()),
            memo: Some("foo".into()),
            label1s: None,
            label2s: None,
        };
        let actual: String = serde_json::to_string(&item).expect("actual");
        let expect: String = r#"{"nest":0,"mark":"x","memo":"foo","label1s":null,"label2s":null}"#.to_string();
        assert_eq!(actual, expect);
    }

}
