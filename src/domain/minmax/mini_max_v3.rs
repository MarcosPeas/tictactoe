use std::{sync::mpsc, thread};

use rand::Rng;

use crate::domain::{
    board::tic_tac_toe_board::{RoundResult, TicTacToeBoard},
    tile::tic_tac_toe_tile::Tile,
};

pub struct MiniMax {
    board: TicTacToeBoard,
}

impl MiniMax {
    pub fn new(board: TicTacToeBoard) -> Self {
        MiniMax { board }
    }

    pub fn execute(&mut self) -> Tile {
        let moves = self.board.get_valids_moves();
        if moves.len() > 8 {
            return self.execute_on_multi_thread();
        }
        self.execute_on_sigle_thread()
    }

    pub fn execute_on_sigle_thread(&mut self) -> Tile {
        let mut results: Vec<i8> = vec![];
        let moves = self.board.get_valids_moves();

        for tile in &moves {
            self.board.do_move(tile.x, tile.y, 1);
            let value = self.mini_max(-1);
            results.push(value);
            self.board.do_move(tile.x, tile.y, 0);
        }

        self.better_value(moves, results)
    }

    pub fn execute_on_multi_thread(&mut self) -> Tile {
        let mut results: Vec<i8> = vec![];
        let mut tiles_results: Vec<Tile> = vec![];
        let moves = self.board.get_valids_moves();

        let (tx, rx) = mpsc::channel();
        for tile in moves {
            let mut cloned_board = self.board.clone();
            cloned_board.do_move(tile.x, tile.y, 1);
            //cloned_board.print();
            //println!();
            let tx1 = tx.clone();
            thread::spawn(move || {
                let mut min_max = MiniMax::new(cloned_board);
                let tile_points = min_max.mini_max(-1);
                tx1.send((tile_points, tile)).unwrap();
            });
        }
        thread::spawn(move || {
            tx.send((i8::MIN, Tile::new(0, 0))).unwrap();
        });
        let clonned = rx.into_iter();
        for received in clonned {
            if received.0 != i8::MIN {
                results.push(received.0);
                tiles_results.push(received.1);
            }
        }
        self.better_value(tiles_results, results)
    }

    pub fn mini_max(&mut self, ply: i8) -> i8 {
        let result = self.board.get_result();
        match result {
            RoundResult::A(_) => 1,
            RoundResult::B(_) => -1,
            RoundResult::Draw => 0,
            RoundResult::NoFinished => {
                let mut results: Vec<i8> = vec![];
                let moves = self.board.get_valids_moves();
                for tile in &moves {
                    let direction = if ply < 0 { -1 } else { 1 };
                    self.board.do_move(tile.x, tile.y, direction);
                    let value = self.mini_max(-ply);                    
                    results.push(value);
                    self.board.do_move(tile.x, tile.y, 0);
                    if ply < 0 && value < 0 {
                        return value;
                    } else if ply > 0 && value > 0 {
                        return value;
                    }
                }
                if ply < 0 {
                    return self.min(&results);
                }
                self.max(&results)
            }
        }
    }

    fn min(&self, points: &[i8]) -> i8 {
        let mut min = i8::MAX;
        points.iter().for_each(|p| min = i8::min(min, *p));
        min
    }

    fn max(&self, points: &[i8]) -> i8 {
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
                max_tiles.push(tiles[i]);
            }
        }
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..max_tiles.len());
        //println!("All points: {:?}", points);
        //println!("Max tiles count: {:?}", max_tiles.len());
        //println!("Max tiles: {:?}", max_tiles);
        return *max_tiles.get(i).unwrap();
    }
}
