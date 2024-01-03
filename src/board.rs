use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Checker {
    Red(bool),
    Black(bool),
}

impl Checker {
    pub fn promote(&mut self) {
        match self {
            Checker::Red(val) => *val = true,
            Checker::Black(val) => *val = true,
        }
    }

    fn is_king(&self) -> bool {
        match self {
            Checker::Red(val) => *val,
            Checker::Black(val) => *val,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CheckerBoard {
    board: [[Option<Checker>; 8]; 8]
}

#[derive(Clone, Copy, Debug)]
pub struct Position(u8, u8);

impl Position {
    fn x(&self) -> u8 {
        self.1
    }

    fn y(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Move(Position, Position);

#[derive(Clone, Debug)]
pub struct MoveChain {
    moves: Vec<Move>
}

impl CheckerBoard {
    fn at(&self, position: Position) -> Option<&Checker> {
        self.board[position.0 as usize][position.1 as usize].as_ref()
    }

    fn at_mut(&mut self, position: Position) -> &mut Option<Checker> {
        &mut self.board[position.0 as usize][position.1 as usize]
    }

    fn move_piece(&mut self, jump: Move) {
        self.board[jump.1.0 as usize][jump.1.1 as usize] = self.board[jump.0.0 as usize][jump.0.1 as usize];
        *self.at_mut(jump.0) = None;
    }

    fn remove_piece(&mut self, position: Position) {
        *self.at_mut(position) = None;
    }

    fn promote_piece(&mut self, position: Position) {
        if let Some(piece) = self.at_mut(position) {
            piece.promote();
        }
    }

    fn valid_move(&self, jump: Move) -> bool {
        let start = self.at(jump.0);
        let end = self.at(jump.1);

        let king: bool;

        if let Some(val) = start {
            king = val.is_king();
        } else {
            return false;
        }

        match start.unwrap() {
            Checker::Red(val) => {},
            Checker::Black(val) => {}
        }




        let x_diff = u8::abs_diff(jump.0.x(), jump.1.x());
        let y_diff = u8::abs_diff(jump.0.y(), jump.1.y());

        if x_diff == 1 && y_diff == 1 && end == None {
            true
        } else if x_diff == 2 && y_diff == 2 && end == None {

        }

        false
    }
}

impl Display for CheckerBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_str: String = "   ┌───┬───┬───┬───┬───┬───┬───┬───┐\n".to_owned();

        for i in (0..8).rev() {
            let mut to_append: String = format!(" {i} │");
            for j in 0..8 {
                let piece = &self.board[i][j];
                match piece {
                    Some(Checker::Red(_)) => { to_append.push_str(" r │")  },
                    Some(Checker::Black(_)) => { to_append.push_str(" b │") },
                    None => to_append.push_str("   │"),
                }
            }
            to_append.push('\n');
            board_str.push_str(&to_append[..]);
            if i != 0 {
                board_str.push_str("   ├───┼───┼───┼───┼───┼───┼───┼───┤\n");
            }
        }
        board_str.push_str("   └───┴───┴───┴───┴───┴───┴───┴───┘\n");
        board_str.push_str("     A   B   C   D   E   F   G   H  \n");
        write!(f, "{board_str}")
    }
}

impl Default for CheckerBoard {
    fn default() -> Self {
        Self { board: [
            [None, Some(Checker::Red(false)), None, Some(Checker::Red(false)), None, Some(Checker::Red(false)), None, Some(Checker::Red(false))],
            [Some(Checker::Red(false)), None, Some(Checker::Red(false)), None, Some(Checker::Red(false)), None, Some(Checker::Red(false)), None],
            [None, Some(Checker::Red(false)), None, Some(Checker::Red(false)), None, Some(Checker::Red(false)), None, Some(Checker::Red(false))],
            [None; 8],
            [None; 8],
            [Some(Checker::Black(false)), None, Some(Checker::Black(false)), None, Some(Checker::Black(false)), None, Some(Checker::Black(false)), None],
            [None, Some(Checker::Black(false)), None, Some(Checker::Black(false)), None, Some(Checker::Black(false)), None, Some(Checker::Black(false))],
            [Some(Checker::Black(false)), None, Some(Checker::Black(false)), None, Some(Checker::Black(false)), None, Some(Checker::Black(false)), None],
        ]}
    }
}

//==========// First Row //============//
pub const A1: Position = Position(7, 0);
pub const B1: Position = Position(7, 1);
pub const C1: Position = Position(7, 2);
pub const D1: Position = Position(7, 3);
pub const E1: Position = Position(7, 4);
pub const F1: Position = Position(7, 5);
pub const G1: Position = Position(7, 6);
pub const H1: Position = Position(7, 7);

//==========// Second Row //===========//
pub const A2: Position = Position(6, 0);
pub const B2: Position = Position(6, 1);
pub const C2: Position = Position(6, 2);
pub const D2: Position = Position(6, 3);
pub const E2: Position = Position(6, 4);
pub const F2: Position = Position(6, 5);
pub const G2: Position = Position(6, 6);
pub const H2: Position = Position(6, 7);

//==========// Third Row //============//
pub const A3: Position = Position(5, 0);
pub const B3: Position = Position(5, 1);
pub const C3: Position = Position(5, 2);
pub const D3: Position = Position(5, 3);
pub const E3: Position = Position(5, 4);
pub const F3: Position = Position(5, 5);
pub const G3: Position = Position(5, 6);
pub const H3: Position = Position(5, 7);

//==========// Fourth Row //===========//
pub const A4: Position = Position(4, 0);
pub const B4: Position = Position(4, 1);
pub const C4: Position = Position(4, 2);
pub const D4: Position = Position(4, 3);
pub const E4: Position = Position(4, 4);
pub const F4: Position = Position(4, 5);
pub const G4: Position = Position(4, 6);
pub const H4: Position = Position(4, 7);

//==========// Fifth Row //============//
pub const A5: Position = Position(3, 0);
pub const B5: Position = Position(3, 1);
pub const C5: Position = Position(3, 2);
pub const D5: Position = Position(3, 3);
pub const E5: Position = Position(3, 4);
pub const F5: Position = Position(3, 5);
pub const G5: Position = Position(3, 6);
pub const H5: Position = Position(3, 7);

//==========// Sixth Row //============//
pub const A6: Position = Position(2, 0);
pub const B6: Position = Position(2, 1);
pub const C6: Position = Position(2, 2);
pub const D6: Position = Position(2, 3);
pub const E6: Position = Position(2, 4);
pub const F6: Position = Position(2, 5);
pub const G6: Position = Position(2, 6);
pub const H6: Position = Position(2, 7);

//==========// Seventh Row //==========//
pub const A7: Position = Position(1, 0);
pub const B7: Position = Position(1, 1);
pub const C7: Position = Position(1, 2);
pub const D7: Position = Position(1, 3);
pub const E7: Position = Position(1, 4);
pub const F7: Position = Position(1, 5);
pub const G7: Position = Position(1, 6);
pub const H7: Position = Position(1, 7);

//==========// Eighth Row //===========//
pub const A8: Position = Position(0, 0);
pub const B8: Position = Position(0, 1);
pub const C8: Position = Position(0, 2);
pub const D8: Position = Position(0, 3);
pub const E8: Position = Position(0, 4);
pub const F8: Position = Position(0, 5);
pub const G8: Position = Position(0, 6);
pub const H8: Position = Position(0, 7);

#[derive(Clone, Copy, Debug)]
//============//
// Not Using //
//============//

struct Bitboard(u64);

struct BitboardBoard {
    pub red_bitboard:   Bitboard,
    pub black_bitboard: Bitboard,
}

impl Display for BitboardBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::from("   ┌───┬───┬───┬───┬───┬───┬───┬───┐\n   │");
        for i in 1..=64 {
            if ((self.red_bitboard.0 >> (64 - i)) & 1) == 1 {
                board.push_str(" r │");
            } else if ((self.black_bitboard.0 >> (64 - i)) & 1) == 1 {
                board.push_str(" b │");
            } else {
                board.push_str("   │");
            }
            if i % 8 == 0 && i != 0 && i != 64 {
                board.push_str("\n   ├───┼───┼───┼───┼───┼───┼───┼───┤\n   │");
            } if i == 64 {
                board.push_str("\n");
            }
        }
        board.push_str("   └───┴───┴───┴───┴───┴───┴───┴───┘\n");
        board.push_str("     A   B   C   D   E   F   G   H  \n");
        write!(f, "{board}")
    }
}
