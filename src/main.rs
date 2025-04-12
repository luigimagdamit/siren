use cursive::views::{Dialog, TextView};
mod radio;

use radio::radio::{Radio, Song};

fn init_radio_test() {
    // Create an output stream and a stream handle
    let r  = Radio::new();
    let song_name = String::from("test");
    let song_path = String::from("./music/test2.mp3");
    let song = Song::new(&song_name, &song_path);
    if let Ok(mut radio) = r {
        let _ = &radio.queue_song(song);
        let _ = radio.change_song(0);
    }
}
fn main() {
    init_radio_test();

    
    // let mut siv = cursive::default();

    // // Creates a dialog with a single "Quit" button
    // siv.add_layer(Dialog::around(TextView::new("Hello Dialog!"))
    //                      .title("Cursive")
    //                      .button("Quit", |s| s.quit()));

    // // Starts the event loop.
    // siv.run();

}
