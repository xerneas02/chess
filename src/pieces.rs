use std::usize;

use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Debug, EnumCountMacro, EnumIter, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5
}

pub const MAP : [char ; Piece::COUNT] = ['P', 'H', 'B', 'R', 'Q', 'K'  ];

const VAL: [isize; Piece::COUNT] = [
    100,         // Pawn
    320,         // Knight
    330,         // Bishop
    500,         // Rook
    900,         // Queen
    100,         // King
];

impl Piece {
    pub fn usize_to_piece(us : usize) -> Piece{
        match us {
            0 => Piece::Pawn  ,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook  ,
            4 => Piece::Queen ,
            5 => Piece::King  ,
            _ => Piece::King
        }
    }

    pub fn value(&self) -> isize{
        VAL[*self as usize]
    }
}