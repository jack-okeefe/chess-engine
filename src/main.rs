use std::collections::HashMap;
use std::env;
use std::io;

mod board;
mod moves;
mod pieces;
mod position;
mod utils;

use board::print_board;
use moves::generate_moves;
use pieces::Piece;
use position::get_starting_position;
use position::Position;
use utils::index_to_bitboard;
use utils::{algebraic_to_index, bit_scan, index_to_algebraic};

pub fn get_input(position: &Position) {
    let mut input: String;

    loop {
        println!("Request a square");
        input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().to_string();

        println!("You wrote {input}");

        match algebraic_to_index(input.clone()) {
            Ok(index) => {
                let square: u64 = 1 << index;
                let moves: u64 = generate_moves(position, &square).clone();
                let indicies = bit_scan(&moves);
                print_board(position, Some(indicies.clone()));
                for i in indicies.clone() {
                    let algebraic = index_to_algebraic(&i).unwrap();
                    print!("{algebraic}, ")
                }
                println!();
                break;
            }
            Err(e) => {
                print_board(position, None);
                println!("{e}");
            }
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut position = get_starting_position();

    let square = &index_to_bitboard(algebraic_to_index("e5".to_string()).unwrap());
    position.insert_piece_at_square(&Piece::WhitePawn, square);

    print_board(&position, None);
    get_input(&position);
}
