extern crate chess;

use chess::game::Game;

fn main() {
    let g = Game::new();
    println!("Hello Chess\n\n{:?}", g);
}
