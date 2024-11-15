use std::collections::HashSet;
use std::fmt::format;
use std::hash::Hash;
use std::{default, io};

use crate::move_generation::generate_moves;
use crate::pieces::{Colour, Piece};
use crate::position::{Position, DARK_SQUARES, LIGHT_SQUARES};
use crate::utils::{algebraic_to_index, bit_scan, index_to_bitboard};

pub const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const RANKS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

enum SquareShade {
    Dark,
    Light,
}
enum SquareHighlight {
    Selected,
    LastMoved,
    Default,
}

fn determine_square_shade(square: &u64) -> SquareShade {
    if square & DARK_SQUARES != 0 {
        SquareShade::Dark
    } else {
        SquareShade::Light
    }
}

fn determine_square_highlight(
    square: &u64,
    selected_square: &u64,
    last_moved_squares: &u64,
) -> SquareHighlight {
    if square & selected_square != 0 {
        SquareHighlight::Selected
    } else if square & last_moved_squares != 0 {
        SquareHighlight::LastMoved
    } else {
        SquareHighlight::Default
    }
}

fn determine_square_colour(
    square: &u64,
    selected_square: &u64,
    last_moved_squares: &u64,
) -> &'static str {
    let square_highlight: SquareHighlight =
        determine_square_highlight(square, selected_square, last_moved_squares);
    let square_shade: SquareShade = determine_square_shade(square);

    match (square_highlight, square_shade) {
        (SquareHighlight::Selected, SquareShade::Dark) => "\x1b[48;2;100;110;64m",
        (SquareHighlight::Selected, SquareShade::Light) => "\x1b[48;2;130;151;105m",
        (SquareHighlight::LastMoved, SquareShade::Dark) => "\x1b[48;2;170;162;58m",
        (SquareHighlight::LastMoved, SquareShade::Light) => "\x1b[48;2;205;210;106m",
        (SquareHighlight::Default, SquareShade::Dark) => "\x1b[48;2;181;136;99m",
        (SquareHighlight::Default, SquareShade::Light) => "\x1b[48;2;240;217;181m",
    }
}

fn determine_piece_str(piece_str: &str, square: &u64, move_squares: &u64) -> String {
    let is_a_move_square = square & move_squares != 0;
    let is_piece_on_square = piece_str != " ";

    match (is_a_move_square, is_piece_on_square) {
        (true, true) => format!("\x1b[38;2;100;110;64m[{piece_str}]\x1b[0m"),
        (true, false) => "\x1b[38;2;100;110;64m ● \x1b[0m".to_string(),
        (false, true) => format!(" {piece_str} "),
        (false, false) => "   ".to_string(),
    }
}

pub fn print_board(position: &Position, selected_square: &u64, move_squares: &u64) {
    let mut board = [" "; 64];

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
            let index = algebraic_to_index(&algebraic).unwrap();
            let current_square = index_to_bitboard(&index);

            // second answer to this s/o
            // https://stackoverflow.com/questions/69981449/how-do-i-print-colored-text-to-the-terminal-in-rust
            // https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
            let ansi_closing = "\x1b[0m";

            let bg_colour_ansi = determine_square_colour(
                &current_square,
                selected_square,
                &position.last_moved_squares,
            );
            let piece_str = determine_piece_str(board[index], &current_square, move_squares);

            let display_string = format!("{bg_colour_ansi}{piece_str}{ansi_closing}");
            print!("{}", display_string);
        }
    }
    println!();
    println!();
    let to_move = match position.turn {
        Colour::White => "WHITE",
        Colour::Black => "BLACK",
    };
    println!("           {to_move} TO MOVE");
    println!();
}

pub fn fill_board(board: &mut [&str; 64], bitboard: &u64, piece: &Piece) {
    let indicies_to_fill = bit_scan(bitboard);
    for index in indicies_to_fill {
        board[index] = piece.str();
    }
}

pub fn get_input(prompt: &str) -> String {
    let mut input: String;
    println!("{}", prompt);
    input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

pub fn ask_for_piece_selection(position: &mut Position) {
    loop {
        let input = get_input("Select a piece to move");

        match algebraic_to_index(&input) {
            Ok(index) => match position.get_piece_with_colour_at(&index_to_bitboard(&index), &position.turn) {
                Some(piece) => {
                    let square: u64 = 1 << index;
                    let moves: u64 = generate_moves(position, &square).clone();

                    print_board(position, &square, &moves);
                    ask_for_move(position, &square, &moves);

                    break;
                }
                None => {
                    print_board(position, &0, &0);
                    println!("No piece on square {}", &input);
                }
            },
            Err(e) => {
                print_board(position, &0, &0);
                println!("{e}");
            }
        }
    }
}

pub fn ask_for_move(position: &mut Position, root_square: &u64, valid_moves: &u64) {
    loop {
        let input = get_input("Which square do you want to move it to? ('q' to cancel)");

        if input == "q" {
            print_board(position, &0, &0);
            ask_for_piece_selection(position);
        }

        match algebraic_to_index(&input) {
            Ok(index) => {
                let square: u64 = index_to_bitboard(&index);
                if square & valid_moves != 0 {
                    position.move_piece(root_square, &square);

                    print_board(position, &0, &0);
                    ask_for_piece_selection(position);

                    break;
                } else {
                    print_board(position, root_square, valid_moves);
                    println!("'{}' is not a valid move.", &input);
                }
            }
            Err(e) => {
                print_board(position, root_square, valid_moves);
            }
        }
    }
}
