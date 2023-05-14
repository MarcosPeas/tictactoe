use serde::{Deserialize, Serialize};

use crate::domain::tile::tic_tac_toe_tile::Tile;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Trail {
    pub head: Tile,
    pub body: Vec<Tile>,
    pub tail: Vec<Tile>,
}

impl Trail {
    pub fn new(head: Tile) -> Self {
        Trail {
            head,
            body: Vec::new(),
            tail: Vec::new(),
        }
    }
}
