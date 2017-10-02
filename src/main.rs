extern crate chess;

use chess::game::Game;

fn main() {
    let mut g = Game::new();
    println!("{}", g.to_fen());
    g.play("e4");
    println!("{}", g.to_fen());
    g.play("c5");
    println!("{}", g.to_fen());
    g.play("Nf3");
    println!("{}", g.to_fen());
}
