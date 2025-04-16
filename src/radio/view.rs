use cursive::{Cursive, CursiveRunnable};
use cursive::views::{Button, Dialog, DummyView, LinearLayout, NamedView, ResizedView, SelectView, EditView};
use cursive::traits::*;
use cursive::theme::Theme;

use crate::radio::theme::InterfaceTheme;
use crate::radio::control::on_submit;
use crate::radio::files::get_files;

pub struct UserInterface<'a> {
    siv: CursiveRunnable,
    dir: &'a str
}
impl<'a>UserInterface <'a>{
    pub fn init() -> Self {
        UserInterface { 
            siv: cursive::default(),
            dir: ""
        }
    }
    pub fn set_dir(&mut self, dir: &'a str) {
        self.dir = dir;
    }
    pub fn run(&mut self) {
        self.set_theme(InterfaceTheme::get(InterfaceTheme::Windows98));
        self.build_view();
        self.init_titles();
        
        self.siv.run();
    }
    fn menu_buttons() -> LinearLayout {
        LinearLayout::vertical()
            .child(Button::new("Add new", add_name))
            .child(Button::new("Delete", delete_name))
            .child(DummyView)
            .child(Button::new("Quit", Cursive::quit))
    
    }
    fn add_title(&mut self, name: &str) {
        self.siv.call_on_name("select", |v: &mut SelectView<String>| {
            v.add_item_str(name);
        });
    }
    fn init_titles(&mut self) {
        let filenames = get_files(self.dir);
        for name in filenames {
            self.add_title(&name);
        }
    }
    fn select() -> ResizedView<NamedView<SelectView>> {
        SelectView::<String>::new()
            .on_submit(on_submit)
            .with_name("select")
            .fixed_size((100, 5))
    }
    fn build_view(&mut self) {
        let select = UserInterface::select();
        let buttons = UserInterface::menu_buttons();

        self.siv.add_layer(
            Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons))
            .title("Select a file to play"));
    }
    fn set_theme(&mut self, theme: Theme) {
        self.siv.set_theme(theme);
    }

}

fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name);
        });
        s.pop_layer();
    }
    s.add_layer(Dialog::around(EditView::new()
        .on_submit(ok)
        .with_name("name")
        .fixed_width(10))
        .title("Enter a new name")
        .button("Ok", |s| {
            let name = s.call_on_name("name", |view: &mut EditView| {
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
        None => s.add_layer(Dialog::info("no name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}



