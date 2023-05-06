use serde::{Deserialize, Serialize};

use crate::domain::{tile::tic_tac_toe_tile::{PieceType}, board::tic_tac_toe_board::TicTacToeBoard};

#[derive(Debug, Deserialize, Serialize)]
pub struct MinMaxInput {
    pub board: TicTacToeBoard,
    pub piece: PieceType,
}
