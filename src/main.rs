use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, ResizedView, SelectView};
use cursive::traits::*;
mod radio;
use cursive::theme::{BaseColor, Color, PaletteColor, Theme, Color::Rgb};
use radio::radio::{Radio, Song};
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader, sync::{Arc, Mutex}, thread, time::Duration};
use std::fs;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
fn init_radio_test(filename: &str) {
    // Create an output stream and a stream handle
    let r  = Radio::new();
    let song_name = String::from(filename);
    let song_path = String::from(filename);
    let song = Song::new(&song_name, &song_path);
    
    match r {
        Ok(mut radio) => {
            
            let _ = &radio.queue_song(song);
            let _ = radio.change_song(0);
        }
        Err(_) => panic!()
    }

}
fn get_files() -> Vec<String> {
    let paths = fs::read_dir("./music").unwrap();

    let mut res = Vec::new();
    for path in paths {
        if let Ok(p) = path {

            if let Some(pt) = p.path().to_str() {
                res.push(pt.to_string());
            }
            
        }
    }
    res
}
fn Windows98_theme(theme: &mut Theme) {
    theme.palette[PaletteColor::Background] = Rgb(0, 128, 128); // Turquoise (teal)
    theme.palette[PaletteColor::View] = Color::Dark(BaseColor::White);
    theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::Black); // White text
    theme.palette[PaletteColor::TitlePrimary] = Color::Dark(BaseColor::Blue); // Classic title bar
    theme.palette[PaletteColor::Highlight] = Color::Dark(BaseColor::Red);   // Highlight
    theme.palette[PaletteColor::HighlightText] = Color::Dark(BaseColor::White);
}
fn main() {
    let mut siv = cursive::default();
    // Clone the current theme so we can modify it
    let mut theme = siv.current_theme().clone();

    
    Windows98_theme(&mut theme);
    // Apply the theme
    siv.set_theme(theme);
    let select = SelectView::<String>::new().on_submit(on_submit).with_name("select")
        
        .fixed_size((100, 5));



    
    let buttons = buttons();

    siv.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(select)
        .child(DummyView)
        .child(buttons))
    .title("Select a file to play"));
    // siv.run();
    
    let filenames = get_files();
    for name in filenames {
        add_title(&mut siv, &name);
    }
    siv.run();
}
fn buttons() -> LinearLayout {
    let buttons = LinearLayout::vertical()
    .child(Button::new("Add new", add_name))
    .child(Button::new("Delete", delete_name))
    .child(DummyView)
    .child(Button::new("Quit", Cursive::quit));

    buttons
    
}
fn add_title(s: &mut Cursive, name: &str) {
    s.call_on_name("select", |v: &mut SelectView<String>| {
        v.add_item_str(name);
    });
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

enum Command {
    Play,
    Stop
}
fn spawn(path: &str) -> Sender<Command> {
    let (tx, rx) = mpsc::channel();
    let p = String::from(path);
    thread::spawn(move || {
        play(&rx, p);
    });
    tx
}
fn play(rx: &Receiver<Command>, path: String) {
    let r  = Radio::new();
    let song = Song::new(&path, &path);
    
    match r {
        Ok(mut radio) => {
            loop {

                if let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        Command::Play => {
                            println!("playing");
                            let song = Song::new(&path, &path);
                            let _ = &radio.queue_song(song);
                            let _ = radio.change_song(0);
                        },
                        Command::Stop => {
                            // if let Some(sink) = &radio.metadata.sink {
                            //     sink.pause();
                            // }
                            break;


                        }
                    }
                }
                thread::sleep(Duration::from_millis(100));
            }
            
        }
        Err(_) => panic!()
    }
}
fn on_submit(s: &mut Cursive, name: &str) {
    // s.pop_layer();

    let tx = spawn(name);
    let a = tx.send(Command::Play);
    
    // let new_str = String::from(name);
    // let handle = std::thread::spawn(move || {
    //     init_radio_test(&new_str);
    // });
    let tx_quit = tx.clone();

    s.add_layer(Dialog::text(format!("Playing: {}", name))
        .title(format!("{}", name))
        .button("Quit", move |s| {
            let _ = tx_quit.send(Command::Stop);
            s.pop_layer();
        })
    );
}
// fn on_submit(s: &mut Cursive, name: &str) {
//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));

//     // Load the audio file
//     let file = File::open(name).unwrap();
//     let source = Decoder::new(BufReader::new(file)).unwrap();

//     // Spawn the thread to handle audio playback
//     let sink_clone = Arc::clone(&sink);
//     println!("{}", name);
//     thread::spawn(move || {
//         // Lock the sink and start playback
//         let sink = sink_clone.lock().unwrap();
//         println!("aaa");
//         sink.append(source);
//         sink.sleep_until_end();
//     });
//     println!("aaa");
//     // Spawn a control thread to interrupt the playback
//     let sink_clone = Arc::clone(&sink);
//     thread::spawn(move || {
//         // Let the playback run for a few seconds, then interrupt


//         // Lock the sink and stop the playback
//         let mut sink = sink_clone.lock().unwrap();
//         thread::sleep(Duration::from_secs(2));
//         sink.stop();  // Interrupts playback
        

//     });
// }
// fn main() {
//     let mut siv = cursive::default();
//     siv.add_layer(Dialog::text("this a survey")
//         .title("important survey")
//         .button("next", show_next));
//     siv.run();

// }
// empty unnamed for the chaining
// fn show_next(s: &mut Cursive) {
//     // to make a popup show up
//     s.pop_layer();
//     s.add_layer(Dialog::text("did you do the thing")
//         .title("question 1")
//         .button("yes!", |s| ())
//         .button("uhhh", |s| s.add_layer(Dialog::text("try again lol"))));
// }
