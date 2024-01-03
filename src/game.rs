use crate::board::*;

#[derive(Clone, Copy, Debug)]
struct Game {
    board: CheckerBoard,
    turn: u32,
}

impl Default for Game {
    fn default() -> Self {
        Game { board: CheckerBoard::default(), turn: 0 }
    }
}

impl Game {
    fn make_move(&mut self, r#move: Move) {
        if ()
    }
}

