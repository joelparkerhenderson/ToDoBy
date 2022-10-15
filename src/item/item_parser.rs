use crate::item::item::*;

/// Parse an indent of spaces.
///
/// Example:
/// 
/// ```
/// let input = "  foo";
/// let (input, indent_str) = indent(input).unwrap();
/// assert_eq!(input, "foo");
/// assert_eq!(indent_str, "  ");
/// ```
///
pub fn indent(input: &str) -> nom::IResult<&str, &str> {
    nom::character::complete::multispace0(input)
}

/// Parse a list marker.
///
/// Example:
/// 
/// ```
/// let input = "*";
/// let (input, list_marker_str) = checkbox_open(input).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(list_marker_str, "*");
/// ```
///
pub fn list_marker(input: &str) -> nom::IResult<&str, &str> {
    nom::branch::alt((
        nom::bytes::complete::tag("*"),
        nom::bytes::complete::tag("+"),
        nom::bytes::complete::tag("-"),
        nom::bytes::complete::tag("•"),
    ))(input)
}

/// Parse a checkbox.
///
/// Example:
/// 
/// ```
/// let input = "[x]";
/// let (input, checkbox_open_str, checkbox_mark_str, checkbox_shut_str) = checkbox(input).unwrap();
/// assert_eq!(input, "");
/// assert_eq!(checkbox_open_str, "[");
/// assert_eq!(checkbox_mark_str, "x");
/// assert_eq!(checkbox_shut_str, "]");
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

/// Parse a label that has 1 phrase.
///
/// Example:
/// 
/// ```
/// let input = "#foo";
/// let (input, (
///     label_open_str, 
///     label_phrase_str
/// )) = label1(input).unwrap();
/// assert_eq!(input, "")
/// assert_eq!(label_open_str, "#");
/// assert_eq!(label_phrase_str, "foo");
/// ```
///
pub fn label1(input: &str) -> nom::IResult<&str, (&str, &str)> {
    nom::sequence::tuple((label_open, label_phrase))(input)
}

/// Parse a label that has 2 phrases.
///
/// Example:
/// 
/// ```
/// let input = "#foo:goo";
/// let (input, (
///     label_open_str,
///     label_phrase_0_str, 
///     label_splitter_0_str,
///     label_phrase_1_str
/// )) = label2(input).unwrap();
/// assert_eq!(input, "")
/// assert_eq!(label_open_str, "#");
/// assert_eq!(label_phrase_0_str, "foo");
/// assert_eq!(label_spliter_0_str, ":");
/// assert_eq!(label_phrase_1_str, "goo");
/// ```
///
pub fn label2(input: &str) -> nom::IResult<&str, (&str, &str, &str, &str)> {
    nom::sequence::tuple((label_open, label_phrase, label_splitter, label_phrase))(input)
}

/// Parse a label with 3 phrases.
///
/// Example:
/// 
/// ```
/// let input = "#foo:goo:hoo";
/// let (input, (
///     label_open_str,
///     label_phrase_0_str, 
///     label_splitter_0_str, 
///     label_phrase_1_str, 
///     label_splitter_1_str, 
///     label_phrase_2_str
/// )) = label3(input).unwrap();
/// assert_eq!(input, "")
/// assert_eq!(label_open_str, "#");
/// assert_eq!(label_phrase_0_str, "foo");
/// assert_eq!(label_spliter_0_str, ":");
/// assert_eq!(label_phrase_1_str, "goo");
/// assert_eq!(label_spliter_1_str, ":");
/// assert_eq!(label_phrase_2_str, "hoo");
/// ```
///
pub fn label3(input: &str) -> nom::IResult<&str, (&str, &str, &str, &str, &str, &str)> {
    nom::sequence::tuple((label_open, label_phrase, label_splitter, label_phrase, label_splitter, label_phrase))(input)
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
///     nest: Some(0),
///     mark: Some("x".into()), 
///     memo: Some("foo".into()),
///     label1s: None,
///     label2s: None,
/// })
/// ```
///
pub fn one(input: &str) -> nom::IResult<&str, Item> {
    let (input, _indent_str) = self::indent(input)?;
    let (input, _list_marker_str) = self::list_marker(input)?;
    let (input, _) = self::indent(input)?;
    let (input, (_checkbox_open_str, checkbox_mark_str, _checkbox_shut_str)) = self::checkbox(input)?;
    let (input, _) = self::indent(input)?;
    let (input, memo) = self::memo(input)?;
    let (input, _) = nom::character::complete::multispace0(input)?;
    let item = Item {
        nest: Some(0 as u8), //Some((indent_str.len() / 2) as i8),
        mark: Some(checkbox_mark_str.into()),
        memo: Some(memo.into()),
        label1s: None,
        label2s: None,
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
    fn test_indent_with_0_spaces() {
        let input = "x";
        let (input, indent_str) = super::indent(input).unwrap();
        assert_eq!(input, "x");
        assert_eq!(indent_str, "");
    }

    #[test]
    fn test_indent_with_2_spaces() {
        let input = "  x";
        let (input, indent_str) = super::indent(input).unwrap();
        assert_eq!(input, "x");
        assert_eq!(indent_str, "  ");
    }

    #[test]
    fn test_list_marker() {
        let input = "*";
        let (input, list_marker_str) = super::list_marker(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(list_marker_str, "*");
    }

    #[test]
    fn test_list_marker_with_high_unicode() {
        let input = "•"; // U+2022 BULLET
        let (input, list_marker_str) = super::list_marker(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(list_marker_str, "•");
    }

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
    fn test_label1() {
        let input = "#foo";
        let (input, (
            label_open_str, 
            label_phrase_str,
        )) = super::label1(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(label_open_str, "#");
        assert_eq!(label_phrase_str, "foo");
    }

    #[test]
    fn test_label1_with_high_unicode() {
        let input = "＃αβ"; // U+FF03 FULLWIDTH NUMBER SIGN, U+03B1 GREEK SMALL LETTER ALPHA, U+03B2 GREEK SMALL LETTER BETA
        let (input, (
            label_open_str, 
            label_phrase_str,
        )) = super::label1(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(label_open_str, "＃");
        assert_eq!(label_phrase_str, "αβ");
    }

    #[test]
    fn test_label2() {
        let input = "#foo:goo";
        let (input, (
            label_open_str, 
            label_phrase_0_str, 
            label_splitter_0_str,
            label_phrase_1_str, 
        )) = super::label2(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(label_open_str, "#");
        assert_eq!(label_phrase_0_str, "foo");
        assert_eq!(label_splitter_0_str, ":");
        assert_eq!(label_phrase_1_str, "goo");
    }

    #[test]
    fn test_label3() {
        let input = "#foo:goo:hoo";
        let (input, (
            label_open_str, 
            label_phrase_0_str, 
            label_splitter_0_str,
            label_phrase_1_str, 
            label_splitter_1_str,
            label_phrase_2_str, 
        )) = super::label3(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(label_open_str, "#");
        assert_eq!(label_phrase_0_str, "foo");
        assert_eq!(label_splitter_0_str, ":");
        assert_eq!(label_phrase_1_str, "goo");
        assert_eq!(label_splitter_1_str, ":");
        assert_eq!(label_phrase_2_str, "hoo");
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
            nest: Some(0),
            mark: Some("x".into()),
            memo: Some("foo".into()),
            label1s: None,
            label2s: None,
        };
        let actual = crate::item::item_parser::one(input);
        let expect = Ok(("", expect_item));
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_one_with_list_marker() {
        let input = indoc!{"
            * [x] foo
        "};
        let expect_item =  Item {
            nest: Some(0),
            mark: Some("x".into()),
            memo: Some("foo".into()),
            label1s: None,
            label2s: None,
        };
        let actual = crate::item::item_parser::one(input);
        let expect = Ok(("", expect_item));
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_many0() {
        let input = indoc!{"
            [ ] foo

            [!] goo

            [x] hoo
        "};
        let expect_items = vec![
            Item {
                nest: Some(0),
                mark: Some(" ".into()),
                memo: Some("foo".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(0),
                mark: Some("!".into()),
                memo: Some("goo".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(0),
                mark: Some("x".into()),
                memo: Some("hoo".into()),
                label1s: None,
                label2s: None,
            },
        ];
        let actual = crate::item::item_parser::many0(input);
        let expect = Ok(("", expect_items));
        assert_eq!(actual, expect);
    }

    // #[test]
    // fn test_many0_with_readme_example() {
    //     let input = indoc!{"
    //         - [ ] Call friends to
    //               ask who's coming
    //               #personal
    //               #priority:1

    //           - [ ] Call Alice

    //           - [ ] Call Bob

    //         - [x] Send invitations
    //               to save the date

    //           - [x] Email Carol

    //           - [x] Text Dave

    //         - [@] Arrange flowers; delegate to the 
    //               florist who will deliver on the day
    //               #phone:1-800-FLOWERS
    //               #order:12345678

    //         - [.] Prepare food; defer the shopping and
    //               cooking to the week before the party

    //         - [/] Reserve venue; drop because we
    //               decided to do the party at home

    //     "}; 
    //     let expect_items = vec![
    //         Item {
    //             nest: Some(0),
    //             mark: Some(" ".into()),
    //             memo: Some("Call friends to\nask who's coming\n#personal\n#priority:1".into()),
    //             label1s: None,
    //             label2s: None,
    //         },
    //         Item {
    //             nest: Some(1),
    //             mark: Some(" ".into()),
    //             memo: Some("Call Alice".into()),
    //             label1s: None,
    //             label2s: None,
    //         },
    //         Item {
    //             nest: Some(1),
    //             mark: Some(" ".into()),
    //             memo: Some("Call Bob".into()),
    //             label1s: None,
    //             label2s: None,
    //         },
    //         Item {
    //             nest: Some(0),
    //             mark: Some("x".into()),
    //             memo: Some("Send invitations\nto save the date".into()),
    //             label1s: None,
    //             label2s: None,
    //         },
    //         Item {
    //             nest: Some(1),
    //             mark: Some("x".into()),
    //             memo: Some("Email Carol".into()),
    //             label1s: None,
    //             label2s: None,
    //         },
    //         Item {
    //             nest: Some(1),
    //             mark: Some("x".into()),
    //             memo: Some("Text Dave".into()),
    //             label1s: None,
    //             label2s: None,
    //         },
    //         Item {
    //             nest: Some(0),
    //             mark: Some("@".into()),
    //             memo: Some("Arrange flowers; delegate to the\nflorist who will deliver on the day\n#phone:1-800-FLOWERS\n#order:12345678".into()),
    //             label1s: None,
    //             label2s: None,
    //         },
    //         Item {
    //             nest: Some(0),
    //             mark: Some(".".into()),
    //             memo: Some("Prepare food; defer the shopping and\ncooking to the week before the party".into()),
    //             label1s: None,
    //             label2s: None,
    //         },
    //         Item {
    //             nest: Some(0),
    //             mark: Some("/".into()),
    //             memo: Some("Reserve venue; drop because we\ndecided to do the party at home".into()),
    //             label1s: None,
    //             label2s: None,
    //         },
    //     ];
    //     let actual = crate::item::item_parser::many0(input);
    //     let expect = Ok(("", expect_items));
    //     assert_eq!(actual, expect);
    // }

}