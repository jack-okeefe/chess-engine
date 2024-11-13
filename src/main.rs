use std::env;

mod board;
mod move_generation;
mod pieces;
mod position;
mod utils;

use board::ask_for_piece_selection;
use board::print_board;
use move_generation::generate_moves;
use pieces::Piece;
use position::get_starting_position;
use position::Position;
use utils::index_to_bitboard;
use utils::{algebraic_to_index, bit_scan, index_to_algebraic};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut position = get_starting_position();

    let square = index_to_bitboard(&algebraic_to_index(&"h4".to_string()).unwrap());
    position.insert_piece_at_square(&Piece::WhiteBishop, &square);

    let square2 = index_to_bitboard(&algebraic_to_index(&"b4".to_string()).unwrap());
    position.insert_piece_at_square(&Piece::WhiteKnight, &square2);

    let square3 = index_to_bitboard(&algebraic_to_index(&"e4".to_string()).unwrap());
    position.insert_piece_at_square(&Piece::WhiteBishop, &square3);

    print_board(&position, &0, &0);
    ask_for_piece_selection(&mut position);
}
