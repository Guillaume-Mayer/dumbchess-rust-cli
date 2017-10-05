extern crate client;

fn main() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("Welcome to DumbChess {}", VERSION);
    client::run();
}
