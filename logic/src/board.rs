use crate::coordinates::Coordinates;

use super::pieces::Piece;

pub struct Board {
    board: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new(board: [[Option<Piece>; 8]; 8]) -> Self {
        Self { board }
    }

    pub fn set_piece(&mut self, coordinates: &Coordinates, piece: Piece) {
        self.board[coordinates.row()][coordinates.column()] = Some(piece);
    }

    pub fn remove_piece(&mut self, coordinates: &Coordinates) {
        self.board[coordinates.row()][coordinates.column()] = None;
    }
}
