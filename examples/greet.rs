use std::env;

fn main() {
    let name: Option<String> = env::args().skip(1).next();
    match name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, world!"),
    }
}
