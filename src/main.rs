extern crate chess;

use chess::game::Game;
use chess::mov::Mov;

fn main() {
    let g = Game::new();
    let m = Mov::Quiet(1, 2);
    //g.play(m);
}
