#[test]
fn should_return_emtpy_when_not_has_matches() {
    let board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    let result = board.matches(1);
    assert_eq!(result.len(), 0);
}

#[test]
fn should_return_tree_tile_when_has_one_vertical_match() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(0, 0, 1);
    board.do_move(0, 1, 1);
    board.do_move(0, 2, 1);
    let result = board.matches(1);
    assert_eq!(result.len(), 3);
}

#[test]
fn should_return_six_tile_when_has_two_vertical_matches() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(0, 0, 1);
    board.do_move(0, 1, 1);
    board.do_move(0, 2, 1);
    board.do_move(1, 0, 1);
    board.do_move(1, 1, 1);
    board.do_move(1, 2, 1);
    let result = board.matches(1);
    assert_eq!(result.len(), 6);
}

#[test]
fn should_return_nine_tile_when_has_tree_vertical_matches() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(0, 0, 1);
    board.do_move(0, 1, 1);
    board.do_move(0, 2, 1);
    board.do_move(1, 0, 1);
    board.do_move(1, 1, 1);
    board.do_move(1, 2, 1);
    board.do_move(2, 0, 1);
    board.do_move(2, 1, 1);
    board.do_move(2, 2, 1);
    let result = board.matches(1);
    assert_eq!(result.len(), 24);
}


#[test]
fn should_return_tree_tile_when_has_one_horizontal_match() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(0, 0, 1);
    board.do_move(1, 0, 1);
    board.do_move(2, 0, 1);
    let result = board.matches(1);
    assert_eq!(result.len(), 3);
}


#[test]
fn should_return_six_tile_when_has_two_horizontal_matches() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(0, 0, 1);
    board.do_move(1, 0, 1);
    board.do_move(2, 0, 1);
    board.do_move(0, 1, 1);
    board.do_move(1, 1, 1);
    board.do_move(2, 1, 1);
    let result = board.matches(1);
    assert_eq!(result.len(), 6);
}


#[test]
fn should_return_nine_tile_when_has_tree_horizontal_matches() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(0, 0, 1);
    board.do_move(1, 0, 1);
    board.do_move(2, 0, 1);
    board.do_move(0, 1, -1);
    board.do_move(1, 1, -1);
    board.do_move(2, 1, -1);
    board.do_move(0, 2, 1);
    board.do_move(1, 2, 1);
    board.do_move(2, 2, 1);
    let result = board.matches(1);
    assert_eq!(result.len(), 6);
}



#[test]
fn should_return_nine_tile_when_has_doble_diagonal_matches() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(0, 0, 1);
    board.do_move(1, 0, -1);
    board.do_move(2, 0, 1);
    board.do_move(0, 1, -1);
    board.do_move(1, 1, 1);
    board.do_move(2, 1, -1);
    board.do_move(0, 2, 1);
    board.do_move(1, 2, -1);
    board.do_move(2, 2, 1);
    let result = board.matches(1);
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
    board.do_move(0, 0, -1);
    board.do_move(1, 1, -1);
    board.do_move(2, 2, -1);
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
    board.do_move(0, 0, 1);
    board.do_move(1, 0, 1);
    board.do_move(2, 0, 1);
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
    board.do_move(2, 0, -1);
    board.do_move(2, 1, -1);
    board.do_move(2, 2, -1);
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
    board.do_move(0, 0, 1);
    board.do_move(1, 0, -1);
    board.do_move(2, 0, 1);
    board.do_move(0, 1, -1);
    board.do_move(1, 1, -1);
    board.do_move(2, 1, 1);
    board.do_move(0, 2, 1);
    board.do_move(1, 2, 1);
    board.do_move(2, 2, -1);
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
    board.do_move(0, 0, -1);
    board.do_move(1, 1, -1);
    board.do_move(2, 2, -1);
    assert_eq!(board.is_full(), false);
}

#[test]
fn should_return_true_when_is_full() {
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(0, 0, 1);
    board.do_move(1, 0, -1);
    board.do_move(2, 0, 1);
    board.do_move(0, 1, -1);
    board.do_move(1, 1, 1);
    board.do_move(2, 1, -1);
    board.do_move(0, 2, 1);
    board.do_move(1, 2, -1);
    board.do_move(2, 2, 1);
    assert_eq!(board.is_full(), true);
}

#[test]
fn should_return_nine_free_tiles(){
    let board  = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    let moves = board.get_valids_moves();
    assert_eq!(moves.len(), 9);
}


#[test]
fn should_return_six_free_tiles(){
    let mut board = crate::domain::board::tic_tac_toe_board::TicTacToeBoard::emtpy_3x3();
    board.do_move(0, 0, 1);
    board.do_move(1, 0, -1);
    board.do_move(2, 0, 1);
    let moves = board.get_valids_moves();
    assert_eq!(moves.len(), 6);
}