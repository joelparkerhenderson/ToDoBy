use std::fs::File;
use std::path::Path;
use once_cell::sync::Lazy;
use regex::Regex;
use regex::bytes::Captures;
use crate::item::item::Item;

static REGEX_LINE_ITEM_OPEN: Lazy<Regex> = Lazy::new(||Regex::new(r"^\s*[\*\+\-#•]?\s*\[.\]").unwrap());
static REGEX_LINE_ITEM_OPEN_CAPTURES: Lazy<Regex> = Lazy::new(||Regex::new(r"^(\s*)[\*\+\-#•]?\s*\[(.)\]\s*(.*)$").unwrap());
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

pub fn load_items_via_path(path: &Path) -> std::io::Result<Vec<Item>> {
    load_items_via_file(std::fs::File::open(path)?)
}

pub fn load_items_via_file(file: File) -> std::io::Result<Vec<Item>> {
    load_items_via_reader(std::io::BufReader::new(file))
}

pub fn load_items_via_reader(reader: impl std::io::BufRead) -> std::io::Result<Vec<Item>> {
    let mut vec: Vec<Item> = Vec::new();
    let mut state = State::Do;
    let mut line_kind: LineKind;
    let mut nest = 0 as u8;
    let mut mark = String::from("?");
    let mut memo = String::from("?");
    let lines = reader.lines();
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
        println!("line_kind: {:?}", line_kind);

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
                if let Some(captures) = REGEX_LINE_ITEM_OPEN_CAPTURES.captures(&s) {
                    //nest = captures.get(1).map_or(0, |m| m.as_str().parse::<u8>().expect("nest"));
                    mark = String::from(captures.get(2).map_or("?", |m| m.as_str()));
                    memo = String::from(captures.get(3).map_or("?", |m| m.as_str().trim()));
                }
                state = State::Doing;
            },
            LineKind::Blank => {
                state = State::Do;
            },
            LineKind::Other => {
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
    fn test_list_item_symbols() {
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
            Item {
                nest: Some(0),
                mark: Some(" ".into()),
                memo: Some("number sign".into()),
                label1s: None,
                label2s: None,
            },
            Item {
                nest: Some(0),
                mark: Some(" ".into()),
                memo: Some("bullet".into()),
                label1s: None,
                label2s: None,
            },
        ];
        assert_eq!(load_items_via_path(Path::new("test/load/list-item-symbols.txt")).unwrap(), expect);
    }

}
