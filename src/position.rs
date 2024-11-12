use std::collections::HashMap;

use crate::pieces::{Piece, Class, Colour};

pub struct Position {
    white_pawn: u64,
    white_knight: u64,
    white_bishop: u64,
    white_rook: u64,
    white_queen: u64,
    white_king: u64,
    black_pawn: u64,
    black_knight: u64,
    black_bishop: u64,
    black_rook: u64,
    black_queen: u64,
    black_king: u64,
    en_passant_square: u64,
}

impl Position {
    pub fn get_bitboard(&self, piece: &Piece) -> u64 {
        match piece {
            Piece::WhitePawn => self.white_pawn,
            Piece::WhiteKnight => self.white_knight,
            Piece::WhiteBishop => self.white_bishop,
            Piece::WhiteRook => self.white_rook,
            Piece::WhiteQueen => self.white_queen,
            Piece::WhiteKing => self.white_king,
            Piece::BlackPawn => self.black_pawn,
            Piece::BlackKnight => self.black_knight,
            Piece::BlackBishop => self.black_bishop,
            Piece::BlackRook => self.black_rook,
            Piece::BlackQueen => self.black_queen,
            Piece::BlackKing => self.black_king,
        }

    }
    pub fn get_bitboard_mut(&mut self, piece: &Piece) -> &mut u64 {
        match piece {
            Piece::WhitePawn => &mut self.white_pawn,
            Piece::WhiteKnight => &mut self.white_knight,
            Piece::WhiteBishop => &mut self.white_bishop,
            Piece::WhiteRook => &mut self.white_rook,
            Piece::WhiteQueen => &mut self.white_queen,
            Piece::WhiteKing => &mut self.white_king,
            Piece::BlackPawn => &mut self.black_pawn,
            Piece::BlackKnight => &mut self.black_knight,
            Piece::BlackBishop => &mut self.black_bishop,
            Piece::BlackRook => &mut self.black_rook,
            Piece::BlackQueen => &mut self.black_queen,
            Piece::BlackKing => &mut self.black_king,
        }

    }

    pub fn get_occupancy(&self) -> u64 {
        return self.white_pawn
                    | self.white_knight
                    | self.white_bishop
                    | self.white_rook
                    | self.white_queen
                    | self.white_king
                    | self.black_pawn
                    | self.black_knight
                    | self.black_bishop
                    | self.black_rook
                    | self.black_queen
                    | self.black_king;
    }

    pub fn get_colour_occupancy(&self, colour: &Colour) -> u64 {
        match colour {
            Colour::White => {
                return self.white_pawn
                    | self.white_knight
                    | self.white_bishop
                    | self.white_rook
                    | self.white_queen
                    | self.white_king;
            }
            Colour::Black => {
                return self.black_pawn
                    | self.black_knight
                    | self.black_bishop
                    | self.black_rook
                    | self.black_queen
                    | self.black_king;
            }
        }
    }

    pub fn get_piece_at_index(&self, square: &u64) -> Option<&Piece> {
        for piece in Piece::iter() {
            if self.get_bitboard(piece) & square != 0 {
                return Some(piece)
            }
        }
        None
    }

    pub fn insert_piece_at_square(&mut self, piece: &Piece, square: &u64) {
        let bitboard = self.get_bitboard_mut(piece);
        *bitboard |= square;
    }
}




pub const FILE_A: u64 = 0b0000000100000001000000010000000100000001000000010000000100000001;
pub const FILE_B: u64 = 0b0000001000000010000000100000001000000010000000100000001000000010;
pub const FILE_G: u64 = 0b0100000001000000010000000100000001000000010000000100000001000000;
pub const FILE_H: u64 = 0b1000000010000000100000001000000010000000100000001000000010000000;
pub const FILES_AB: u64 = 0b0000001100000011000000110000001100000011000000110000001100000011;
pub const FILES_GH: u64 = 0b1100000011000000110000001100000011000000110000001100000011000000;
pub const RANK_1: u64 = 0b0000000000000000000000000000000000000000000000000000000011111111;
pub const RANK_2: u64 = 0b0000000000000000000000000000000000000000000000001111111100000000;
pub const RANK_7: u64 = 0b0000000011111111000000000000000000000000000000000000000000000000;
pub const RANK_8: u64 = 0b1111111100000000000000000000000000000000000000000000000000000000;


pub fn get_starting_position() -> Position {
    let starting_position = Position {
        white_pawn: 0b0000000000000000000000000000000000000000000000001111111100000000,
        white_knight: 0b0000000000000000000000000000000000000000000000000000000001000010,
        white_bishop: 0b0000000000000000000000000000000000000000000000000000000000100100,
        white_rook: 0b0000000000000000000000000000000000000000000000000000000010000001,
        white_queen: 0b0000000000000000000000000000000000000000000000000000000000001000,
        white_king: 0b0000000000000000000000000000000000000000000000000000000000010000,
        black_pawn: 0b0000000011111111000000000000000000000000000000000000000000000000,
        black_knight: 0b0100001000000000000000000000000000000000000000000000000000000000,
        black_bishop: 0b0010010000000000000000000000000000000000000000000000000000000000,
        black_rook: 0b1000000100000000000000000000000000000000000000000000000000000000,
        black_queen: 0b0000100000000000000000000000000000000000000000000000000000000000,
        black_king: 0b0001000000000000000000000000000000000000000000000000000000000000,
        en_passant_square: 0b0,
    };
    starting_position
}

// pub fn get_starting_position() -> HashMap<Piece, u64> {
//     let mut starting_position: HashMap<Piece, u64> = HashMap::new();
//     // all 0's to copy/paste
//     // 0b0000000000000000000000000000000000000000000000000000000000000000
//     starting_position.insert(
//         WHITE_PAWN,
//         0b0000000000000000000000000000000000000000000000001111111100000000,
//     );
//     starting_position.insert(
//         WHITE_KNIGHT,
//         0b0000000000000000000000000000000000000000000000000000000001000010,
//     );
//     starting_position.insert(
//         WHITE_BISHOP,
//         0b0000000000000000000000000000000000000000000000000000000000100100,
//     );
//     starting_position.insert(
//         WHITE_ROOK,
//         0b0000000000000000000000000000000000000000000000000000000010000001,
//     );
//     starting_position.insert(
//         WHITE_QUEEN,
//         0b0000000000000000000000000000000000000000000000000000000000001000,
//     );
//     starting_position.insert(
//         WHITE_KING,
//         0b0000000000000000000000000000000000000000000000000000000000010000,
//     );
//     starting_position.insert(
//         BLACK_PAWN,
//         0b0000000011111111000000000000000000000000000000000000000000000000,
//     );
//     starting_position.insert(
//         BLACK_KNIGHT,
//         0b0100001000000000000000000000000000000000000000000000000000000000,
//     );
//     starting_position.insert(
//         BLACK_BISHOP,
//         0b0010010000000000000000000000000000000000000000000000000000000000,
//     );
//     starting_position.insert(
//         BLACK_ROOK,
//         0b1000000100000000000000000000000000000000000000000000000000000000,
//     );
//     starting_position.insert(
//         BLACK_QUEEN,
//         0b0000100000000000000000000000000000000000000000000000000000000000,
//     );
//     starting_position.insert(
//         BLACK_KING,
//         0b0001000000000000000000000000000000000000000000000000000000000000,
//     );
//     starting_position
// }


