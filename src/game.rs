use crate::bitboard::Bitboard;
use crate::pieces::Piece;
use crate::pieces::MAP;
use crate::color::*;
use std::collections::btree_map::Values;
use std::io;

use Piece::*;

use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

const ORDER: [usize; Piece::COUNT] = [Pawn as usize, Knight as usize, Bishop as usize, Rook as usize,  Queen as usize, King as usize]; 


#[derive(Debug, EnumCountMacro, EnumIter)]
pub enum Color {
    White = 0,
    Black = 1
}

pub fn algebraic_to_index(algebraic: &str, inverted: bool) -> Option<usize> {
    if algebraic.len() != 2 {
        return None;
    }
    let file = algebraic.chars().nth(0)?;
    let rank = algebraic.chars().nth(1)?;
    if file < 'A' || file > 'H' || rank < '1' || rank > '8' {
        return None;
    }
    let file_index = file as usize - 'A' as usize;
    let mut rank_index = rank as usize - '1' as usize;
    rank_index = if inverted {7 - rank_index} else {rank_index};
    Some(rank_index * 8 + file_index)
}

pub fn index_to_algebraic(index: usize, inverted: bool) -> String {
    let file_index = index % 8;
    let mut rank_index = index / 8;
    rank_index = if inverted {7 - rank_index} else {rank_index};

    let file = ('A' as u8 + file_index as u8) as char;
    let rank = ('1' as u8 + rank_index as u8) as char;

    format!("{}{}", file, rank)
}

pub fn invert_index(index:usize) -> usize {
    let file_index = index % 8;
    let mut rank_index = index / 8;
    rank_index = 7 - rank_index;
    rank_index * 8 + file_index
}

pub fn get_bitboards() -> [Bitboard; Piece::COUNT*Color::COUNT] {
    let mut bitboards: [Bitboard; Piece::COUNT*Color::COUNT] = [Bitboard{bits:0}; Piece::COUNT*Color::COUNT];

    bitboards[Pawn as usize].bits                  = 0b0000000000000000000000000000000000000000000000001111111100000000;
    bitboards[(Pawn as usize)+Piece::COUNT].bits   = bitboards[Pawn as usize].bits;

    bitboards[Rook as usize].bits                  = 0b0000000000000000000000000000000000000000000000000000000010000001;
    bitboards[(Rook as usize)+Piece::COUNT].bits   = bitboards[Rook as usize].bits;

    bitboards[Knight as usize].bits                = 0b0000000000000000000000000000000000000000000000000000000001000010;
    bitboards[(Knight as usize)+Piece::COUNT].bits = bitboards[Knight as usize].bits ;

    bitboards[Bishop as usize].bits                = 0b0000000000000000000000000000000000000000000000000000000000100100;
    bitboards[(Bishop as usize)+Piece::COUNT].bits = bitboards[Bishop as usize].bits;

    bitboards[Queen as usize].bits                 = 0b0000000000000000000000000000000000000000000000000000000000001000;
    bitboards[(Queen as usize)+Piece::COUNT].bits  = bitboards[Queen as usize].bits;

    bitboards[King as usize].bits                  = 0b0000000000000000000000000000000000000000000000000000000000010000;
    bitboards[(King as usize)+Piece::COUNT].bits   = bitboards[King as usize].bits;

    bitboards
}

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn get_player_piece_input(turn: bool) -> String {
    if !turn {
        print!("{}Red{} play.\n", RED, RESET);
    } else {
        print!("{}Blue{} play.\n", BLUE, RESET);
    }
    println!("Enter the piece you want to move:");
    get_input()
}

pub fn get_player_move_input(turn: bool, moves: &Vec<usize>) -> String {
    print!("Possible moves : ");
    for &m in moves {
        print!("{} ",  index_to_algebraic(m, turn));
    }
    print!("\n");
    println!("Enter your move :");
    get_input()
}


pub fn get_player_and_opponent_bitboards(bitboards: &[Bitboard; Piece::COUNT*Color::COUNT], turn: bool) -> (Bitboard, Bitboard) {
    let mut player = Bitboard{bits:0};
    let mut opponent = Bitboard{bits:0};

    for i in 0..Piece::COUNT {
        player.combine_bitboard(bitboards[i + if !turn {0} else {Piece::COUNT}]);
        opponent.combine_bitboard(bitboards[i + if turn {0} else {Piece::COUNT}]);
    } 
    (player, opponent)
}

pub fn get_piece_index_to_move(bitboards: &[Bitboard; Piece::COUNT*Color::COUNT], turn: bool, index: usize) -> Option<usize> {
    for i in 0..Piece::COUNT {
        if bitboards[i + if !turn {0} else {Piece::COUNT}].get_bit(index) == 1 {
            return Some(i);
        }
    }
    None
}

pub fn check_win_conditions(bitboards: &[Bitboard; Piece::COUNT*Color::COUNT], win: &mut bool, print: bool) {
    if bitboards[Piece::King as usize].bits == 0 {
        if print {println!("{}Blue{} wins!", BLUE, RESET);}
        *win = true;
    }
    if bitboards[(Piece::King as usize) + Piece::COUNT].bits == 0 {
        if print {println!("{}Red{} wins!", RED, RESET);}
        *win = true;
    }
}

pub fn update_game_state(bitboards: &mut [Bitboard; Piece::COUNT*Color::COUNT], opponent: Bitboard, last_opponent_move: &mut Option<(usize, usize)>, castle: &mut [bool; Color::COUNT], turn: bool, piece_index: usize, from_index: usize, to_index: usize) {
    let invert_input = invert_index(to_index);
    let mut en_passant: bool = false;
    let piece = Piece::usize_to_piece(piece_index);

    bitboards[piece_index + if !turn {0} else {Piece::COUNT}].move_piece(from_index, to_index);

    if let Some((_, to)) = *last_opponent_move {
        if to_index == to + 8 && piece == Pawn {
            bitboards[Piece::Pawn as usize + if turn {0} else {Piece::COUNT}].remove_piece(invert_input + 8);
            en_passant = true;
        }
    }
                            
    if piece == King && castle[turn as usize] && to_index == 6 {
        bitboards[Piece::Rook as usize + if !turn {0} else {Piece::COUNT}].move_piece(7, 5)
    }
    if piece == King && castle[turn as usize] && to_index == 2 {
        bitboards[Piece::Rook as usize + if !turn {0} else {Piece::COUNT}].move_piece(0, 3)
    }

    if bitboards[piece_index + if !turn {0} else {Piece::COUNT}].bits & opponent.bits != 0 {                                        
        if !en_passant {
            for j in 0..Piece::COUNT {
                if bitboards[j + if turn {0} else {Piece::COUNT}].get_bit(invert_input) == 1 {
                    bitboards[j + if turn {0} else {Piece::COUNT}].remove_piece(invert_input);
                    break;
                }
            }
        }
    }

    if piece == Pawn {
        *last_opponent_move = Some((invert_index(from_index), invert_input));
        if to_index > 55 {
            bitboards[Pawn  as usize + if !turn {0} else {Piece::COUNT}].remove_piece(to_index);
            bitboards[Queen as usize + if !turn {0} else {Piece::COUNT}].add_piece(to_index);
        }
    } else {
        *last_opponent_move = None;
    }
}


pub fn play_player(bitboards: &mut [Bitboard; Piece::COUNT*Color::COUNT], turn: bool, last_opponent_move: &mut Option<(usize, usize)>, castle: &mut [bool; Color::COUNT]) {
    let mut wrong = true;
    let (player, opponent) = get_player_and_opponent_bitboards(&bitboards, turn);
    let opponent = opponent.mirror();

    while wrong {
        let input = get_player_piece_input(turn);
        if let Some(index) = algebraic_to_index(&input, turn) {
            if player.get_bit(index) == 0 {
                println!("No pieces here!");
                continue;
            }
            
            if let Some(piece_index) = get_piece_index_to_move(&bitboards, turn, index) {
                let piece = Piece::usize_to_piece(piece_index);
                let moves = player.moves(index, opponent, piece, last_opponent_move, castle[turn as usize]);
                let move_input = get_player_move_input(turn, &moves);

                if let Some(move_index) = algebraic_to_index(&move_input, turn) {

                    if moves.contains(&move_index) {
                        update_game_state(bitboards, opponent, last_opponent_move, castle, turn, piece_index, index, move_index);
                    }
                    else {
                        print!("Invalid move!\n");
                        continue;
                    }
                }
                else {
                    print!("Invalid move!\n");
                    continue;
                }

            }
            else{
                print!("Invalid case!\n");
                continue;
            }
            
        } else {
            print!("Invalid case!\n");
            continue;
        }  

        wrong = !wrong;
    }

    //Check if king or rook has mooved
    for i in 0..Color::COUNT{
        if bitboards[(Piece::King as usize) + i*Piece::COUNT].get_bit(4) == 0 {castle[i] = false;}
        if bitboards[(Piece::Rook as usize) + i*Piece::COUNT].get_bit(0) == 0 {castle[i] = false;}
        if bitboards[(Piece::Rook as usize) + i*Piece::COUNT].get_bit(7) == 0 {castle[i] = false;}
    }
}

pub fn play_ai(bitboards: &mut [Bitboard; Piece::COUNT*Color::COUNT], turn: bool, last_opponent_move: &mut Option<(usize, usize)>, castle: &mut [bool; Color::COUNT], depth: usize, use_alpha_beta: bool) {
    if use_alpha_beta {
        play_alpha_beta(bitboards, turn, last_opponent_move, castle, depth);
    } else {
        play_minimax(bitboards, turn, last_opponent_move, castle, depth);
    }
}


fn play_alpha_beta(bitboards: &mut [Bitboard; Piece::COUNT*Color::COUNT], turn: bool, last_opponent_move: &mut Option<(usize, usize)>, castle: &mut [bool; Color::COUNT], depth: usize) {
    let mut best_move = None;
    let mut alpha = std::isize::MIN;
    let mut beta = std::isize::MAX;

    let (player, opponent) = get_player_and_opponent_bitboards(&bitboards, turn);
    let opponent = opponent.mirror();
    
    let mut moves_with_scores: Vec<(usize, usize, usize, isize)> = Vec::new();

    for i in ORDER {
        let piece = Piece::usize_to_piece(i);
        let player_piece_indices = bitboards[piece as usize + if !turn { 0 } else { Piece::COUNT }].get_indices();
        for &index in &player_piece_indices {
            let moves = player.moves(index, opponent, piece, last_opponent_move, castle[turn as usize]);
            for &move_index in &moves {
                let mut cloned_bitboards = bitboards.clone();
                let mut cloned_last_opponent_move = last_opponent_move.clone();
                let mut cloned_castle = castle.clone();
                update_game_state(&mut cloned_bitboards, opponent, &mut cloned_last_opponent_move, &mut cloned_castle, turn, i, index, move_index);

                let mut win = false;
                check_win_conditions(&mut cloned_bitboards, &mut win, false);

                let score;
                if win {score = King.value() + 1000;}
                else   {score = evaluate_board(&cloned_bitboards, !turn, cloned_last_opponent_move, cloned_castle);}
                
                let mut inserted = false;
                for j in 0..moves_with_scores.len() {
                    if score > moves_with_scores[j].3 {
                        moves_with_scores.insert(j, (i, index, move_index, score));
                        inserted = true;
                        break;
                    }
                }
                if !inserted {
                    moves_with_scores.push((i, index, move_index, score));
                }
            }
        }
    }

    for (piece_index, from_index, to_index, _) in moves_with_scores {
        let mut cloned_bitboards = bitboards.clone();
        let mut cloned_last_opponent_move = last_opponent_move.clone();
        let mut cloned_castle = castle.clone();
        update_game_state(&mut cloned_bitboards, opponent, &mut cloned_last_opponent_move, &mut cloned_castle, turn, piece_index, from_index, to_index);

        let score = alpha_beta(&mut cloned_bitboards, cloned_last_opponent_move, &mut cloned_castle, false, !turn, depth - 1, 2, alpha, beta);
        if score > alpha {
            alpha = score;
            best_move = Some((piece_index, from_index, to_index));
        }
        if beta <= alpha {
            break;
        }
    }

    if let Some((piece_index, from_index, to_index)) = best_move {
        update_game_state(bitboards, opponent, last_opponent_move, castle, turn, piece_index, from_index, to_index);
    }
}


fn alpha_beta(bitboards: &mut [Bitboard; Piece::COUNT*Color::COUNT], last_opponent_move: Option<(usize, usize)>, castle: &mut [bool; Color::COUNT], maximizing_player: bool, turn: bool, depth: usize, cur_depth: isize, mut alpha: isize, mut beta: isize) -> isize {
    if depth == 0 {
        let score = evaluate_board(&bitboards, !(maximizing_player^turn), last_opponent_move, *castle);
        //print!("{score}\n");
        //display_board(bitboards);

        return score;
    }


    let mut moves_with_scores: Vec<(usize, usize, usize, isize)> = Vec::new();
    let (player, opponent) = get_player_and_opponent_bitboards(&bitboards, turn);
    let opponent = opponent.mirror();

    for i in ORDER {
        let piece = Piece::usize_to_piece(i);
        let player_piece_indices = bitboards[piece as usize + if !turn { 0 } else { Piece::COUNT }].get_indices();
        for &index in &player_piece_indices {
            let moves = player.moves(index, opponent, piece, &last_opponent_move, castle[maximizing_player as usize]);
            for &move_index in &moves {
                let mut cloned_bitboards = bitboards.clone();
                let mut cloned_last_opponent_move = last_opponent_move.clone();
                let mut cloned_castle = castle.clone();
                update_game_state(&mut cloned_bitboards, opponent, &mut cloned_last_opponent_move, &mut cloned_castle, turn, i, index, move_index);

                let score = evaluate_board(&cloned_bitboards, !turn, cloned_last_opponent_move, cloned_castle);
                moves_with_scores.push((i, index, move_index, score));
            }
        }
    }

    if maximizing_player {
        moves_with_scores.sort_by(|(_, _, _, score1), (_, _, _, score2)| score2.cmp(score1));
    } else {
        moves_with_scores.sort_by(|(_, _, _, score1), (_, _, _, score2)| score1.cmp(score2));
    }

    if maximizing_player {
        let mut max_eval = std::isize::MIN;
        for (piece_index, from_index, to_index, _) in moves_with_scores {
            let mut cloned_bitboards = bitboards.clone();
            let mut cloned_last_opponent_move = last_opponent_move.clone();
            let mut cloned_castle = castle.clone();
            update_game_state(&mut cloned_bitboards, opponent, &mut cloned_last_opponent_move, &mut cloned_castle, turn, piece_index, from_index, to_index);

            let mut win = false;
            check_win_conditions(&mut cloned_bitboards, &mut win, false);

            if win {return King.value() + 1000/cur_depth;}

            let eval = alpha_beta(&mut cloned_bitboards, cloned_last_opponent_move, &mut cloned_castle, false, !turn, depth - 1, cur_depth+1, alpha, beta);
            max_eval = max_eval.max(eval);
            alpha = alpha.max(max_eval);
            if beta <= alpha {
                return max_eval;
            }
        }
        return max_eval;
    } else {
        let mut min_eval = std::isize::MAX;
        for (piece_index, from_index, to_index, _) in moves_with_scores {
            let mut cloned_bitboards = bitboards.clone();
            let mut cloned_last_opponent_move = last_opponent_move.clone();
            let mut cloned_castle = castle.clone();
            update_game_state(&mut cloned_bitboards, opponent, &mut cloned_last_opponent_move, &mut cloned_castle, turn, piece_index, from_index, to_index);

            let mut win = false;
            check_win_conditions(&mut cloned_bitboards, &mut win, false);

            if win {return -(King.value() + 1000/cur_depth);}

            let eval = alpha_beta(&mut cloned_bitboards, cloned_last_opponent_move, &mut cloned_castle, true, !turn, depth - 1, cur_depth+1, alpha, beta);
            min_eval = min_eval.min(eval);
            beta = beta.min(min_eval);
            if beta <= alpha {
                return min_eval;
            }
        }
        return min_eval;
    }
}


pub fn play_minimax(bitboards: &mut [Bitboard; Piece::COUNT*Color::COUNT], turn: bool, last_opponent_move: &mut Option<(usize, usize)>, castle: &mut [bool; Color::COUNT], depth: usize) {
    let mut best_score = {std::isize::MIN};
    let mut best_move = None;

    let (player, opponent) = get_player_and_opponent_bitboards(&bitboards, turn);
    let opponent = opponent.mirror();

    for i in 0..Piece::COUNT {
        let piece = Piece::usize_to_piece(i);
        let player_piece_indices = bitboards[piece as usize + if !turn{0} else {Piece::COUNT}].get_indices();
        for &index in &player_piece_indices {
            let moves = player.moves(index, opponent, piece, last_opponent_move, castle[turn as usize]);
            for &move_index in &moves {
                let mut cloned_bitboards = bitboards.clone();
                let mut cloned_last_opponent_move = last_opponent_move.clone();
                let mut cloned_castle = castle.clone();
                update_game_state(&mut cloned_bitboards, opponent, &mut cloned_last_opponent_move, &mut cloned_castle, turn, i, index, move_index);

                let score = minimax(&mut cloned_bitboards, cloned_last_opponent_move, cloned_castle, false, !turn, depth - 1);

                if score > best_score {
                    best_score = score;
                    best_move = Some((i, index, move_index));
                }

            }
        }
    }

    if let Some((piece_index, from_index, to_index)) = best_move {
        update_game_state(bitboards, opponent, last_opponent_move, castle, turn, piece_index, from_index, to_index);
    }
}

fn minimax(bitboards: &mut [Bitboard; Piece::COUNT*Color::COUNT], last_opponent_move: Option<(usize, usize)>, castle: [bool; Color::COUNT], maximizing_player: bool, turn: bool, depth: usize) -> isize {
    if depth == 0 {
        let eval = evaluate_board(&bitboards, !(maximizing_player^turn), last_opponent_move, castle);
        return eval;
    }

    let (player, opponent) = get_player_and_opponent_bitboards(&bitboards, turn);
    let opponent = opponent.mirror();

    if maximizing_player {
        let mut max_eval = std::isize::MIN;
        for i in 0..Piece::COUNT {
            let piece = Piece::usize_to_piece(i);
            let player_piece_indices = bitboards[piece as usize + if !turn{0} else {Piece::COUNT}].get_indices();
            for &index in &player_piece_indices {
                let moves = player.moves(index, opponent, piece, &last_opponent_move, castle[maximizing_player as usize]);
                for &move_index in &moves {
                    let mut cloned_bitboards = bitboards.clone();
                    let mut cloned_last_opponent_move = last_opponent_move.clone();
                    let mut cloned_castle = castle.clone();
                    update_game_state(&mut cloned_bitboards, opponent, &mut cloned_last_opponent_move, &mut cloned_castle, turn, i, index, move_index);



                    let eval = minimax(&mut cloned_bitboards, cloned_last_opponent_move, cloned_castle, !maximizing_player, !turn, depth - 1);
                    max_eval = max_eval.max(eval);
                }
            }
        }
        return max_eval;
    } else {
        let mut min_eval = std::isize::MAX;
        for i in 0..Piece::COUNT {
            let piece = Piece::usize_to_piece(i);
            let player_piece_indices = bitboards[piece as usize + if !turn{0} else {Piece::COUNT}].get_indices();
            for &index in &player_piece_indices {
                let moves = player.moves(index, opponent, piece, &last_opponent_move, castle[maximizing_player as usize]);
                for &move_index in &moves {
                    let mut cloned_bitboards = bitboards.clone();
                    let mut cloned_last_opponent_move = last_opponent_move.clone();
                    let mut cloned_castle = castle.clone();
                    update_game_state(&mut cloned_bitboards, opponent, &mut cloned_last_opponent_move, &mut cloned_castle, turn, i, index, move_index);

                    let eval = minimax(&mut cloned_bitboards, cloned_last_opponent_move, cloned_castle, !maximizing_player, !turn, depth - 1);
                    min_eval = min_eval.min(eval);
                }
            }
        }
        return min_eval;
    }
}

fn evaluate_board(bitboards: &[Bitboard; Piece::COUNT*Color::COUNT], maximizing_player: bool, last_opponent_move: Option<(usize, usize)>, castle: [bool; Color::COUNT]) -> isize {

    let (player, opponent) = get_player_and_opponent_bitboards(&bitboards, maximizing_player);
    let opponent = opponent.mirror();

    let mut white_score = 0;
    let mut black_score = 0;

    for i in 0..Piece::COUNT {
        let piece = Piece::usize_to_piece(i);
        white_score += bitboards[i].count_bits() as isize * piece.value();
        black_score += bitboards[i + Piece::COUNT].count_bits() as isize * piece.value();
    }

    for i in ORDER {
        let piece = Piece::usize_to_piece(i);
        let player_piece_indices = bitboards[piece as usize + if !maximizing_player { 0 } else { Piece::COUNT }].get_indices();
        for &index in &player_piece_indices {
            let moves = player.moves(index, opponent, piece, &last_opponent_move, castle[maximizing_player as usize]);
            white_score += moves.len() as isize * piece.value()/10;
        }

        let opponent_piece_indices = bitboards[piece as usize + if maximizing_player { 0 } else { Piece::COUNT }].get_indices();
        for &index in &opponent_piece_indices {
            let moves = opponent.moves(index, opponent, piece, &last_opponent_move, castle[!maximizing_player as usize]);
            black_score += moves.len() as isize * piece.value()/10;
        }
    }

    let material_score = if maximizing_player { black_score - white_score } else { white_score - black_score };

    let pawn_structure_score = 0;//bitboards.iter().map(|&bb| bb.evaluate_pawn_structure()).sum::<isize>();

    let total_score = material_score + pawn_structure_score;

    if maximizing_player {
        total_score
    } else {
        -total_score
    }
}


pub fn display_board(bitboards: &[Bitboard]) {
    println!("    A  B  C  D  E  F  G  H");
    println!("  +------------------------+");

    for row in (0..8).rev() {
        print!("{} |", row + 1);
        for col in 0..8 {
            let mut piece_found = false;
            for i in 0..Piece::COUNT {
                if bitboards[i].get_bit(col + row * 8) == 1 {
                    print!("{} {}{} {}", if row%2 == col%2 {WHITE_BG} else {BLACK_BG}, RED, MAP[i], RESET);
                    piece_found = true;
                    break;
                } else if bitboards[i + Piece::COUNT].mirror().get_bit(col + row * 8) == 1 {
                    print!("{} {}{} {}", if row%2 == col%2 {WHITE_BG} else {BLACK_BG}, BLUE, MAP[i], RESET);
                    piece_found = true;
                    break;
                }
            }
            if !piece_found {
                print!("{}   {}", if row%2 == col%2 {WHITE_BG} else {BLACK_BG}, RESET);
                
            }
        }
        println!("|");
    }

    println!("  +------------------------+");
}