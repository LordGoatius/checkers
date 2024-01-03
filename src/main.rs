pub mod board;
pub mod game;

use crate::board::*;

fn main() {
    let board = CheckerBoard::default();
    println!("{board}");
}
