use std::{fmt::Error, fs::File};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, PlayError, Sink, StreamError};

struct RadioMeta {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Option<Sink>
}
pub enum RadioMetaError {
    StreamOpenFail,
    SinkInitFail
}
impl RadioMeta {
    fn new() -> Result<RadioMeta, RadioMetaError>{
        let open_output_stream = OutputStream::try_default();
        match open_output_stream {
            Ok((_stream, stream_handle)) => {
                let success = RadioMeta {
                    _stream,
                    stream_handle,
                    sink: None
                };
                Ok(success)
            },
            Err(_) => Err(RadioMetaError::StreamOpenFail)
        }
    }
    fn init_sink(&mut self) -> Result<(), RadioMetaError>{
        let sink_result = Sink::try_new(&self.stream_handle);
        match sink_result {
            Ok(sink_success) => {
                self.sink = Some(sink_success);
                Ok(())
            },
            Err(_) => Err(RadioMetaError::SinkInitFail)   
        }
    }
}
struct Radio {
    metadata: RadioMeta,
    songs: Vec<Song> // should hold the song names. promise that the string will have lifetime or shorter than a
}

impl Radio {
    fn new() -> Result<Radio, RadioMetaError> {
        let mut metadata = RadioMeta::new()?;
        metadata.init_sink()?;

        Ok(Radio {
            metadata,
            songs: Vec::new()
        })
    }
    fn queue_song(&mut self, song: Song) {
        self.songs.push(song);
    }
    fn change_song(&self, index: usize) {
        let song = self.songs.get(index);
        match song {
            Some(s) => {
                let file = File::open(s.path.clone()).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();
                if let Some(sink) = &self.metadata.sink {
                    sink.append(source);
                    sink.sleep_until_end();
                }

            },
            None => panic!()
        }
    //     if let Some(song_success) = &song {
    //         match &song_success.fd {
    //             Some(st) => {
    //                 let source = Decoder::new(BufReader::new(st)).unwrap();
        
    //                 if let Some(s) = &self.metadata.sink {
    //                     // Play the audio
    //                     s.append(source);
                
    //                     // block current call until the end
    //                     s.sleep_until_end();
    //                 }
    //             },
    //             _ => panic!()
    //         }
            
    //     }
        
    }
}
struct Song {
    name: String,
    path: String,

    
}
impl Song {
    fn new(name: String, path: String) -> Song {
        Song {name, path }
    }
    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    fn change_path(&mut self, new_path: String) {
        self.path = new_path;
    }

        
}
pub enum SongError {
    PathNotFound,
    FileOpenFailure,
    Unknown
}
fn main() {
    // Create an output stream and a stream handle
    let r  = Radio::new();

    match r {
        Ok(mut success) => {
            println!("uhh");
            let song = Song::new(String::from("test"), String::from("./music/test2.mp3"));

            success.queue_song(song);
            success.change_song(0);

        }
        Err(_) => panic!()
    }
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // // Create a sink (audio output)
    // let sink = Sink::try_new(&stream_handle).unwrap();

    // Load the audio file

}
