use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
pub struct Tile {
    pub x: u8,
    pub y: u8,
    pub piece: i8,
}

impl Tile {
    pub fn new(x: u8, y: u8) -> Self {
        Tile {
            x,
            y,
            piece: 0,
        }
    }

    pub fn add_piece(&mut self, piece: i8) {
        self.piece = piece;
    }

    pub fn get_id(self) -> String {
        format!("{}-{}", self.x.clone(), self.y.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
pub enum PieceType {
    A,
    B,
    None,
}
