use std::collections::{HashSet};

use crate::pieces::Piece;
use crate::position::Position;
use crate::utils::{algebraic_to_index, bit_scan};

pub const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const RANKS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

pub fn print_board(position: &Position, highlighted_indicies: Option<HashSet<usize>>) {
    let mut board = [" "; 64];
    let highlighted = match highlighted_indicies {
        Some(indicies) => indicies,
        None => HashSet::new(),
    };

    for piece in Piece::iter() {
        let bitboard = &position.get_bitboard(piece);
        fill_board(&mut board, bitboard, piece);
    }

    // clear terminal using special char
    // https://rosettacode.org/wiki/Terminal_control/Clear_the_screen#Rust
    print!("{}[2J", 27 as char);

    // offset to get file names aligned
    print!("\n     ");
    for file in FILES {
        print!(" {file} ")
    }
    println!();

    for rank in RANKS.iter().rev() {
        print!("\n{rank}    ");
        for file in FILES {
            let algebraic: String = format!("{}{}", file, rank);
            let index = algebraic_to_index(algebraic).unwrap();
            let piece = board[index];
            if highlighted.contains(&index) {
                print!("\x1b[93m[{piece}]\x1b[0m");
            } else {
                print!("[{piece}]");
            }
        }
    }
    println!();
    println!();
}

pub fn fill_board(board: &mut [&str; 64], bitboard: &u64, piece: &Piece) {
    let indicies_to_fill = bit_scan(bitboard);
    for index in indicies_to_fill {
        board[index] = piece.str();
    }
}

