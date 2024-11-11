#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Colour {
    White,
    Black,
}

// https://users.rust-lang.org/t/custom-struct-as-key-to-hashmap/21534
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Piece {
    pub piece_type: Type,
    pub colour: Colour,
    pub str: &'static str,
}

pub const WHITE_PAWN: Piece = Piece {
    piece_type: Type::Pawn,
    colour: Colour::White,
    str: "P",
};
pub const WHITE_KNIGHT: Piece = Piece {
    piece_type: Type::Knight,
    colour: Colour::White,
    str: "N",
};
pub const WHITE_BISHOP: Piece = Piece {
    piece_type: Type::Bishop,
    colour: Colour::White,
    str: "B",
};
pub const WHITE_ROOK: Piece = Piece {
    piece_type: Type::Rook,
    colour: Colour::White,
    str: "R",
};
pub const WHITE_QUEEN: Piece = Piece {
    piece_type: Type::Queen,
    colour: Colour::White,
    str: "Q",
};
pub const WHITE_KING: Piece = Piece {
    piece_type: Type::King,
    colour: Colour::White,
    str: "K",
};
pub const BLACK_PAWN: Piece = Piece {
    piece_type: Type::Pawn,
    colour: Colour::Black,
    str: "p",
};
pub const BLACK_KNIGHT: Piece = Piece {
    piece_type: Type::Knight,
    colour: Colour::Black,
    str: "n",
};
pub const BLACK_BISHOP: Piece = Piece {
    piece_type: Type::Bishop,
    colour: Colour::Black,
    str: "b",
};
pub const BLACK_ROOK: Piece = Piece {
    piece_type: Type::Rook,
    colour: Colour::Black,
    str: "r",
};
pub const BLACK_QUEEN: Piece = Piece {
    piece_type: Type::Queen,
    colour: Colour::Black,
    str: "q",
};
pub const BLACK_KING: Piece = Piece {
    piece_type: Type::King,
    colour: Colour::Black,
    str: "k",
};
