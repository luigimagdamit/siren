use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;

use cursive::Cursive;
use cursive::views::Dialog;
use std::{thread, time::Duration};
use crate::radio::radio::{Radio, Song};
enum Command {
    Play,
    Stop
}

/// creates a process for audio playback
/// then creates a popup that will stop the process
/// when it quits out
/// Tx = transmitter
pub fn on_submit(s: &mut Cursive, name: &str) {
    let tx = spawn(name);
    let _ = tx.send(Command::Play);
    
    s.add_layer(
        Dialog::text(format!("Playing: \"{}\"", name)
    )
        .title(name.to_string())
        .button("Quit", move |s| {
            let _ = tx.send(Command::Stop);
            s.pop_layer();
        })
    );
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
    match r {
        Ok(mut radio) => {
            loop {
                if let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        Command::Play => {
                            let song = Song::new(&path, &path);
                            let _ = &radio.queue_song(song);
                            let _ = radio.change_song(0);
                        },
                        Command::Stop => {
                            let _ = &radio.stop_song();
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