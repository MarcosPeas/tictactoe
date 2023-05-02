use serde::{Deserialize, Serialize};

use crate::domain::{tile::tile::{PieceType}, board::board::Board};

#[derive(Debug, Deserialize, Serialize)]
pub struct MinMaxInput {
    pub board: Board,
    pub piece: PieceType,
}
