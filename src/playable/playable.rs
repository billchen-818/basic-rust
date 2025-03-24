pub trait Playable {
    fn play(&self);

    fn pause(&self) {
        println!("Pausing...");
    }
}
