use std::collections::HashMap;

use crate::pieces::{Colour, Piece, Type};
use crate::position::{FILE_A, FILE_H, RANK_1, RANK_8};
use crate::position::{get_piece_at_index};

pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}


pub fn check_if_square_obstructed(
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

pub fn is_at_edge_in_direction(direction: &Direction, square: &u64) -> bool {
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

pub fn step_in_direction(direction: &Direction, square: &u64) -> u64 {
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

pub fn generate_straight_moves(
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

            !is_square_obstructed && !at_travel_limit && !was_previous_capture && !was_previous_edge
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

pub fn generate_moves(position: &HashMap<Piece, u64>, square: &u64) -> u64 {
    let mut moves: u64 = 0b0;

    if let Some(piece) = get_piece_at_index(position, square) {
        match piece.piece_type {
            Type::Pawn => {
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
            Type::Knight => {}
            Type::Bishop => {
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
            Type::Rook => {
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
            Type::Queen => {
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
            Type::King => {}
        }
    }
    moves
}
