#[test]
fn should_return_a_victory_tile() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});

    let min_max = crate::domain::minmax::min_max_v2::MinMax::new();
    let result = min_max.execute(board, crate::domain::tile::tic_tac_toe_tile::PieceType::A);
    assert_eq!(result.x, 2);
    assert_eq!(result.y, 1);
} 

#[test]
fn should_return_a_mark_tile_1() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});

    let min_max = crate::domain::minmax::min_max_v2::MinMax::new();
    let result = min_max.execute(board, crate::domain::tile::tic_tac_toe_tile::PieceType::A);
    assert_eq!(result.x, 1);
    assert_eq!(result.y, 0);
} 