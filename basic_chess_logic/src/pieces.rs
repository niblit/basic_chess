pub struct Piece {
    color: PieceColor,
    kind: PieceType,
}

impl Piece {
    pub fn new(color: PieceColor, kind: PieceType) -> Self {
        Self { color, kind }
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match (&self.color, &self.kind) {
            (PieceColor::Black, PieceType::Pawn) => String::from("p"),
            (PieceColor::Black, PieceType::Knight) => String::from("n"),
            (PieceColor::Black, PieceType::Bishop) => String::from("b"),
            (PieceColor::Black, PieceType::Rook) => String::from("r"),
            (PieceColor::Black, PieceType::Queen) => String::from("q"),
            (PieceColor::Black, PieceType::King) => String::from("k"),

            (PieceColor::White, PieceType::Pawn) => String::from("P"),
            (PieceColor::White, PieceType::Knight) => String::from("N"),
            (PieceColor::White, PieceType::Bishop) => String::from("B"),
            (PieceColor::White, PieceType::Rook) => String::from("R"),
            (PieceColor::White, PieceType::Queen) => String::from("Q"),
            (PieceColor::White, PieceType::King) => String::from("K"),
        }
    }
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum PieceColor {
    Black,
    White,
}
