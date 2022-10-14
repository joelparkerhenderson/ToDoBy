//use std::borrow::Cow;
//use indoc::indoc;
use std::path::Path;

mod item;
mod load;
mod ui;

use load::*;
//use crate::item::item::Item;
  
fn main() {
    let items = load_items_via_path(Path::new("todo.txt"));
    println!("{:?}", items);
}


#[cfg(test)]
mod tests {
    // use super::*;
    // use indoc::indoc;
    // use item::Item;
}
