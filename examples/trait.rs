pub use basic_rust::playable::Playable;
use std::default::Default;

struct Audio(String);
struct Video(String);

impl Playable for Audio {
    fn play(&self) {
        println!("Playing audio: {}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Playing video: {}", self.0);
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self("audio.mp3".to_string())
    }
}

fn main() {
    let audio = Audio("audio.mp3".to_string());
    let video = Video("video.mp4".to_string());

    audio.play();
    audio.pause();

    video.play();
    video.pause();
}
