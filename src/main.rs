use core::hash;
use std::cmp;
use std::collections::btree_map::Range;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::Binary;
use std::hash::Hash;
use std::io::Write;
use std::ops::Index;
use std::str::SplitWhitespace;
use std::string::ParseError;
use std::{any::type_name, io};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Colour {
    White,
    Black,
}

// https://users.rust-lang.org/t/custom-struct-as-key-to-hashmap/21534
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Piece {
    piece_type: PieceType,
    colour: Colour,
    str: &'static str,
}

// struct Position {
//     white_pawn: u64,
//     white_knight: u64,
//     white_bishop: u64,
//     white_rook: u64,
//     white_queen: u64,
//     white_king: u64,
//     black_pawn: u64,
//     black_knight: u64,
//     black_bishop: u64,
//     black_rook: u64,
//     black_queen: u64,
//     black_king: u64,
// }

const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const RANKS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
const FILE_A: u64 = 0b0000000100000001000000010000000100000001000000010000000100000001;
const FILE_H: u64 = 0b1000000010000000100000001000000010000000100000001000000010000000;
const RANK_1: u64 = 0b0000000000000000000000000000000000000000000000000000000011111111;
const RANK_8: u64 = 0b1111111100000000000000000000000000000000000000000000000000000000;

const WHITE_PAWN: Piece = Piece {
    piece_type: PieceType::Pawn,
    colour: Colour::White,
    str: "P",
};
const WHITE_KNIGHT: Piece = Piece {
    piece_type: PieceType::Knight,
    colour: Colour::White,
    str: "N",
};
const WHITE_BISHOP: Piece = Piece {
    piece_type: PieceType::Bishop,
    colour: Colour::White,
    str: "B",
};
const WHITE_ROOK: Piece = Piece {
    piece_type: PieceType::Rook,
    colour: Colour::White,
    str: "R",
};
const WHITE_QUEEN: Piece = Piece {
    piece_type: PieceType::Queen,
    colour: Colour::White,
    str: "Q",
};
const WHITE_KING: Piece = Piece {
    piece_type: PieceType::King,
    colour: Colour::White,
    str: "K",
};
const BLACK_PAWN: Piece = Piece {
    piece_type: PieceType::Pawn,
    colour: Colour::Black,
    str: "p",
};
const BLACK_KNIGHT: Piece = Piece {
    piece_type: PieceType::Knight,
    colour: Colour::Black,
    str: "n",
};
const BLACK_BISHOP: Piece = Piece {
    piece_type: PieceType::Bishop,
    colour: Colour::Black,
    str: "b",
};
const BLACK_ROOK: Piece = Piece {
    piece_type: PieceType::Rook,
    colour: Colour::Black,
    str: "r",
};
const BLACK_QUEEN: Piece = Piece {
    piece_type: PieceType::Queen,
    colour: Colour::Black,
    str: "q",
};
const BLACK_KING: Piece = Piece {
    piece_type: PieceType::King,
    colour: Colour::Black,
    str: "k",
};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // let pieces = vec![white_pawn, white_knight, white_bishop, white_rook, white_queen, white_king, black_pawn, black_knight, black_bishop, black_rook, black_queen, black_king];

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
    fn index_to_algebraic(index: &usize) -> Result<String, &'static str> {
        if *index > 63 {
            return Err("Invalid index. Must be less than 64");
        } else {
            let file_index = *index % 8;
            let file = FILES[file_index].to_string();
            let rank = (*index / 8 + 1).to_string();
            let algebraic = format!("{file}{rank}");
            return Ok(algebraic);
        }
    }

    fn print_board(position: &HashMap<Piece, u64>, highlighted_indicies: Option<HashSet<usize>>) {
        let mut board = [" "; 64];
        let highlighted = match highlighted_indicies {
            Some(indicies) => indicies,
            None => HashSet::new(),
        };

        for (piece, bitboard) in position {
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

    fn bit_scan(bitboard: &u64) -> HashSet<usize> {
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

    fn fill_board(board: &mut [&str; 64], bitboard: &u64, piece: &Piece) {
        let indicies_to_fill = bit_scan(bitboard);
        for index in indicies_to_fill {
            board[index] = piece.str;
        }
    }

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
        0b0000000000000000000000000000000000000000000000000000000010000001,
    );
    position.insert(
        WHITE_QUEEN,
        0b0000000000000000000000000000010000000000000000000000000000001000,
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

    // let mut white_pawn_indicies = bit_scan(starting_position.white_pawn << 8);
    // for index in white_pawn_indicies {
    //     println!("{index}");
    // }

    fn index_to_bitboard(index: usize) -> u64 {
        1 << index
    }

    fn get_piece_at_index(position: &HashMap<Piece, u64>, square: &u64) -> Option<Piece> {
        for (piece, bitboard) in position {
            if (bitboard & square) != 0 {
                return Some(*piece);
            }
        }
        None
    }

    fn check_if_square_obstructed(
        position: &HashMap<Piece, u64>,
        square: &u64,
        friendly_colour: &Colour,
    ) -> bool {
        for (piece, bitboard) in position.iter() {
            if piece.colour != *friendly_colour {
                continue;
            }
            if bitboard & square != 0 {
                return true;
            }
        }
        false
    }
    fn is_at_edge_in_direction(direction: &Direction, square: &u64) -> bool {
        let is_on_file_a = square & FILE_A != 0;
        let is_on_file_h = square & FILE_H != 0;
        let is_on_rank_1 = square & RANK_1 != 0;
        let is_on_rank_8 = square & RANK_8 != 0;

        let is_at_edge = match direction {
            Direction::North => is_on_rank_8,
            Direction::East => is_on_file_h,
            Direction::South => is_on_rank_1,
            Direction::West => is_on_file_a,
            Direction::NorthEast => is_on_rank_8 || is_on_file_h,
            Direction::SouthEast => is_on_rank_1 || is_on_file_h,
            Direction::SouthWest => is_on_rank_1 || is_on_file_a,
            Direction::NorthWest => is_on_rank_8 || is_on_file_a,
        };

        return is_at_edge;
    }

    fn step_in_direction(direction: &Direction, square: &u64) -> u64 {
        let mask = square.clone();
        match direction {
            Direction::North => return mask << 8,
            Direction::East => return mask << 1,
            Direction::South => return mask >> 8,
            Direction::West => return mask >> 1,
            Direction::NorthEast => return mask << 9,
            Direction::SouthEast => return mask >> 9,
            Direction::SouthWest => return mask >> 7,
            Direction::NorthWest => return mask << 7,
        };
    }

    enum Direction {
        North,
        East,
        South,
        West,
        NorthEast,
        SouthEast,
        SouthWest,
        NorthWest,
    }
    fn generate_straight_moves(
        directions: Vec<Direction>,
        travel_limit: u8,
        position: &HashMap<Piece, u64>,
        root_square: &u64,
        friendly_colour: &Colour,
        moves: &mut u64,
    ) {
        for direction in directions {
            let mut current_square = root_square.clone();
            let mut travel_distance: u8 = 0;
            let mut was_previous_capture = false;
            let mut was_previous_edge = false;
            while {
                // don't check for obstructed square on own square
                let mut is_square_obstructed = false;
                if travel_distance != 0 {
                    is_square_obstructed =
                        check_if_square_obstructed(&position, &current_square, &friendly_colour);
                }

                let at_travel_limit = travel_distance >= travel_limit;

                !is_square_obstructed
                    && !at_travel_limit
                    && !was_previous_capture
                    && !was_previous_edge
            } {
                // don't want to allow moving to the same square,
                // but also need to start algorithm here in case the
                // root square is on an edge
                if current_square != *root_square {
                    *moves |= current_square;
                }
                if let Some(target_piece) = get_piece_at_index(position, &current_square) {
                    was_previous_capture = friendly_colour != &target_piece.colour
                }
                was_previous_edge = is_at_edge_in_direction(&direction, &current_square);
                current_square = step_in_direction(&direction, &current_square);
                travel_distance += 1;
            }
        }
    }

    fn generate_moves(position: &HashMap<Piece, u64>, square: &u64) -> u64 {
        let mut moves: u64 = 0b0;

        if let Some(piece) = get_piece_at_index(position, square) {
            match piece.piece_type {
                PieceType::Pawn => {
                    match piece.colour {
                        Colour::White => {
                            // add double push square
                            if square.trailing_zeros() / 8 == 1 {
                                moves |= square << 16;
                            }
                            // add single push square
                            moves |= square << 8
                        }
                        Colour::Black => {
                            // double push check
                            if square.trailing_zeros() / 8 == 6 {
                                moves |= square >> 16;
                            }
                            moves |= square >> 8
                        }
                    }
                }
                PieceType::Knight => {}
                PieceType::Bishop => {
                    generate_straight_moves(
                        vec![
                            Direction::NorthEast,
                            Direction::SouthEast,
                            Direction::SouthWest,
                            Direction::NorthWest,
                        ],
                        7,
                        position,
                        square,
                        &piece.colour,
                        &mut moves,
                    );
                }
                PieceType::Rook => {
                    generate_straight_moves(
                        vec![
                            Direction::North,
                            Direction::East,
                            Direction::South,
                            Direction::West,
                        ],
                        7,
                        position,
                        square,
                        &piece.colour,
                        &mut moves,
                    );
                }
                PieceType::Queen => {
                    generate_straight_moves(
                        vec![
                            Direction::North,
                            Direction::East,
                            Direction::South,
                            Direction::West,
                            Direction::NorthEast,
                            Direction::SouthEast,
                            Direction::SouthWest,
                            Direction::NorthWest,
                        ],
                        7,
                        position,
                        square,
                        &piece.colour,
                        &mut moves,
                    );
                }
                PieceType::King => {}
            }
        }
        moves
    }

    // println!("moves: {:#064b}", moves);

    fn get_input(position: &HashMap<Piece, u64>) {
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

    get_input(&position);
}
