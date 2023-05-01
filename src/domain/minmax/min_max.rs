use std::{
    sync::mpsc::{self, Sender},
    thread,
};

use crate::domain::{
    board::board::{Board, RoundResult},
    tile::tile::{PieceType, Tile},
};
use rand::Rng;

pub struct MinMax;

impl MinMax {
    pub fn new() -> Self {
        MinMax {}
    }

    pub fn execute(
        &self,
        board: Board,
        piece_type: PieceType,
        player_piece_type: PieceType,
    ) -> Tile {
        let valid_moves = board.get_valids_moves();
        if valid_moves.len() >= 4 {
            return self.execute_on_multi_thread(board, piece_type, player_piece_type);
        }
        return self.execute_on_main_thread(board, piece_type, player_piece_type);
    }

    fn execute_on_main_thread(
        &self,
        board: Board,
        piece_type: PieceType,
        player_piece_type: PieceType,
    ) -> Tile {
        let moves = board.get_valids_moves();
        let mut points: Vec<i32> = Vec::new();

        if moves.len() == 1 {
            return moves.first().unwrap().clone();
        }
        for i in 0..moves.len() {
            let tile = &moves[i];
            let tile_points =
                self.get_move_point(&board, tile, &piece_type, player_piece_type.clone(), false);
            points.push(tile_points);
        }
        let selected_tiles = self.select_betters_target_tiles(&points, &moves);
        return self.select_a_random_tile(&selected_tiles);
    }

    fn execute_on_multi_thread(
        &self,
        board: Board,
        piece_type: PieceType,
        player_piece_type: PieceType,
    ) -> Tile {
        let moves = board.get_valids_moves();
        let mut points: Vec<i32> = Vec::new();

        let (tx, rx) = mpsc::channel();

        if moves.len() == 1 {
            return moves.first().unwrap().clone();
        }

        for i in 0..moves.len() {
            let tile = &moves[i].clone();
            let tx1: Sender<i32> = tx.clone();
            let _tile = tile.clone();
            let c_board = board.clone();
            let c_tile = tile.clone();
            let c_piece_type = piece_type.clone();
            let c_player_piece_type = player_piece_type.clone();
            thread::spawn(move || {
                let min_max = MinMax::new();
                let tile_points = min_max.get_move_point(
                    &c_board,
                    &c_tile,
                    &c_piece_type,
                    c_player_piece_type,
                    false,
                );
                tx1.send(tile_points).unwrap();
            });
        }

        thread::spawn(move || {
            tx.send(-999_999_999).unwrap();
        });
        let clon = rx.into_iter();
        for received in clon {
            if received != -999_999_999 {
                points.push(received);
            }
        }
        let selected_tiles = self.select_betters_target_tiles(&points, &moves);
        return self.select_a_random_tile(&selected_tiles);
    }

    fn select_betters_target_tiles(&self, points: &Vec<i32>, moves: &Vec<Tile>) -> Vec<Tile> {
        let mut max_point = -1000_000_000i32;
        let mut selecteds_tiles: Vec<Tile> = Vec::new();
        for i in 0..points.len() {
            let p = points[i];
            if max_point < p {
                max_point = p;
            }
        }

        for i in 0..points.len() {
            let p = points[i];
            if max_point == p {
                selecteds_tiles.push(moves[i].clone());
            }
        }

        selecteds_tiles.iter().for_each(|tile| {
            println!("{:?}", tile);
        });
        println!("*****************************");
        selecteds_tiles
    }

    fn select_a_random_tile(&self, selecteds_tiles: &Vec<Tile>) -> Tile {
        if selecteds_tiles.len() == 1 {
            return selecteds_tiles.first().unwrap().clone();
        }
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..selecteds_tiles.len());
        return selecteds_tiles.get(i).unwrap().clone();
    }

    fn get_move_point(
        &self,
        board: &Board,
        tile: &Tile,
        piece_type: &PieceType,
        player_piece_type: PieceType,
        is_max: bool,
    ) -> i32 {
        let mut cloned_board = board.clone();
        let mut t = Tile::new(tile.x, tile.y);
        t.add_piece(piece_type.clone());
        cloned_board.do_move(t);
        let result = cloned_board.get_result();
        match result {
            RoundResult::A(_) => {
                if player_piece_type == PieceType::A {
                    1
                } else {
                    -1
                }
            }
            RoundResult::B(_) => {
                if player_piece_type == PieceType::B {
                    1
                } else {
                    -1
                }
            }
            RoundResult::Drow => 0,
            RoundResult::NoFinished => {
                let mut points: Vec<i32> = Vec::new();
                let mut next_round_piece = PieceType::A;
                if piece_type.clone() == PieceType::A {
                    next_round_piece = PieceType::B;
                }

                let moves = cloned_board.get_valids_moves();
                for i in 0..moves.len() {
                    let tile = &moves[i];
                    let p = self.get_move_point(
                        &cloned_board,
                        tile,
                        &next_round_piece,
                        player_piece_type.clone(),
                        !is_max,
                    );
                    points.push(p);
                }
                if is_max {
                    return self.max(points);
                }
                return self.min(points);
            }
        }
    }

    fn min(&self, list_points: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        list_points.iter().for_each(|i| {
            min = i32::min(*i, min);
        });
        min
    }

    fn max(&self, list_points: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        list_points.iter().for_each(|i| {
            max = i32::max(*i, max);
        });
        max
    }
}
