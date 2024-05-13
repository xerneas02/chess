#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]

use core::num;
use std::fmt;
use std::io;
use itertools::max;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
use std::time::Instant;
use itertools::Itertools;
use std::mem::transmute;


mod pieces;
mod bitboard;
mod game;
mod color;

use color::*;
use game::*;
use bitboard::Bitboard;
use pieces::Piece;
use Piece::*;
use pieces::MAP;

fn main() {
    let max_turn: f64 = 300.0;

    let mut bitboards: [Bitboard; Piece::COUNT * Color::COUNT] = get_bitboards();
    let mut last_opponent_move: Option<(usize, usize)> = None;
    let mut castle: [bool; Color::COUNT] = [true; Color::COUNT];
    
    let mut turn: bool = false;
    let mut win : bool = false;
    let mut count_turn: f64 = 0.0;

    let mut time_mean: f64 = 0.0;

    while !win && count_turn < max_turn {
        display_board(&bitboards);

        let start = Instant::now();
        if turn {
            play_player(&mut bitboards, turn, &mut last_opponent_move, &mut castle);
        }
        else {
            play_ai(&mut bitboards, turn, &mut last_opponent_move, &mut castle, 5, true);
        }
        let duration = start.elapsed();

        time_mean += duration.as_secs_f64();

        check_win_conditions(&bitboards, &mut win, true);
        turn = !turn;

        count_turn += 1.0;
        
    }

    display_board(&bitboards);
    print!("{}ms", (time_mean/count_turn)*1000.0);

}
