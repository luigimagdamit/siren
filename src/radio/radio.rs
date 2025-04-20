use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

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

pub struct Radio  {
    pub metadata: RadioMeta,
    songs: Vec<Song> // should hold the song names. promise that the string will have lifetime or shorter than a
}

#[warn(dead_code)]
pub enum RadioError {
    IndexNotFound,
    InitFailure,
    SourceFailure
}
impl <'a> Radio  {
    pub fn new() -> Result<Radio, RadioMetaError> {
        let mut metadata = RadioMeta::new()?;
        metadata.init_sink()?;
        Ok(Radio {
            metadata,
            songs: Vec::new()
        })
    }
    pub fn queue_song(&mut self, song: Song) {
        self.songs.push(song);
    }
    
    pub fn change_song(&self, index: usize) -> Result<(), RadioError> {
        if let Some(song) = self.songs.get(index) {
            let source = Radio::create_source(&song.path);
            if let Some(sink) = &self.metadata.sink {
                if !sink.empty() { sink.stop(); }
                sink.append(source?);
            }
            Ok(())
        } else { Err(RadioError::IndexNotFound) }
        
    }
    pub fn stop_song(&self)  {
        if let Some(sink) = &self.metadata.sink {
            sink.stop();
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
#[derive(Clone)]
pub struct Song  {
    pub name: String,
    pub path: String,

    
}

impl Song  {
    pub fn new(name: String, path: String) -> Song  {
        Song {name, path }
    }
    pub fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    pub fn change_path(&mut self, new_path: String) {
        self.path = new_path;
    }

        
}
pub enum SongError {
    PathNotFound,
    FileOpenFailure,
    Unknown
}