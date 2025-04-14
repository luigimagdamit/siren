use std::{fmt::Error, fs::File, thread};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, PlayError, Sink, StreamError};


pub struct RadioMeta {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    pub sink: Option<Sink>
}
pub enum RadioMetaError {
    StreamOpenFail,
    SinkInitFail
}
impl RadioMeta {
    pub fn new() -> Result<RadioMeta, RadioMetaError>{
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
    pub fn init_sink(&mut self) -> Result<(), RadioMetaError>{
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

pub struct Radio <'a> {
    pub metadata: RadioMeta,
    songs: Vec<Song<'a>> // should hold the song names. promise that the string will have lifetime or shorter than a
}

pub enum RadioError {
    IndexNotFound,
    InitFailure,
    SourceFailure
}
impl <'a> Radio <'a> {
    pub fn new() -> Result<Radio<'a> , RadioMetaError> {
        let mut metadata = RadioMeta::new()?;
        metadata.init_sink()?;

        Ok(Radio {
            metadata,
            songs: Vec::new()
        })
    }
    pub fn queue_song(&mut self, song: Song<'a>) {
        self.songs.push(song);
    }
    
    pub fn change_song(&self, index: usize) -> Result<(), RadioError> {
        if let Some(song) = self.songs.get(index) {
            let source = Radio::create_source(song.path);
            if let Some(sink) = &self.metadata.sink {
                sink.append(source?);
                sink.sleep_until_end();
            }
            Ok(())
        } else {
            Err(RadioError::IndexNotFound)
        }
        
    }
    pub fn create_source(path: &'a str) -> Result<Decoder<BufReader<File>>, RadioError> {
        let file = File::open(path);
        if let Ok(file_success) = file {
            let source = Decoder::new(BufReader::new(file_success)).unwrap();
            return Ok(source);
        }
        Err(RadioError::SourceFailure)
        
    }
}
pub struct Song <'a> {
    pub name: &'a str,
    pub path: &'a str,

    
}
impl <'a> Song <'a> {
    pub fn new(name: &'a str, path: &'a str) -> Song <'a> {
        Song {name, path }
    }
    pub fn change_name(&mut self, new_name: &'a str) {
        self.name = new_name;
    }
    pub fn change_path(&mut self, new_path: &'a str) {
        self.path = new_path;
    }

        
}
pub enum SongError {
    PathNotFound,
    FileOpenFailure,
    Unknown
}