use std::{
    sync::mpsc,
    thread,
};

use rand::Rng;

use crate::domain::{
    board::board::Board,
    tile::tile::{PieceType, Tile},
};

pub struct MinMax;

impl MinMax {
    pub fn new() -> Self {
        MinMax {}
    }

    pub fn execute(&self, board: Board, piece_type: PieceType) -> Tile {
        let moves = board.get_valids_moves();
        println!("Valid moves: {}", moves.len());
        if moves.len() >= 20 {
            return self.execute_on_multi_thread(board, piece_type);
        }
        return self.execute_on_sigle_thread(board, piece_type);
    }

    fn execute_on_sigle_thread(&self, board: Board, piece_type: PieceType) -> Tile {
        let valid_moves = board.get_valids_moves();
        let mut points_result: Vec<i8> = Vec::new();
        let mut indexes: Vec<i8> = Vec::new();
        let next_type_move = if piece_type == PieceType::A {
            PieceType::B
        } else {
            PieceType::A
        };
        for i in 0..valid_moves.len() {
            let mut cloned_board = board.clone();
            let mut tile_move = valid_moves.get(i).unwrap().clone();
            tile_move.add_piece(piece_type.clone());
            cloned_board.do_move(tile_move);
            let value = self.min_max(&cloned_board, &next_type_move.clone(), false);
            points_result.push(value);
            indexes.push(i.try_into().unwrap());
        }
        return self.better_value(valid_moves, points_result);
    }

    fn execute_on_multi_thread(&self, board: Board, piece_type: PieceType) -> Tile {
        let (tx, rx) = mpsc::channel();
        let valid_moves = board.get_valids_moves();
        let mut points_result: Vec<i8> = Vec::new();

        for i in 0..valid_moves.len() {
            let mut cloned_board = board.clone();
            let mut tile_move = valid_moves.get(i).unwrap().clone();
            tile_move.add_piece(piece_type.clone());
            cloned_board.do_move(tile_move);
            let tx1 = tx.clone();
            let next_type_move = if piece_type == PieceType::A {
                PieceType::B
            } else {
                PieceType::A
            };
            thread::spawn(move || {
                let min_max = MinMax::new();
                let tile_points = min_max.min_max(&cloned_board, &next_type_move.clone(), false);
                tx1.send(tile_points).unwrap();
            });
        }

        thread::spawn(move || {
            tx.send(i8::MIN).unwrap();
        });
        let clonned = rx.into_iter();
        for received in clonned {
            if received != i8::MIN {
                points_result.push(received);
            } 
        }

        return self.better_value(valid_moves, points_result);
    }

    fn min_max(&self, board: &Board, piece_type: &PieceType, is_max: bool) -> i8 {
        let result = board.get_result();
        match result {
            crate::domain::board::board::RoundResult::A(_) => 1,
            crate::domain::board::board::RoundResult::B(_) => -1,
            crate::domain::board::board::RoundResult::Draw => 0,
            crate::domain::board::board::RoundResult::NoFinished => {
                let mut points: Vec<i8> = Vec::new();
                let moves = board.get_valids_moves();
                for movement in moves {
                    let mut cloned_board = board.clone();
                    let mut board_move = movement.clone();
                    board_move.add_piece(piece_type.clone());
                    cloned_board.do_move(board_move);
                    let next_type_move = if *piece_type == PieceType::A {
                        PieceType::B
                    } else {
                        PieceType::A
                    };
                    let request_points =
                        self.min_max(&cloned_board, &next_type_move.clone(), !is_max);
                    points.push(request_points);
                }
                if is_max {
                    return self.max(&points);
                } else {
                    return self.min(&points);
                }
            }
        }
    }

    fn min(&self, points: &Vec<i8>) -> i8 {
        let mut min = i8::MAX;
        points.iter().for_each(|p| min = i8::min(min, *p));
        min
    }

    fn max(&self, points: &Vec<i8>) -> i8 {
        let mut max = i8::MIN;
        points.iter().for_each(|p| max = i8::max(max, *p));
        max
    }

    fn better_value(&self, tiles: Vec<Tile>, points: Vec<i8>) -> Tile {
        let max_value = self.max(&points);
        let mut max_tiles: Vec<Tile> = Vec::new();
        for i in 0..points.len() {
            let tile_points = points[i];
            if tile_points == max_value {
                max_tiles.push(tiles[i].clone());
            }
        }
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..max_tiles.len());
        println!("All points: {:?}", points);
        println!("Max tiles count: {:?}", max_tiles.len());
        println!("Max tiles: {:?}", max_tiles);
        return max_tiles.get(i).unwrap().clone();
    }
}
