// #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Class {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(PartialEq)]
pub enum Colour {
    White,
    Black,
}

pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen, 
    BlackKing,
}

impl Piece {
    pub fn iter() -> &'static [Piece] {
        &[
            Piece::WhitePawn,
            Piece::WhiteKnight,
            Piece::WhiteBishop,
            Piece::WhiteRook,
            Piece::WhiteQueen,
            Piece::WhiteKing,
            Piece::BlackPawn,
            Piece::BlackKnight,
            Piece::BlackBishop,
            Piece::BlackRook,
            Piece::BlackQueen, 
            Piece::BlackKing,
        ]
    }

    pub fn str(&self) -> &'static str {
        match self {
            Piece::WhitePawn => "P",
            Piece::WhiteKnight => "N",
            Piece::WhiteBishop => "B",
            Piece::WhiteRook => "R",
            Piece::WhiteQueen => "Q",
            Piece::WhiteKing => "K",
            Piece::BlackPawn => "p",
            Piece::BlackKnight => "n",
            Piece::BlackBishop => "b",
            Piece::BlackRook => "r",
            Piece::BlackQueen => "q",
            Piece::BlackKing => "k",
        }
    }
    pub fn class(&self) -> Class {
        match self {
            Piece::WhitePawn => Class::Pawn,
            Piece::WhiteKnight => Class::Knight,
            Piece::WhiteBishop => Class::Bishop,
            Piece::WhiteRook => Class::Rook,
            Piece::WhiteQueen => Class::Queen,
            Piece::WhiteKing => Class::King,
            Piece::BlackPawn => Class::Pawn,
            Piece::BlackKnight => Class::Knight,
            Piece::BlackBishop => Class::Bishop,
            Piece::BlackRook => Class::Rook,
            Piece::BlackQueen => Class::Queen,
            Piece::BlackKing => Class::King,
        }
    }
    pub fn colour(&self) -> Colour {
        match self {
            Piece::WhitePawn => Colour::White,
            Piece::WhiteKnight => Colour::White,
            Piece::WhiteBishop => Colour::White,
            Piece::WhiteRook => Colour::White,
            Piece::WhiteQueen => Colour::White,
            Piece::WhiteKing => Colour::White,
            Piece::BlackPawn => Colour::Black,
            Piece::BlackKnight => Colour::Black,
            Piece::BlackBishop => Colour::Black,
            Piece::BlackRook => Colour::Black,
            Piece::BlackQueen => Colour::Black,
            Piece::BlackKing => Colour::Black,
        }

    }
}

// https://users.rust-lang.org/t/custom-struct-as-key-to-hashmap/21534
// #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Piece {
//     pub piece_type: Type,
//     pub colour: Colour,
//     pub str: &'static str,
// }

// pub const WHITE_PAWN: Piece = Piece {
//     piece_type: Type::Pawn,
//     colour: Colour::White,
//     str: "P",
// };
// pub const WHITE_KNIGHT: Piece = Piece {
//     piece_type: Type::Knight,
//     colour: Colour::White,
//     str: "N",
// };
// pub const WHITE_BISHOP: Piece = Piece {
//     piece_type: Type::Bishop,
//     colour: Colour::White,
//     str: "B",
// };
// pub const WHITE_ROOK: Piece = Piece {
//     piece_type: Type::Rook,
//     colour: Colour::White,
//     str: "R",
// };
// pub const WHITE_QUEEN: Piece = Piece {
//     piece_type: Type::Queen,
//     colour: Colour::White,
//     str: "Q",
// };
// pub const WHITE_KING: Piece = Piece {
//     piece_type: Type::King,
//     colour: Colour::White,
//     str: "K",
// };
// pub const BLACK_PAWN: Piece = Piece {
//     piece_type: Type::Pawn,
//     colour: Colour::Black,
//     str: "p",
// };
// pub const BLACK_KNIGHT: Piece = Piece {
//     piece_type: Type::Knight,
//     colour: Colour::Black,
//     str: "n",
// };
// pub const BLACK_BISHOP: Piece = Piece {
//     piece_type: Type::Bishop,
//     colour: Colour::Black,
//     str: "b",
// };
// pub const BLACK_ROOK: Piece = Piece {
//     piece_type: Type::Rook,
//     colour: Colour::Black,
//     str: "r",
// };
// pub const BLACK_QUEEN: Piece = Piece {
//     piece_type: Type::Queen,
//     colour: Colour::Black,
//     str: "q",
// };
// pub const BLACK_KING: Piece = Piece {
//     piece_type: Type::King,
//     colour: Colour::Black,
//     str: "k",
// };
