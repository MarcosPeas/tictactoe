use crate::domain::{board::tic_tac_toe_board::TicTacToeBoard, minmax::mini_max_v3::MiniMax};

mod domain;
mod infra;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn main() {
    should_return_a_victory_tile();
}

fn should_return_a_victory_tile() {
    let piece = 1;
    let mut board = TicTacToeBoard::emtpy_3x3();
    //board.do_move(1, 1, -1);
    //board.do_move(1, 1, -1);
    if piece == -1 {
        board = board.reverse();
    }
    let mut min_max = MiniMax::new(board);

    let result = min_max.execute();
    println!("Jogar em: {:?}", result);
}
