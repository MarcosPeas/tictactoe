use crate::domain::{
    board::tic_tac_toe_board::TicTacToeBoard, minmax::mini_max_v3::MiniMax,
    tile::tic_tac_toe_tile::PieceType,
};

mod domain;
mod infra;
mod lib;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn main() {
    should_return_a_victory_tile();
}

fn should_return_a_victory_tile() {
    let piece_type = PieceType::A;
    let mut board = TicTacToeBoard::emtpy_3x3();
    board.do_move(1, 1, -1);
    board.do_move(2, 0, 1);
    board.do_move(2, 2, -1);
    board.do_move(0, 0, 1);
    board.do_move(2, 1, -1);
    board.do_move(0, 1, 1);
    board.do_move(0, 2, -1);

    if piece_type == PieceType::B {
        board = board.reverse();
    }

    let mut min_max = MiniMax::new(board);

    let result = min_max.execute();
    println!("Jogar em: {:?}", result);

    /*let input_value = MinMaxInput{
        board,
        piece: PieceType::A,
    };
    let serialized = serde_json::to_string(&input_value).unwrap();
    println!("{}", serialized);*/
}
