use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::item::item::Item;

static REGEX_LINE_ITEM_OPEN: Lazy<Regex> = Lazy::new(||Regex::new(r"^\s*[\*\+\-•]?\s*\[.\]").unwrap());
static REGEX_LINE_ITEM_OPEN_CAPTURES: Lazy<Regex> = Lazy::new(||Regex::new(r"^(\s*)[\*\+\-•]?\s*\[(.)\]\s*(.*)$").unwrap());
static REGEX_LINE_BLANK: Lazy<Regex> = Lazy::new(||Regex::new(r"^\s*$").unwrap());

#[derive(Debug, PartialEq)]
enum State {
    Do,
    Doing,
}

#[derive(Debug, PartialEq)]
enum LineKind {
    Blank,
    ItemOpen,
    Other,
}

/// Calculate the nest level.
/// 
/// Nesting can be done by any combination of:
/// 
///    * 1 tab
///    * 4 spaces
/// 
pub fn whitespace_to_nest(s: &str) -> u8 {
    s.matches("\t").count() as u8 +
    (s.matches(" ").count() as u8 / 4)
}

/// Load items via path.
/// 
/// ```
/// let path = Path::new("example.txt");
/// let items = load_items_via_path(file).unwrap();
/// ```
/// 
pub fn load_items_via_path(path: &Path) -> ::std::io::Result<Vec<Item>> {
    load_items_via_file(::std::fs::File::open(path)?)
}

/// Load items via file.
/// 
/// ```
/// let file = File::open("example.txt").unwrap();
/// let items = load_items_via_file(file).unwrap();
/// ```
/// 
pub fn load_items_via_file(file: File) -> ::std::io::Result<Vec<Item>> {
    load_items_via_buf_read(::std::io::BufReader::new(file))
}

/// Load items via str.
/// 
/// ```
/// let str = "[ ] foo\n[!] goo\n[x] hoo\n";
/// let items = load_items_via_str(str).unwrap();
/// ```
/// 
pub fn load_items_via_str(s: &str) -> ::std::io::Result<Vec<Item>> {
    load_items_via_string_reader(::stringreader::StringReader::new(s))
}

/// Load items via string reader.
/// 
/// ```
/// let string_reader = StringReader::new("[ ] foo\n[!] goo\n[x] hoo\n");
/// let items = load_items_via_string_reader(str).unwrap();
/// ```
/// 
pub fn load_items_via_string_reader(string_reader: ::stringreader::StringReader) -> std::io::Result<Vec<Item>> {
    load_items_via_buf_read(::std::io::BufReader::new(string_reader))
}

/// Load items via buf read.
/// 
/// ```
/// let buf_read = BufReader::new(File::open("example.txt").unwrap())
/// let items = load_items_via_buf_read(str).unwrap();
/// ```
/// 
pub fn load_items_via_buf_read(buf_read: impl std::io::BufRead) -> std::io::Result<Vec<Item>> {
    let mut vec: Vec<Item> = Vec::new();
    let mut state = State::Do;
    let mut line_kind: LineKind;
    let mut nest = 0 as u8;
    let mut mark = String::from("?");
    let mut memo = String::from("?");
    let lines = buf_read.lines();
    for line in lines {
        let s = line?;
        println!("line: {}", s);
 
        // What line kind are we handling?
        line_kind = if REGEX_LINE_ITEM_OPEN.is_match(&s) {
            LineKind::ItemOpen
        } else if REGEX_LINE_BLANK.is_match(&s) {
            LineKind::Blank
        } else {
            LineKind::Other  
        };

        // If there's an item in progress, then can we finish it?
        if state == State::Doing && (line_kind == LineKind::ItemOpen || line_kind == LineKind::Blank) {
            vec.push(Item {
                nest: Some(nest.clone()),
                mark: Some(mark.clone()),
                memo: Some(memo.clone()),
                label1s: None,
                label2s: None,
            })
        }
        match line_kind {
            LineKind::ItemOpen => {
                state = State::Doing;
                if let Some(captures) = REGEX_LINE_ITEM_OPEN_CAPTURES.captures(&s) {
                    nest = captures.get(1).map_or(0, |m| whitespace_to_nest(m.as_str()));
                    mark = String::from(captures.get(2).map_or("?", |m| m.as_str()));
                    memo = String::from(captures.get(3).map_or("?", |m| m.as_str().trim()));
                }
            },
            LineKind::Blank => {
                state = State::Do;
            },
            LineKind::Other => {
                state = State::Doing;
                memo.push_str("\n");
                memo.push_str(&s.trim());
            }
        }
    }
    if state == State::Doing {
        vec.push(Item {
            nest: Some(nest.clone()),
            mark: Some(mark.clone()),
            memo: Some(memo.clone()),
            label1s: None,
            label2s: None,
        })
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_whitespace_to_nest() {
        assert_eq!(whitespace_to_nest(""), 0);
        assert_eq!(whitespace_to_nest(" "), 0);
        assert_eq!(whitespace_to_nest("  "), 0);
        assert_eq!(whitespace_to_nest("   "), 0);
        assert_eq!(whitespace_to_nest("    "), 1);
        assert_eq!(whitespace_to_nest("     "), 1);
        assert_eq!(whitespace_to_nest("      "), 1);
        assert_eq!(whitespace_to_nest("       "), 1);
        assert_eq!(whitespace_to_nest("        "), 2);
        assert_eq!(whitespace_to_nest("\t"), 1);
        assert_eq!(whitespace_to_nest("\t\t"), 2);
    }

    #[test]
    fn test_load_items_via_str() {
        let str = indoc!{"
            [ ] foo
            [!] goo
            [x] hoo
        "};
        let actual = load_items_via_str(str).unwrap();
        let expect = vec![
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
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_load_items_via_string_reader() {
        let str = indoc!{"
            [ ] foo
            [!] goo
            [x] hoo
        "};
        let string_reader = ::stringreader::StringReader::new(&str);
        let actual = load_items_via_string_reader(string_reader).unwrap();
        let expect = vec![
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
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_1_content() {
        let expect = vec![
            Item {
                nest: Some(0),
                mark: Some(" ".into()),
                memo: Some("alpha1".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(0),
                mark: Some("!".into()),
                memo: Some("bravo1".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(0),
                mark: Some("x".into()),
                memo: Some("charlie1".into()),
                label1s: None,
                label2s: None,
            },
        ];
        assert_eq!(load_items_via_path(Path::new("test/load/1-content-and-0-between.txt")).unwrap(), expect);
        assert_eq!(load_items_via_path(Path::new("test/load/1-content-and-1-between.txt")).unwrap(), expect);
        assert_eq!(load_items_via_path(Path::new("test/load/1-content-and-2-between.txt")).unwrap(), expect);
    }

    #[test]
    fn test_2_content() {
        let expect = vec![
            Item {
                nest: Some(0),
                mark: Some(" ".into()),
                memo: Some("alpha1\nalpha2".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(0),
                mark: Some("!".into()),
                memo: Some("bravo1\nbravo2".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(0),
                mark: Some("x".into()),
                memo: Some("charlie1\ncharlie2".into()),
                label1s: None,
                label2s: None,
            },
        ];
        assert_eq!(load_items_via_path(Path::new("test/load/2-content-and-0-between.txt")).unwrap(), expect);
        assert_eq!(load_items_via_path(Path::new("test/load/2-content-and-1-between.txt")).unwrap(), expect);
        assert_eq!(load_items_via_path(Path::new("test/load/2-content-and-2-between.txt")).unwrap(), expect);
    }

    #[test]
    fn test_list_markers() {
        let str = indoc!{"
            + [ ] plus
            - [ ] minus
            * [ ] asterisk
        "};
        let actual = load_items_via_str(str).unwrap();
        let expect = vec![
            Item {
                nest: Some(0),
                mark: Some(" ".into()),
                memo: Some("plus".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(0),
                mark: Some(" ".into()),
                memo: Some("minus".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(0),
                mark: Some(" ".into()),
                memo: Some("asterisk".into()),
                label1s: None,
                label2s: None,
            },
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_indent_with_spaces() {
        let str = indoc!{"
            [ ] 0-space
             [ ] 1-space
              [ ] 2-space
               [ ] 3-space
                [ ] 4-space
                 [ ] 5-space
                  [ ] 6-space
                   [ ] 7-space
                    [ ] 8-space
        "};
        let actual = load_items_via_str(&str).unwrap();
        let expect = vec![
            Item {
                nest: Some(0),
                mark: Some(" ".into()),
                memo: Some("0-space".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(0),
                mark: Some(" ".into()),
                memo: Some("1-space".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(0),
                mark: Some(" ".into()),
                memo: Some("2-space".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(0),
                mark: Some(" ".into()),
                memo: Some("3-space".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(1),
                mark: Some(" ".into()),
                memo: Some("4-space".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(1),
                mark: Some(" ".into()),
                memo: Some("5-space".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(1),
                mark: Some(" ".into()),
                memo: Some("6-space".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(1),
                mark: Some(" ".into()),
                memo: Some("7-space".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(2),
                mark: Some(" ".into()),
                memo: Some("8-space".into()),
                label1s: None,
                label2s: None,
            },
        ];
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_indent_with_tabs() {
        let str = indoc!{"
            [ ] 0-tab
            \t[ ] 1-tab
            \t\t[ ] 2-tab
        "};
        let actual = load_items_via_str(&str).unwrap();
        let expect = vec![
            Item {
                nest: Some(0),
                mark: Some(" ".into()),
                memo: Some("0-tab".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(1),
                mark: Some(" ".into()),
                memo: Some("1-tab".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(2),
                mark: Some(" ".into()),
                memo: Some("2-tab".into()),
                label1s: None,
                label2s: None,
            },
        ];
        assert_eq!(actual, expect);
    }

}
