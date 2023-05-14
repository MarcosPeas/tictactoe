use serde::{Deserialize, Serialize};

use crate::domain::board::tic_tac_toe_board::TicTacToeBoard;


#[derive(Debug, Deserialize, Serialize)]
pub struct MiniMaxInput {
    pub board: TicTacToeBoard,
    pub piece: i8,
}
