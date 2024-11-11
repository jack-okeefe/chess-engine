use std::collections::HashMap;
use std::env;
use std::io;

mod board;
mod moves;
mod pieces;
mod utils;

use board::print_board;
use moves::generate_moves;
use pieces::Piece;
use pieces::{
    BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP,
    WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
};
use utils::{algebraic_to_index, bit_scan, index_to_algebraic};

pub fn get_input(position: &HashMap<Piece, u64>) {
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
                print_board(&position, None);
                println!("{e}");
            }
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut position: HashMap<Piece, u64> = HashMap::new();
    // 0b0000000000000000000000000000000000000000000000000000000000000000
    position.insert(
        WHITE_PAWN,
        0b0000000000000000000000000000000000000000000000001111111100000000,
    );
    position.insert(
        WHITE_KNIGHT,
        0b0000000000000000000000000000000000000000000000000000000001000010,
    );
    position.insert(
        WHITE_BISHOP,
        0b0000000000000000000000000000000000000000000000000000000000100100,
    );
    position.insert(
        WHITE_ROOK,
        0b0000000000000000000000000000010000000000000000000000000010000001,
    );
    position.insert(
        WHITE_QUEEN,
        0b0000000000000000000000000000000000000000000000000000000000001000,
    );
    position.insert(
        WHITE_KING,
        0b0000000000000000000000000000000000000000000000000000000000010000,
    );
    position.insert(
        BLACK_PAWN,
        0b0000000011111111000000000000000000000000000000000000000000000000,
    );
    position.insert(
        BLACK_KNIGHT,
        0b0100001000000000000000000000000000000000000000000000000000000000,
    );
    position.insert(
        BLACK_BISHOP,
        0b0010010000000000000000000000000000000000000000000000000000000000,
    );
    position.insert(
        BLACK_ROOK,
        0b1000000100000000000000000000000000000000000000000000000000000000,
    );
    position.insert(
        BLACK_QUEEN,
        0b0000100000000000000000000000000000000000000000000000000000000000,
    );
    position.insert(
        BLACK_KING,
        0b0001000000000000000000000000000000000000000000000000000000000000,
    );

    print_board(&position, None);

    // println!("moves: {:#064b}", moves);

    get_input(&position);
}
