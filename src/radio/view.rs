use std::collections::HashMap;

use cursive::view::IntoBoxedView;
use cursive::{Cursive, CursiveRunnable};
use cursive::views::{Dialog, DummyView, LinearLayout, NamedView, ResizedView, SelectView};
use cursive::traits::*;
use cursive::theme::Theme;
use crate::radio::theme::InterfaceTheme;
use crate::radio::control::on_submit;
use crate::radio::radio::Song;

struct VersionMenu {
    view: SelectView
}
impl VersionMenu {
    fn new() -> VersionMenu {
        let view = SelectView::<String>::new();
        Self { view }
    }
    fn set_select_callback(mut self, f: fn(&mut Cursive, &str)) -> Self {
        self.view.set_on_submit(f);
        self
    }
    fn add_items(mut self, titles: Vec<String>) -> Self {
        for title in titles {
            let name = title.clone();
            self.view.add_item_str(name);
        }
        self
    }
    fn into_resized_view(self, title: &str, size: (i32, i32)) -> ResizedView<NamedView<SelectView>> {
        self.view.with_name(title).fixed_size(size)
    }
    fn build(versions: &Vec<String>) -> ResizedView<NamedView<SelectView>> {
        VersionMenu::new()
                    .set_select_callback(on_submit)
                    .add_items(versions.to_vec())
                    .into_resized_view("Title", (100, 5))
    }
    fn add_version_menu(s: &mut Cursive, name: &str, table: &HashMap<String, Vec<Song>>) {
    
        let mut versions = Vec::new();
        let song_list = table.get(name);

        match song_list {
            Some(song_list) => {
                for song in song_list {
                    versions.push(song.path.clone());
                }
                let menu_layout = Dialog::around(LinearLayout::horizontal()
                        .child(VersionMenu::build(&versions))
                        .child(DummyView))
                        .title("Versions")
                        .button("Back", |s| {
                            s.pop_layer();
                        });
                    s.add_layer(menu_layout);
            },
            None => panic!()
        }
        
    }

}
pub struct UserInterface<'a> {
    siv: CursiveRunnable,
    dir: &'a str,
    table: HashMap<String, Vec<Song>>
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
    pub fn set_table(&mut self, table: HashMap<String, Vec<Song>>) {
        self.table = table;
    }
    pub fn run(&mut self) {
        self.set_theme(InterfaceTheme::get(InterfaceTheme::Windows98));
        self.build_view();
        self.siv.run();
    }

    fn build_view(&mut self) {
        let menu = wrap_dialogue(self.create_song_menu());
        self.siv.add_layer(menu);
        for (name, _) in &self.table {
            self.siv.call_on_name("select", |v: &mut SelectView<String>| {
                v.add_item_str(name);
            }
        );}
    }

    fn create_song_menu(&mut self) -> ResizedView<NamedView<SelectView>> {
        let table = self.table.clone();
        SelectView::<String>::new()
            .on_submit(move |s, name: &str| {
                VersionMenu::add_version_menu(s, name, &table);
            })
            .with_name("select")
            .fixed_size((100, 5))
    }



    fn set_theme(&mut self, theme: Theme) {
        self.siv.set_theme(theme);
    }

}



fn wrap_dialogue<V: IntoBoxedView + 'static>(view: V) -> Dialog {
    Dialog::around(LinearLayout::horizontal()
    .child(view)
    .child(DummyView))
    .title("Songs")
    .button("Exit", |s| {
        s.quit();
    })
}