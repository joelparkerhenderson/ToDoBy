extern crate cursive;
extern crate cursive_tabs;
extern crate cursive_calendar_view;
extern crate cursive_markup;
extern crate cursive_table_view;
extern crate cursive_tree_view;
extern crate nom;

use indoc::indoc;

mod item;
use crate::item::item::Item;
  
use cursive::{
    Cursive,
    traits::*,
    views::{
        Button, 
        Dialog, 
        DummyView, 
        EditView,
        LinearLayout, 
        SelectView
    },
};


fn items() -> Vec<Item> {
    let input = indoc!{"
        [ ] Alpha
        [!] Bravp
        [x] Charlie and this is a really long one
    "};
    let (_input, items) = crate::item::item_parser::many0(input).expect("items");
    items
}

fn main() {

    let mut siv = cursive::default();

    let mut select_view = SelectView::<String>::new();
    for item in items() {
        select_view.add_item_str(item.to_string());
    }

    let select = select_view
        .on_submit(on_submit)
        .with_name("select");
        //.fixed_size((10, 5));
    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons))
        .title("Select a profile"));

    siv.run();
}

fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name)
        });
        s.pop_layer();
    }

    s.add_layer(Dialog::around(EditView::new()
            .on_submit(ok)
            .with_name("name")
            .fixed_width(10))
        .title("Enter a new name")
        .button("Ok", |s| {
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}

fn delete_name(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(format!("Name: {}\nAwesome: yes", name))
        .title(format!("{}'s info", name))
        .button("Quit", Cursive::quit));
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use indoc::indoc;
    // use item::Item;
}
