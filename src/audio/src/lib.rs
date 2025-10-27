use std::path::Path;
use rodio::{Decoder, OutputStream, /*source::Source,*/ Sink};
use std::fs::File;
use std::io::BufReader;
pub struct Music<'a> {
    pub name: String,
    pub path: &'a Path, //Should be converted to Some(&str) through Path.to_str()
    pub id: u32
}


pub fn start_playing(path: &str) -> Result<(OutputStream, Sink), Box<dyn std::error::Error>> {
    /*
    Use the Default output medium of the system. For eg, the default speaker, or the connected headphone
    */
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let file = BufReader::new(File::open(path)?);
    let file_decoded = Decoder::new(file)?;
    
    /*
    Make a new sink, that can append, play, pause, or change the speed of a song. Very usefull.
    */
    let sink = Sink::connect_new(&stream_handle.mixer());
    sink.append(file_decoded);
    /* 
    Return both sink and stream_handle as if the handle is dropped out of the scope, then sink will stop
    */
    Ok((stream_handle, sink))
}

pub fn pause_playing(sink: &Sink) -> () {
    //This pauses the sink if not already paused.
    sink.pause();
}

pub fn resume_playing(sink: &Sink) -> () {
    sink.play();
}
