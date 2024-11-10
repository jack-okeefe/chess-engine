use core::hash;
use std::cmp;
use std::collections::btree_map::Range;
use std::env;
use std::fmt::Binary;
use std::io::Write;
use std::ops::Index;
use std::string::ParseError;
use std::{any::type_name, io};
use std::collections::HashSet;

enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

enum Colour {
    White,
    Black,
}

struct Piece {
    piece_type: PieceType,
    colour: Colour,
    str: &'static str,
}
struct Position {
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
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    const FILES: std::ops::RangeInclusive<char> = 'a'..='h';
    const RANKS: std::ops::RangeInclusive<char> = '1'..='8';

    let white_pawn: Piece = Piece {
        piece_type: PieceType::Pawn,
        colour: Colour::White,
        str: "P",
    };
    let white_knight: Piece = Piece {
        piece_type: PieceType::Knight,
        colour: Colour::White,
        str: "N",
    };
    let white_bishop: Piece = Piece {
        piece_type: PieceType::Bishop,
        colour: Colour::White,
        str: "B",
    };
    let white_rook: Piece = Piece {
        piece_type: PieceType::Rook,
        colour: Colour::White,
        str: "R",
    };
    let white_queen: Piece = Piece {
        piece_type: PieceType::Queen,
        colour: Colour::White,
        str: "Q",
    };
    let white_king: Piece = Piece {
        piece_type: PieceType::King,
        colour: Colour::White,
        str: "K",
    };
    let black_pawn: Piece = Piece {
        piece_type: PieceType::Pawn,
        colour: Colour::Black,
        str: "p",
    };
    let black_knight: Piece = Piece {
        piece_type: PieceType::Knight,
        colour: Colour::Black,
        str: "n",
    };
    let black_bishop: Piece = Piece {
        piece_type: PieceType::Bishop,
        colour: Colour::Black,
        str: "b",
    };
    let black_rook: Piece = Piece {
        piece_type: PieceType::Rook,
        colour: Colour::Black,
        str: "r",
    };
    let black_queen: Piece = Piece {
        piece_type: PieceType::Queen,
        colour: Colour::Black,
        str: "q",
    };
    let black_king: Piece = Piece {
        piece_type: PieceType::King,
        colour: Colour::Black,
        str: "k",
    };
    // let pieces = vec![white_pawn, white_knight, white_bishop, white_rook, white_queen, white_king, black_pawn, black_knight, black_bishop, black_rook, black_queen, black_king];

    let starting_position: Position = Position {
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
    };
    

    fn algebraic_to_index(algebraic: String) -> Result<usize, &'static str> {
        if algebraic.len() != 2 {
            return Err("Input must be exactly 2 characters long.");
        }

        let file: char = algebraic.chars().nth(0).unwrap();
        let rank: char = algebraic.chars().nth(1).unwrap();

        let file_index = if FILES.contains(&file) {
            file as u8 - b'a'
        } else {
            return Err("Invalid file. Must be between 'a' and 'h'.");
        };

        let rank_index = if RANKS.contains(&rank) {
            rank as u8 - b'1'
        } else {
            return Err("Invalid rank. Must be between '1' and '8'.");
        };

        // Calculate the final index: (rank_index * 8 + file_index)
        let index = ((rank_index * 8) + file_index) as usize;

        Ok(index)
    }

    fn display_board(board: [&str; 64]) {
        // clear terminal using special char
        // https://rosettacode.org/wiki/Terminal_control/Clear_the_screen#Rust
        print!("{}[2J", 27 as char);

        // offset to get file names aligned
        print!("\n     ");
        for file in FILES {
            print!(" {file} ")
        }
        println!();

        for rank in RANKS.rev() {
            print!("\n{rank}    ");
            for file in FILES {
                let algebraic: String = format!("{}{}", file, rank);
                let index = algebraic_to_index(algebraic).unwrap();
                let piece = board[index];
                print!("[{piece}]");
            }
        }
        println!();
        println!();

    }

    // fn get_input(board: [&str; 64]) {
    //     let mut input: String = String::new();

    //     loop {
    //         println!("Request a square");
    //         input = String::new();

    //         io::stdin()
    //             .read_line(&mut input)
    //             .expect("Failed to read input");
    //         let input = input.trim().to_string();

    //         println!("You wrote {input}");

    //         match algebraic_to_index(input.clone()) {
    //             Ok(index) => {
    //                 display_board(board);
    //                 println!("Piece at {} is {}", input, board[index]);
    //                 break;
    //             }
    //             Err(e) => {
    //                 display_board(board);
    //                 println!("{e}");
    //             }
    //         }
    //     }
    // }

    
    fn bit_scan(bitboard: u64) -> HashSet<usize> {
        let mut mask = bitboard.clone();
        let mut indicies = HashSet::new();
        let mut index: usize = 0;
        while mask > 0 {

            if mask.trailing_zeros() == 0 {
                indicies.insert(index);
            }
            let shift_by = cmp::max(1, mask.trailing_zeros());
            mask = mask >> shift_by;
            index += shift_by as usize;
        }
        indicies
    }
    let mut board = [" "; 64];

    fn fill_board(board: &mut [&str; 64], bitboard: u64, piece: Piece) {
        let indicies_to_fill = bit_scan(bitboard);
        for index in indicies_to_fill {
            board[index] = piece.str;
        }

    }
    
    fill_board(&mut board, starting_position.white_pawn, white_pawn);
    fill_board(&mut board, starting_position.white_knight, white_knight);
    fill_board(&mut board, starting_position.white_bishop, white_bishop);
    fill_board(&mut board, starting_position.white_rook, white_rook);
    fill_board(&mut board, starting_position.white_queen, white_queen);
    fill_board(&mut board, starting_position.white_king, white_king);
    fill_board(&mut board, starting_position.black_pawn, black_pawn);
    fill_board(&mut board, starting_position.black_knight, black_knight);
    fill_board(&mut board, starting_position.black_bishop, black_bishop);
    fill_board(&mut board, starting_position.black_rook, black_rook);
    fill_board(&mut board, starting_position.black_queen, black_queen);
    fill_board(&mut board, starting_position.black_king, black_king);
    display_board(board);

    // get_input(board);

    // find_piece(board, file, rank);
}
