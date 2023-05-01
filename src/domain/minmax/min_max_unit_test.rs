#[test]
fn should_return_a_victory_tile() {
    let mut board = crate::domain::board::board::Board::emtpy_3x3();
    board.do_move(crate::domain::tile::tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tile::PieceType::A});
    board.do_move(crate::domain::tile::tile::Tile{x: 1, y: 0, piece: crate::domain::tile::tile::PieceType::A});
    board.do_move(crate::domain::tile::tile::Tile{x: 0, y: 2, piece: crate::domain::tile::tile::PieceType::A});
    board.do_move(crate::domain::tile::tile::Tile{x: 0, y: 1, piece: crate::domain::tile::tile::PieceType::B});
    board.do_move(crate::domain::tile::tile::Tile{x: 1, y: 1, piece: crate::domain::tile::tile::PieceType::B});
    board.do_move(crate::domain::tile::tile::Tile{x: 2, y: 0, piece: crate::domain::tile::tile::PieceType::B});

    let min_max = crate::domain::minmax::min_max::MinMax::new();
    let result = min_max.execute(board, crate::domain::tile::tile::PieceType::A, crate::domain::tile::tile::PieceType::A);
    assert_eq!(result.x, 2);
    assert_eq!(result.y, 1);
} 

#[test]
fn should_return_a_mark_tile_1() {
    let mut board = crate::domain::board::board::Board::emtpy_3x3();
    board.do_move(crate::domain::tile::tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tile::PieceType::A});
    board.do_move(crate::domain::tile::tile::Tile{x: 2, y: 2, piece: crate::domain::tile::tile::PieceType::A});
    board.do_move(crate::domain::tile::tile::Tile{x: 1, y: 1, piece: crate::domain::tile::tile::PieceType::B});
    board.do_move(crate::domain::tile::tile::Tile{x: 1, y: 2, piece: crate::domain::tile::tile::PieceType::B});

    let min_max = crate::domain::minmax::min_max::MinMax::new();
    let result = min_max.execute(board, crate::domain::tile::tile::PieceType::A, crate::domain::tile::tile::PieceType::A);
    assert_eq!(result.x, 1);
    assert_eq!(result.y, 0);
} 