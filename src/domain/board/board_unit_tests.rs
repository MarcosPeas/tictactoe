#[test]
fn should_return_emtpy_when_not_has_matches() {
    let board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    let result = board.matches(crate::domain::tile::tic_tac_toe_tile::PieceType::A);
    assert_eq!(result.len(), 0);
}

#[test]
fn should_return_tree_tile_when_has_one_vertical_match() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    let result = board.matches(crate::domain::tile::tic_tac_toe_tile::PieceType::A);
    assert_eq!(result.len(), 3);
}

#[test]
fn should_return_six_tile_when_has_two_vertical_matches() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    let result = board.matches(crate::domain::tile::tic_tac_toe_tile::PieceType::A);
    assert_eq!(result.len(), 6);
}

#[test]
fn should_return_nine_tile_when_has_tree_vertical_matches() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    let result = board.matches(crate::domain::tile::tic_tac_toe_tile::PieceType::A);
    assert_eq!(result.len(), 24);
}


#[test]
fn should_return_tree_tile_when_has_one_horizontal_match() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    let result = board.matches(crate::domain::tile::tic_tac_toe_tile::PieceType::A);
    assert_eq!(result.len(), 3);
}


#[test]
fn should_return_six_tile_when_has_two_horizontal_matches() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    let result = board.matches(crate::domain::tile::tic_tac_toe_tile::PieceType::A);
    assert_eq!(result.len(), 6);
}


#[test]
fn should_return_nine_tile_when_has_tree_horizontal_matches() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    let result = board.matches(crate::domain::tile::tic_tac_toe_tile::PieceType::A);
    assert_eq!(result.len(), 6);
}



#[test]
fn should_return_nine_tile_when_has_doble_diagonal_matches() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    let result = board.matches(crate::domain::tile::tic_tac_toe_tile::PieceType::A);
    assert_eq!(result.len(), 6);
}

#[test]
fn should_return_drow_when_do_not_has_matches() {
    let board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    assert_eq!(board.get_result(), crate::domain::board::tic_tac_toe_board::RoundResult::NoFinished);
}


#[test]
fn should_return_b_when_has_a_diagonal_match() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    match board.get_result() {
        super::tic_tac_toe_board::RoundResult::A(_) => assert!(false),
        super::tic_tac_toe_board::RoundResult::B(_) => assert!(true),
        super::tic_tac_toe_board::RoundResult::Draw => assert!(false),
        super::tic_tac_toe_board::RoundResult::NoFinished => assert!(false),
    }
}

#[test]
fn should_return_a_when_has_horizontal_match() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    match board.get_result() {
        super::tic_tac_toe_board::RoundResult::A(_) => assert!(true),
        super::tic_tac_toe_board::RoundResult::B(_) => assert!(false),
        super::tic_tac_toe_board::RoundResult::Draw => assert!(false),
        super::tic_tac_toe_board::RoundResult::NoFinished => assert!(false),
    }
}

#[test]
fn should_return_b_when_has_vertical_match() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    match board.get_result() {
        super::tic_tac_toe_board::RoundResult::A(_) => assert!(false),
        super::tic_tac_toe_board::RoundResult::B(_) => assert!(true),
        super::tic_tac_toe_board::RoundResult::Draw => assert!(false),
        super::tic_tac_toe_board::RoundResult::NoFinished => assert!(false),
    }
}


#[test]
fn should_return_drow_to_finisher_when_the_board_is_full() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    match board.get_result() {
        super::tic_tac_toe_board::RoundResult::A(_) => assert!(false),
        super::tic_tac_toe_board::RoundResult::B(_) => assert!(false),
        super::tic_tac_toe_board::RoundResult::Draw => assert!(true),
        super::tic_tac_toe_board::RoundResult::NoFinished => assert!(false),
    }
}

#[test]
fn should_return_false_when_is_not_full() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    assert_eq!(board.is_full(), false);
}

#[test]
fn should_return_true_when_is_full() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 1, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 2, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    assert_eq!(board.is_full(), true);
}

#[test]
fn should_return_nine_free_tiles(){
    let board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    let moves = board.get_valids_moves();
    assert_eq!(moves.len(), 9);
}


#[test]
fn should_return_six_free_tiles(){
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 0, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 1, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::B});
    board.do_move(crate::domain::tile::tic_tac_toe_tile::Tile{x: 2, y: 0, piece: crate::domain::tile::tic_tac_toe_tile::PieceType::A});
    let moves = board.get_valids_moves();
    assert_eq!(moves.len(), 6);
}