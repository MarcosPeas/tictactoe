use crate::domain::tile::tile::PieceType;

mod domain;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn main (){
    should_return_a_victory_tile();
}

fn should_return_a_victory_tile() {
    let mut board = crate::domain::board::board::Board::emtpy_3x3();
    board.do_move(crate::domain::tile::tile::Tile{x: 1, y: 1, piece: PieceType::B});
    board.do_move(crate::domain::tile::tile::Tile{x: 0, y: 2, piece: PieceType::A});
    board.do_move(crate::domain::tile::tile::Tile{x: 2, y: 0, piece: PieceType::B});
    board.do_move(crate::domain::tile::tile::Tile{x: 2, y: 2, piece: PieceType::A});
    board.do_move(crate::domain::tile::tile::Tile{x: 1, y: 0, piece: PieceType::B});
    let min_max = crate::domain::minmax::min_max_v2::MinMax::new();
    let result = min_max.execute(board, PieceType::A);
    println!("Jogar em: {:?}", result);
} 