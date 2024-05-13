use crate::pieces::Piece;
use Piece::*;

#[derive(Debug, Copy, Clone)]
pub struct Bitboard {
    pub bits : u64,
}

// Define file masks for evaluating pawn structure
const FILE_MASKS: [u64; 8] = [
    0x0101010101010101, // File A
    0x0202020202020202, // File B
    0x0404040404040404, // File C
    0x0808080808080808, // File D
    0x1010101010101010, // File E
    0x2020202020202020, // File F
    0x4040404040404040, // File G
    0x8080808080808080, // File H
];

impl Bitboard {
    pub fn get_bit(&self, index: usize) -> u64 {
        let mask = 1 << index;
        (self.bits & mask) >> index
    }

    pub fn add_piece(&mut self, index: usize) {
        let mask = 1 << index;
        self.bits |= mask;
    }

    pub fn remove_piece(&mut self, index: usize) {
        let mask = 1 << index;
        self.bits &= !mask;
    }

    pub fn move_piece(&mut self, from_index: usize, to_index: usize) {
        self.remove_piece(from_index);
        self.add_piece(to_index);
    }

    pub fn mirror(&self) -> Bitboard {
        let mut mirrored_bits = 0;
        for row in 0..8 {
            let row_bits = (self.bits >> (row * 8)) & 0xFF;
            mirrored_bits |= (row_bits as u64) << ((7 - row) * 8);
        }
        Bitboard { bits: mirrored_bits }
    }

    pub fn combine_bitboards(bitboards: &[Bitboard]) -> Bitboard {
        let mut combined_bits = 0;
        for &bitboard in bitboards {
            combined_bits |= bitboard.bits;
        }
        Bitboard { bits: combined_bits }
    }

    pub fn combine_bitboard(&mut self, other: Bitboard) {
        self.bits |= other.bits;
    }

    pub fn get_indices(&self) -> Vec<usize> {
        let mut indices = Vec::new();
        let mut tmp: Bitboard = Bitboard{bits:self.bits};
        let mut index = 0;

        while index < 64 && tmp.bits != 0 {
            if self.get_bit(index) == 1 {
                indices.push(index);
                tmp.bits &= tmp.bits-1;
            }
            index += 1;
        }
        indices
    }

    pub fn count_bits(&self) -> u32 {
        self.bits.count_ones()
    }

    pub fn moves(&self, index : usize, opponent: Bitboard, piece : Piece, last_opponent_move : &Option<(usize, usize)>, castle : bool) -> Vec<usize> {
        match piece {
            Pawn   => self.pawn_moves  (index, opponent, last_opponent_move),
            Knight => self.knight_moves(index),
            Bishop => self.bishop_moves(index, opponent),
            Rook   => self.rook_moves  (index, opponent),
            Queen  => self.queen_moves (index, opponent),
            King   => self.king_moves  (index, opponent, castle),
        }
    }

    pub fn pawn_moves(&self, index: usize, opponent: Bitboard, last_opponent_move : &Option<(usize, usize)>) -> Vec<usize> {
        let mut moves = Vec::new();
        let mut forward_one = index + 8;
        let mut forward_two = index + 16;
        let mut capture_left = index + 7;
        let mut capture_right = index + 9;

        if let Some((from, to)) = *last_opponent_move {
            // En passant
            if (to >= from && to - from == 16) || (to < from && from - to == 16) {
                if index + 1 == to || index - 1 == to {
                    if self.get_bit(to) == 0 {
                        moves.push(to + 8);
                    }
                }
            }
        }

        
        if self.get_bit(forward_one) == 0 && opponent.get_bit(forward_one) == 0 {
            moves.push(forward_one);
            // Check if the pawn is in its starting position and can move two squares forward
            if index < 16 && self.get_bit(forward_two) == 0 && opponent.get_bit(forward_two) == 0{
                moves.push(forward_two);
            }
        }

        // Check if the pawn can capture diagonally to the left
        if index % 8 != 0 && opponent.get_bit(capture_left) == 1 {
            moves.push(capture_left);
        }

        // Check if the pawn can capture diagonally to the right
        if index % 8 != 7 && opponent.get_bit(capture_right) == 1 {
            moves.push(capture_right);
        }

        moves
    }

    pub fn knight_moves(&self, index: usize) -> Vec<usize> {
        let mut moves = Vec::new();
        let offsets = [-17, -15, -10, -6, 6, 10, 15, 17];

        for &offset in &offsets {
            let dest = (index as isize + offset) as usize;
            let dx = (index % 8) as isize - (dest % 8) as isize;
            if dest < 64 && dx.abs() <= 2 && self.get_bit(dest) == 0{
                moves.push(dest);
            }
        }

        moves
    }

    pub fn bishop_moves(&self, index: usize, opponent: Bitboard) -> Vec<usize> {
        let mut moves = Vec::new();
        let directions = [-9, -7, 7, 9];
    
        for &dir in &directions {
            let mut dest = (index as isize) + dir;
            while dest >= 0 && dest < 64 {

                let src_col  = index % 8;
                let dest_col = dest as usize % 8;
                let src_lin  = index / 8;
                let dest_lin = dest as usize / 8;

                if ((dest_col >= src_col && (dir == 9 || dir == -7)) || (dest_col <= src_col && (dir == -9 || dir == 7))) && src_lin != dest_lin {
                    if self.get_bit(dest as usize) == 1 {
                        break;
                    }
                    moves.push(dest as usize);
                    if opponent.get_bit(dest as usize) == 1 {
                        break;
                    }
                } else {
                    break;
                }
                dest += dir;
            }
        }
    
        moves
    }
    
    pub fn rook_moves(&self, index: usize, opponent: Bitboard) -> Vec<usize> {
        let mut moves = Vec::new();
        let directions = [-8, -1, 1, 8];
    
        for &dir in &directions {
            let mut dest = (index as isize) + dir;
            while dest >= 0 && dest < 64 {
                let src_col = index % 8;
                let dest_col = dest as usize % 8;
                if (dest_col > src_col && dir == 1) || (dest_col < src_col && dir == -1) || (dir.abs() > 1) {
                    if self.get_bit(dest as usize) == 1 {
                        break;
                    }
                    moves.push(dest as usize);
                    if opponent.get_bit(dest as usize) == 1 {
                        break;
                    }
                } else {
                    break;
                }
                dest += dir;
            }
        }
    
        moves
    }
    

    pub fn queen_moves(&self, index: usize, opponent: Bitboard) -> Vec<usize> {
        let mut moves = self.bishop_moves(index, opponent);
        moves.extend(self.rook_moves(index, opponent));
        moves
    }

    pub fn king_moves(&self, index: usize, opponent: Bitboard, castle : bool) -> Vec<usize> {
        let mut moves = Vec::new();
        let offsets = [-9, -8, -7, -1, 1, 7, 8, 9];

        let castle_right: u64 = 0b0000000000000000000000000000000000000000000000000000000001100000;
        let castle_left : u64 = 0b0000000000000000000000000000000000000000000000000000000000001110;

        if castle && Bitboard::combine_bitboards(&[*self, opponent]).bits & castle_right == 0 {
            moves.push(6);
        } 
        
        if castle && Bitboard::combine_bitboards(&[*self, opponent]).bits & castle_left == 0 {
            moves.push(2);
        }

        for &offset in &offsets {
            let dest = (index as isize + offset) as usize;
            let dx = (index % 8) as isize - (dest % 8) as isize;
            if dest < 64 && dx.abs() <= 1 && self.get_bit(dest) == 0 {
                moves.push(dest);
            }
        }

        moves
    }

    pub fn evaluate_pawn_structure(&self) -> isize {
        let mut score = 0;

        // Evaluate pawn islands
        let mut previous_file_has_pawn = false;
        let mut current_file_has_pawn:bool;
        let mut pawn_islands = 0;

        for file in 0..8 {
            current_file_has_pawn = (self.bits & FILE_MASKS[file]) != 0;
            if current_file_has_pawn && !previous_file_has_pawn {
                pawn_islands += 1;
            }
            previous_file_has_pawn = current_file_has_pawn;
        }
        score -= 20 * pawn_islands;

        // Evaluate doubled pawns
        for rank in 0..8 {
            let mut file_with_pawn = 0;
            for file in 0..8 {
                if (self.bits & (1 << (file + rank * 8))) != 0 {
                    file_with_pawn += 1;
                }
            }
            if file_with_pawn > 1 {
                score -= 10;
            }
        }

        // Evaluate isolated pawns
        for file in 0..8 {
            let left_adjacent_file = if file > 0 { file - 1 } else { file };
            let right_adjacent_file = if file < 7 { file + 1 } else { file };
            let left_mask = FILE_MASKS[left_adjacent_file];
            let right_mask = FILE_MASKS[right_adjacent_file];
            let file_mask = FILE_MASKS[file];

            let isolated_pawn_mask = file_mask & !(left_mask | right_mask);
            if (self.bits & isolated_pawn_mask) != 0 {
                score -= 20;
            }
        }

        // Evaluate pawn chains
        let mut current_chain_length = 0;
        let mut total_chain_length = 0;
        for file in 0..8 {
            for rank in 0..8 {
                let square_index = file + rank * 8;
                if (self.bits & (1 << square_index)) != 0 {
                    current_chain_length += 1;
                } else if current_chain_length > 0 {
                    total_chain_length += current_chain_length * current_chain_length;
                    current_chain_length = 0;
                }
            }
            if current_chain_length > 0 {
                total_chain_length += current_chain_length * current_chain_length;
                current_chain_length = 0;
            }
        }
        score += total_chain_length;

        score
    }
}