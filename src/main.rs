
mod radio;
use crate::radio::view::UserInterface;
use cursive::reexports::ahash::HashMap;
use cursive::{Cursive, CursiveRunnable};
use cursive::views::{Button, Dialog, DummyView, LinearLayout, NamedView, ResizedView, SelectView, EditView};
use cursive::traits::*;
use cursive::theme::Theme;

use crate::radio::control::on_submit;


use crate::radio::files::{get_name, create_table};
fn main() {
    // let mut ui = UserInterface::init();
    // ui.set_dir("./music");
    // ui.run();

    let table = create_table("./music");
    
    let mut s = cursive::default();

    let select = song_menu();
    s.add_layer(select);
    for (name, path_list) in table {
        let name = name.clone();
        s.call_on_name("select", |v: &mut SelectView<String>| {
            v.add_item_str(name);
        });
    }
    s.run();

}

fn song_menu() -> ResizedView<NamedView<SelectView>> {
    SelectView::<String>::new()
        .with_name("select")
        .fixed_size((100, 5))
}

fn add_name(s: &mut Cursive, table: &mut HashMap<String, Vec<String>>) {
    for (name, path_list) in table {
        let name = name.clone();
        s.call_on_name("select", |v: &mut SelectView<String>| {
            v.add_item_str(name);
        });
    }
}

