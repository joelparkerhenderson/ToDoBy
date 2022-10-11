use crate::item::item::*;

/// Parse a checkbox.
///
/// Example:
/// 
/// ```
/// let input = "[x]";
/// let (input, checkbox_open_str, checkbox_mark_str, checkbox_shut_str) = checkbox(input).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(checkbox_open_str, '[');
/// assert_eq!(checkbox_mark_str, "x");
/// assert_eq!(checkbox_shut_str, ']');
/// ```
///
pub fn checkbox(input: &str) -> nom::IResult<&str, (&str, &str, &str)> {
    nom::sequence::tuple((checkbox_open, checkbox_mark, checkbox_shut))(input)
}

/// Parse a checkbox open.
///
/// Example:
/// 
/// ```
/// let input = "[";
/// let (input, checkbox_open_str) = checkbox_open(input).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(checkbox_open_str, '[');
/// ```
///
pub fn checkbox_open(input: &str) -> nom::IResult<&str, &str> {
    nom::branch::alt((
        nom::bytes::complete::tag("["), // U+005B LEFT SQUARE BRACKET
        nom::bytes::complete::tag("［"), // U+FF3B FULLWIDTH LEFT SQUARE BRACKET
    ))(input)
}

/// Parse a checkbox mark.
///
/// Example:
/// 
/// ```
/// let input = "x";
/// let (input, checkbox_mark_char) = checkbox_mark(input).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(checkbox_mark_char, 'x');
/// ```
///
/// TODO: Unicode
/// 
pub fn checkbox_mark(input: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::take(1 as u8)(input)
}

/// Parse a checkbox shut.
///
/// Example:
/// 
/// ```
/// let input = "]";
/// let (input, checkbox_shut_str) = checkbox_shut(input).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(checkbox_shut_str, "]");
/// ```
///
pub fn checkbox_shut(input: &str) -> nom::IResult<&str, &str> {
    nom::branch::alt((
        nom::bytes::complete::tag("]"), // U+005D RIGHT SQUARE BRACKET
        nom::bytes::complete::tag("］"), // U+FF3D FULLWIDTH RIGHT SQUARE BRACKET
    ))(input)
}

/// Parse a label.
///
/// Example:
/// 
/// ```
/// let input = "#foo";
/// let (input, label_open, label_phrase) = label(input).unwrap();
/// assert_eq!(input, "")
/// assert_eq!(label_open_str, "#");
/// assert_eq!(label_phrase_str, "foo");
/// ```
///
pub fn label(input: &str) -> nom::IResult<&str, (&str, &str)> {
    nom::sequence::tuple((label_open, label_phrase))(input)
}

/// Parse a label open.
///
/// Example:
/// 
/// ```
/// let input = "#"
/// let (input, label_open_str) = label_open(input);
/// assert_eq!(input, "");
/// assert_eq!(label_open_str, "x");
/// ```
///
pub fn label_open(input: &str) -> nom::IResult<&str, &str> {
    nom::branch::alt((
        nom::bytes::complete::tag("#"), // U+0023 NUMBER SIGN
        nom::bytes::complete::tag("＃"), // U+FF03 FULLWIDTH NUMBER SIGN
    ))(input)
}

/// Parse a label phrase.
///
/// Example:
/// 
/// ```
/// let input = "foo"
/// let (input, label_phrase_str) = label_phrase(input);
/// assert_eq!(input, "");
/// assert_eq!(label_phrase_str, "foo");
/// ```
///
pub fn label_phrase(input: &str) -> nom::IResult<&str, &str> {
    nom_unicode::complete::alphanumeric0(input)
}

/// Parse a label splitter.
///
/// Example:
/// 
/// ```
/// let input = ":"
/// let (input, label_splitter_str) = label_splitter(input);
/// assert_eq!(input, "");
/// assert_eq!(label_splitter_str, ":");
/// ```
///
pub fn label_splitter(input: &str) -> nom::IResult<&str, &str> {
    nom::branch::alt((
        nom::bytes::complete::tag(":"), // U+003A COLON
        nom::bytes::complete::tag("："), // U+FF1A FULLWIDTH COLON
    ))(input)
}

/// Parse a memo.
///
/// A memo is any text until a newline or end.
/// 
/// Example:
/// 
/// ```
/// let input = "foo"
/// let (_input, memo) = memo(input);
/// assert_eq!(memo, "foo")
/// ```
///
pub fn memo(input: &str) -> nom::IResult<&str, &str> {
    nom::character::complete::not_line_ending(input)
}

/// Parse one item.
///
/// Example:
/// 
/// ```
/// let input = "[x] foo"
/// let (_input, item) = crate::item::item::item_parser::one(input);
/// assert_eq!(item, Item { 
///     checkbox_open: "[".into(), 
///     checkbox_mark: "x".into(), 
///     checkbox_shut: "]".into(), 
///     memo: "foo".into(),
/// })
/// ```
///
pub fn one(input: &str) -> nom::IResult<&str, Item> {
    let (input, _) = nom::character::complete::multispace0(input)?;
    let (input, (checkbox_open_str, checkbox_mark_str, checkbox_shut_str)) = self::checkbox(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, memo) = self::memo(input)?;
    let (input, _) = nom::character::complete::multispace0(input)?;
    let item = Item { 
        checkbox_open: checkbox_open_str.into(),
        checkbox_mark: checkbox_mark_str.into(),
        checkbox_shut: checkbox_shut_str.into(),
        memo: memo.into(),
    };
    Ok((input, item))
}

/// Parse many items.
///
/// Example:
/// 
/// ```
/// let input = "[x] foo"
/// let (_input, item) = crate::item::item::item_parser::one(input);
/// assert_eq!(item, Item { mark: "x".into(), memo: "foo".into() } )
/// ```
///
pub fn many0(input: &str) -> nom::IResult<&str, Vec<Item>> {
    let (input, items) = nom::multi::many0(self::one)(input)?;
    Ok((input, items))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_checkbox() {
        let input = "[x]";
        let (input, (checkbox_open_str, checkbox_mark_str, checkbox_shut_str)) = super::checkbox(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(checkbox_open_str, "[");
        assert_eq!(checkbox_mark_str, "x");
        assert_eq!(checkbox_shut_str, "]");
    }

    #[test]
    fn test_checkbox_with_high_unicode() {
        let input = "［✓］"; // U+FF3B FULLWIDTH LEFT SQUARE BRACKET, U+2713 CHECK MARK, U+FF3D, FULLWIDTH RIGHT SQUARE BRACKET
        let (input, (checkbox_open_str, checkbox_mark_str, checkbox_shut_str)) = super::checkbox(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(checkbox_open_str, "［");
        assert_eq!(checkbox_mark_str, "✓");
        assert_eq!(checkbox_shut_str, "］");
    }

    #[test]
    fn test_checkbox_open() {
        let input = "[";
        let (input, checkbox_open_str) = super::checkbox_open(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(checkbox_open_str, "[");
    }

    #[test]
    fn test_checkbox_open_with_high_unicode() {
        let input = "［"; // U+FF3B FULLWIDTH LEFT SQUARE BRACKET
        let (input, checkbox_open_str) = super::checkbox_open(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(checkbox_open_str, "［");
    }

    #[test]
    fn test_checkbox_mark() {
        let input = "x";
        let (input, checkbox_mark_str) = super::checkbox_mark(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(checkbox_mark_str, "x");
    }

    #[test]
    fn test_checkbox_mark_with_high_unicode() {
        let input = "✓"; // U+2713 CHECK MARK
        let (input, checkbox_mark_str) = super::checkbox_mark(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(checkbox_mark_str, "✓");
    }

    #[test]
    fn test_checkbox_shut() {
        let input = "]";
        let (input, checkbox_shut_str) = super::checkbox_shut(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(checkbox_shut_str, "]");
    }

    #[test]
    fn test_checkbox_shut_with_high_unicode() {
        let input = "］"; // U+FF3D FULLWIDTH RIGHT SQUARE BRACKET
        let (input, checkbox_shut_str) = super::checkbox_shut(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(checkbox_shut_str, "］");
    }

    #[test]
    fn test_label() {
        let input = "#foo";
        let (input, (label_open_str, label_phrase_str)) = super::label(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(label_open_str, "#");
        assert_eq!(label_phrase_str, "foo");
    }

    #[test]
    fn test_label_with_high_unicode() {
        let input = "＃αβ"; // U+FF03 FULLWIDTH NUMBER SIGN, U+03B1 GREEK SMALL LETTER ALPHA, U+03B2 GREEK SMALL LETTER BETA
        let (input, (label_open_str, label_phrase_str)) = super::label(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(label_open_str, "＃");
        assert_eq!(label_phrase_str, "αβ");
    }

    #[test]
    fn test_label_open() {
        let input = "#";
        let (input, label_open_str) = super::label_open(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(label_open_str, "#");
    }

    #[test]
    fn test_label_open_with_high_unicode() {
        let input = "＃"; // U+FF03 FULLWIDTH NUMBER SIGN, 
        let (input, label_open_str) = super::label_open(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(label_open_str, "＃");
    }

    #[test]
    fn test_label_phrase() {
        let input = "foo";
        let (input, label_phrase_str) = super::label_phrase(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(label_phrase_str, "foo");
    }

    #[test]
    fn test_label_phrase_with_high_unicode() {
        let input = "αβ"; // U+03B1 GREEK SMALL LETTER ALPHA, U+03B2 GREEK SMALL LETTER BETA
        let (input, label_phrase_str) = super::label_phrase(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(label_phrase_str, "αβ");
    }

    #[test]
    fn test_label_splitter() {
        let input = ":";
        let (input, label_splitter_str) = super::label_splitter(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(label_splitter_str, ":");
    }

    #[test]
    fn test_label_splitter_with_high_unicode() {
        let input = "："; // U+FF1A FULLWIDTH COLON
        let (input, label_splitter_str) = super::label_splitter(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(label_splitter_str, "：");
    }

    #[test]
    fn test_memo() {
        let input = "foo";
        let actual = crate::item::item_parser::memo(input);
        let expect = Ok(("", "foo"));
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_one() {
        let input = indoc!{"
            [x] foo
        "};
        let expect_item =  Item {
            checkbox_open: "[".into(),
            checkbox_mark: "x".into(),
            checkbox_shut: "]".into(),
            memo: "foo".into(),
        };
        let actual = crate::item::item_parser::one(input);
        let expect = Ok(("", expect_item));
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_many0() {
        let input = indoc!{"
            [ ] Do

            [!] Doing

            [x] Done
        "};
        let expect_items = vec![
            Item {
                checkbox_open: "[".into(),
                checkbox_mark: " ".into(),
                checkbox_shut: "]".into(),
                memo: "Do".into(),
            },
            Item {
                checkbox_open: "[".into(),
                checkbox_mark: "!".into(),
                checkbox_shut: "]".into(),
                memo: "Doing".into(),
            },
            Item {
                checkbox_open: "[".into(),
                checkbox_mark: "x".into(),
                checkbox_shut: "]".into(),
                memo: "Done".into(),
            },
        ];
        let actual = crate::item::item_parser::many0(input);
        let expect = Ok(("", expect_items));
        assert_eq!(actual, expect);
    }
    
}