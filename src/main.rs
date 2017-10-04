extern crate chess;

use chess::game::Game;
use std::io;
use std::io::Write;

fn main() {
    let mut g = Game::new();
    println!("Welcome to DumbChess");
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut comm = String::new();
        io::stdin().read_line(&mut comm).expect("Failed to read line");
        let comm = comm.trim();
        match comm {
            "quit" | "exit" => break,
            "new" => g = Game::new(),
            "fen" => println!("{}", g.to_fen()),
            "pgn" => println!("{}", g.to_pgn()),
            _ => g.play(comm),
        }
    }
}
