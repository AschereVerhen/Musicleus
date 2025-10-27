use audio::*;
use std::path::Path;


fn main() {
    //First lets test with a hardcoded music path:
    // ~/Downloads/music.mp3
    let audio_path: &Path = Path::new("/home/aschere/Downloads/music.mp3");
    let music = Music {
        name: "hope to see you again".to_string(),
        path: &audio_path,
        id: 1 //since its the first song we play
    };

    match start_playing(music.path.to_str().unwrap()) {
        Ok((_stream, sink)) => {
            //Music will start playing by now...
            println!("Starting song: {}", music.name);
            println!("Pausing Playback in 5 seconds");
            std::thread::sleep(std::time::Duration::from_secs(5));
            pause_playing(&sink);
            println!("Resume playing in 5 seconds");
            std::thread::sleep(std::time::Duration::from_secs(5));
            resume_playing(&sink);
            sink.sleep_until_end();
        },
        Err(e) => {
            println!("Error Occured: {e:?}")
        }
    }
}
