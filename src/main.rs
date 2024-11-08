use std::collections::btree_map::Range;
use std::env;
use std::ops::Index;
use std::string::ParseError;
use std::{any::type_name, io};
use std::io::Write;


enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

enum Colour {
    White,
    Black
}

struct Piece {
    piece_type: PieceType,
    colour: Colour,
    str: String,
    int: u8,
}


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    const FILES: std::ops::RangeInclusive<char> = 'a'..='h';
    const RANKS: std::ops::RangeInclusive<char> = '1'..='8';

    let white_pawn: Piece = Piece {
        piece_type: PieceType::Pawn,
        colour: Colour::White,
        str: "P".to_string(),
        int: 0
    };
    let white_knight: Piece = Piece {
        piece_type: PieceType::Knight,
        colour: Colour::White,
        str: "N".to_string(),
        int: 1
    };
    let white_bishop: Piece = Piece {
        piece_type: PieceType::Bishop,
        colour: Colour::White,
        str: "B".to_string(),
        int: 2
    };
    let white_rook: Piece = Piece {
        piece_type: PieceType::Rook,
        colour: Colour::White,
        str: "R".to_string(),
        int: 3
    };
    let white_queen: Piece = Piece {
        piece_type: PieceType::Queen,
        colour: Colour::White,
        str: "Q".to_string(),
        int: 4
    };
    let white_king: Piece = Piece {
        piece_type: PieceType::King,
        colour: Colour::White,
        str: "K".to_string(),
        int: 5
    };
    let black_pawn: Piece = Piece {
        piece_type: PieceType::Pawn,
        colour: Colour::Black,
        str: "p".to_string(),
        int: 6
    };
    let black_knight: Piece = Piece {
        piece_type: PieceType::Knight,
        colour: Colour::Black,
        str: "n".to_string(),
        int: 7
    };
    let black_bishop: Piece = Piece {
        piece_type: PieceType::Bishop,
        colour: Colour::Black,
        str: "b".to_string(),
        int: 8
    };
    let black_rook: Piece = Piece {
        piece_type: PieceType::Rook,
        colour: Colour::Black,
        str: "r".to_string(),
        int: 9
    };
    let black_queen: Piece = Piece {
        piece_type: PieceType::Queen,
        colour: Colour::Black,
        str: "q".to_string(),
        int: 10
    };
    let black_king: Piece = Piece {
        piece_type: PieceType::King,
        colour: Colour::Black,
        str: "k".to_string(),
        int: 11
    };



    let board: [&str; 64] = [
        "R", "N", "B", "Q", "K", "B", "N", "R", "P", "P", "P", "P", "P", "P", "P", "P", " ",
        " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ",
        " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "P", "P", "P", "P", "P",
        "P", "P", "P", "R", "N", "B", "Q", "K", "B", "N", "R",
    ];




    let board: [&str; 64] = [
        "R", "N", "B", "Q", "K", "B", "N", "R", "P", "P", "P", "P", "P", "P", "P", "P", " ",
        " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ",
        " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "P", "P", "P", "P", "P",
        "P", "P", "P", "R", "N", "B", "Q", "K", "B", "N", "R",
    ];

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

        print!("{}[2J", 27 as char);
        for rank in RANKS.rev() {
            println!("");
            for file in FILES {
                let algebraic: String = format!("{}{}", file, rank);
                let index = algebraic_to_index(algebraic).unwrap();
                let piece = board[index];
                print!("[{piece}]");
            }
        }

        println!("");
    }

    fn get_input(board: [&str; 64]) {
        let mut input: String = String::new();

        loop {
            println!("Request a square");
            input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            let input= input.trim().to_string();

            println!("You wrote {input}");

            match algebraic_to_index(input.clone()) {
                Ok(index) => {
                    display_board(board);
                    println!("Piece at {} is {}", input, board[index]);
                    break;
                },
                Err(e) => {
                    display_board(board);
                    println!("{e}");
                }
            }
        }
    }
    display_board(board);

    get_input(board);

    // find_piece(board, file, rank);
}
