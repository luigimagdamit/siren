use std::collections::HashMap;
use std::path;
use std::rc::Rc;
use cursive::reexports::crossbeam_channel::select;
use cursive::{Cursive, CursiveRunnable};
use cursive::views::{Button, Dialog, DummyView, LinearLayout, NamedView, ResizedView, SelectView, EditView};
use cursive::traits::*;
use cursive::theme::Theme;
use crate::radio::theme::InterfaceTheme;
use crate::radio::control::on_submit;
use crate::radio::files::get_files;

pub struct UserInterface<'a> {
    siv: CursiveRunnable,
    dir: &'a str,
    table: HashMap<String, Vec<String>>
}
impl<'a>UserInterface <'a>{
    pub fn init() -> Self {
        UserInterface { 
            siv: cursive::default(),
            dir: "",
            table: HashMap::new()
        }
    }
    pub fn set_dir(&mut self, dir: &'a str) {
        self.dir = dir;
    }
    pub fn set_table(&mut self, table: HashMap<String, Vec<String>>) {
        self.table = table;
    }
    pub fn run(&mut self) {
        self.set_theme(InterfaceTheme::get(InterfaceTheme::Windows98));
        self.build_view();
        // self.init_titles(get_files("./music"));
        
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
    fn init_titles(&mut self, paths: Vec<String>) {
        let filenames = paths;
        for name in filenames {
            self.add_title(&name);
        }
    }
    fn select() -> ResizedView<NamedView<SelectView>> {
        SelectView::<String>::new()
            .on_submit(on_submit)
            .with_name("select_version")
            .fixed_size((100, 5))
    }
    fn build_view(&mut self) {
        let select = UserInterface::select();
        let buttons = UserInterface::menu_buttons();

        // self.siv.add_layer(
        //     Dialog::around(LinearLayout::horizontal()
        //     .child(select)
        //     .child(DummyView)
        //     .child(buttons))
        //     .title("Select a file to play"));
        let menu = self.song_menu();
        self.siv.add_layer(
            Dialog::around(LinearLayout::horizontal()
            .child(menu)
            .child(DummyView)
            .child(buttons))
            .title("Songs")
            .button("Exit", |s| {
                s.quit();
            }));
        for (name, path_list) in &self.table {
            println!("{}", name);
            let name = name.clone();
            self.siv.call_on_name("select", |v: &mut SelectView<String>| {
                v.add_item_str(name);
            }
        );}
    }
    fn select_song(&mut self) -> Dialog {
        let version_select = UserInterface::select();
        let menu = Dialog::around(LinearLayout::horizontal()
            .child(version_select)
            .child(DummyView)
            ).title("Select a file to play");
        menu
        
    }
    fn song_menu(&mut self) -> ResizedView<NamedView<SelectView>> {
        let table = self.table.clone();

        let menu = SelectView::<String>::new()
            
            .on_submit(move |s, name: &str| {
                let table = &table;
                let versions = table.get(name).unwrap().clone();
                s.add_layer(UserInterface::version_menu(name));
                
                for version_name in versions {
                    s.call_on_name("select_version", |v: &mut SelectView<String>| {
                        v.add_item_str(version_name);
                    });
                }
                
                
            })
            
            .with_name("select")
            .fixed_size((100, 5));
        
        menu
    }

    fn version_menu(name: &str) -> Dialog{
        let select = UserInterface::select();
        let buttons = UserInterface::menu_buttons();


            Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons))
            .title(name)
            .button("Back", |s| {
                s.pop_layer();
            })
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



