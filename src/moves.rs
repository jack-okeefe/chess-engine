use crate::pieces::{Class, Colour};
use crate::position::{self, Position};
use crate::position::{FILES_AB, FILES_GH, FILE_A, FILE_H, RANK_1, RANK_2, RANK_7, RANK_8};

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

pub enum KnightDirection {
    NorthNorthEast,
    EastNorthEast,
    EastSouthEast,
    SouthSouthEast,
    SouthSouthWest,
    WestSouthWest,
    WestNorthWest,
    NorthNorthWest,
}

pub fn check_if_square_obstructed(
    position: &Position,
    square: &u64,
    friendly_colour: &Colour,
) -> bool {
    let occupancy = position.get_colour_occupancy(friendly_colour);
    square & occupancy != 0
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
    moves: &mut u64,
    directions: Vec<Direction>,
    travel_limit: u8,
    position: &Position,
    root_square: &u64,
    friendly_colour: &Colour,
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

            let is_over_travel_limit = travel_distance > travel_limit;

            !is_square_obstructed
                && !is_over_travel_limit
                && !was_previous_capture
                && !was_previous_edge
        } {
            // don't want to allow moving to the same square,
            // but also need to start algorithm here in case the
            // root square is on an edge
            if current_square != *root_square {
                *moves |= current_square;
            }
            if let Some(target_piece) = position.get_piece_at_index(&current_square) {
                was_previous_capture = friendly_colour != &target_piece.colour()
            }
            was_previous_edge = is_at_edge_in_direction(&direction, &current_square);
            current_square = step_in_direction(&direction, &current_square);
            travel_distance += 1;
        }
    }
}

enum PawnAttackDirection {
    East,
    West,
}

fn generate_pawn_attacks(
    root_square: &u64,
    position: &Position,
    attack_direction: &PawnAttackDirection,
    friendly_colour: &Colour,
) -> u64 {
    let attack_square = match (friendly_colour, attack_direction) {
        (Colour::White, PawnAttackDirection::East) => (root_square & !FILE_H) << 9,
        (Colour::Black, PawnAttackDirection::East) => (root_square & !FILE_H) >> 7,
        (Colour::White, PawnAttackDirection::West) => (root_square & !FILE_A) << 7,
        (Colour::Black, PawnAttackDirection::West) => (root_square & !FILE_A) >> 9,
    };
    return attack_square & position.get_colour_occupancy(&!friendly_colour);
}

pub fn generate_moves(position: &Position, square: &u64) -> u64 {
    let mut moves: u64 = 0b0;

    if let Some(piece) = position.get_piece_at_index(square) {
        match piece.class() {
            Class::Pawn => {
                let mut first_push: u64 = match piece.colour() {
                    Colour::White => square << 8,
                    Colour::Black => square >> 8,
                };
                first_push &= !position.get_occupancy();
                let second_push: u64;
                let pawn_on_home_row = match piece.colour() {
                    Colour::White => square & RANK_2 != 0,
                    Colour::Black => square & RANK_7 != 0,
                };
                let mut second_push: u64 = 0b0;
                if pawn_on_home_row {
                    second_push = match piece.colour() {
                        Colour::White => first_push << 8,
                        Colour::Black => first_push >> 8,
                    };
                    second_push &= !position.get_occupancy();
                }
                moves |= first_push | second_push;

                let pawn_east_attacks = generate_pawn_attacks(
                    square,
                    position,
                    &PawnAttackDirection::East,
                    &piece.colour(),
                );
                moves |= pawn_east_attacks;

                let pawn_west_attacks = generate_pawn_attacks(
                    square,
                    position,
                    &PawnAttackDirection::West,
                    &piece.colour(),
                );
                moves |= pawn_west_attacks;
            }
            Class::Knight => {
                // NorthNorthEast --> NorthNorthWest
                moves |= (square & !FILE_H) << 17;
                moves |= (square & !FILES_GH) << 10;
                moves |= (square & !FILES_GH) >> 6;
                moves |= (square & !FILE_H) >> 15;
                moves |= (square & !FILE_A) << 15;
                moves |= (square & !FILES_AB) << 6;
                moves |= (square & !FILES_AB) >> 10;
                moves |= (square & !FILE_A) >> 17;

                let occupied_squares = position.get_colour_occupancy(&piece.colour());
                moves &= !occupied_squares;
            }
            Class::Bishop => {
                generate_straight_moves(
                    &mut moves,
                    vec![
                        Direction::NorthEast,
                        Direction::SouthEast,
                        Direction::SouthWest,
                        Direction::NorthWest,
                    ],
                    7,
                    position,
                    square,
                    &piece.colour(),
                );
            }
            Class::Rook => {
                generate_straight_moves(
                    &mut moves,
                    vec![
                        Direction::North,
                        Direction::East,
                        Direction::South,
                        Direction::West,
                    ],
                    7,
                    position,
                    square,
                    &piece.colour(),
                );
            }
            Class::Queen => {
                generate_straight_moves(
                    &mut moves,
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
                    &piece.colour(),
                );
            }
            Class::King => {
                generate_straight_moves(
                    &mut moves,
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
                    1,
                    position,
                    square,
                    &piece.colour(),
                );
            }
        }
    }
    moves
}
