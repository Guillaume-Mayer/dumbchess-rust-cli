extern crate chess;

fn main() {
    let b = chess::chess::Board::new();
    println!("Hello Chess\n{}", b.to_str());
}
