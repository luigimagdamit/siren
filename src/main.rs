
mod radio;
use crate::radio::view::UserInterface;
use std::env;


use crate::radio::files::{get_name, create_table};
fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let table = create_table(&path);
    let mut ui = UserInterface::init();
    ui.set_table(table);
    ui.set_dir("");
    ui.run();

}


