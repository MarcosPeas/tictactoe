use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::tile::tic_tac_toe_tile::Tile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicTacToeBoard {
    tiles: HashMap<String, Tile>,
    pub width: u8,
    pub height: u8,
}

impl TicTacToeBoard {
    pub fn emtpy_3x3() -> Self {
        TicTacToeBoard::from_size(3, 3)
    }

    pub fn from_size(x: u8, y: u8) -> Self {
        let mut board = TicTacToeBoard {
            tiles: HashMap::default(),
            width: y,
            height: x,
        };
        for _x in 0..x {
            for _y in 0..y {
                board.do_move(_x, _y, 0);
            }
        }
        board
    }

    pub fn do_move(&mut self, x: u8, y: u8, piece: i8) {
        let tile = Tile { x, y, piece };
        self.tiles.insert(tile.get_id(), tile);
    }

    pub fn get_result(&self) -> RoundResult {
        let result = self.clone().matches(1);
        if !result.is_empty() {
            return RoundResult::A(result);
        }
        let result = self.clone().matches(-1);
        if !result.is_empty() {
            return RoundResult::B(result);
        }
        if self.is_full() {
            return RoundResult::Draw;
        }
        RoundResult::NoFinished
    }

    pub fn is_full(&self) -> bool {
        let tiles = self.tiles.clone();
        for _x in 0..self.height {
            for _y in 0..self.width {
                let id = format!("{}-{}", _x, _y);
                let tile = *tiles.get(&id).unwrap();
                if tile.piece == 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_valids_moves(&self) -> Vec<Tile> {
        let mut free_tiles: Vec<Tile> = Vec::new();
        for _x in 0..self.height {
            for _y in 0..self.width {
                let id = format!("{}-{}", _x, _y);
                let tile = *self.tiles.get(&id).unwrap();
                if tile.piece == 0 {
                    free_tiles.push(tile);
                }
            }
        }
        free_tiles
    }

    pub fn matches(self, piece_type: i8) -> Vec<Tile> {
        let mut result = vec![];
        result.append(&mut self.verify_vertical(piece_type));
        result.append(&mut self.verify_horizontal(piece_type));
        result.append(&mut self.verify_diagonal(piece_type));
        result
    }

    fn verify_vertical(&self, piece_type: i8) -> Vec<Tile> {
        let mut tiles_finisheds = vec![];
        let tiles = self.tiles.clone();
        let tile00 = *tiles.get(&String::from("0-0")).unwrap();
        let tile01 = *tiles.get(&String::from("0-1")).unwrap();
        let tile02 = *tiles.get(&String::from("0-2")).unwrap();
        if tile00.piece != 0
            && tile00.piece == piece_type
            && (tile00.piece == tile01.piece && tile01.piece == tile02.piece)
        {
            tiles_finisheds.push(tile00);
            tiles_finisheds.push(tile01);
            tiles_finisheds.push(tile02);
        }

        let tile10 = *tiles.get(&String::from("1-0")).unwrap();
        let tile11 = *tiles.get(&String::from("1-1")).unwrap();
        let tile12 = *tiles.get(&String::from("1-2")).unwrap();
        if tile10.piece != 0
            && tile10.piece == piece_type
            && (tile10.piece == tile11.piece && tile11.piece == tile12.piece)
        {
            tiles_finisheds.push(tile10);
            tiles_finisheds.push(tile11);
            tiles_finisheds.push(tile12);
        }

        let tile20 = *tiles.get(&String::from("2-0")).unwrap();
        let tile21 = *tiles.get(&String::from("2-1")).unwrap();
        let tile22 = *tiles.get(&String::from("2-2")).unwrap();
        if tile20.piece != 0
            && tile20.piece == piece_type
            && (tile20.piece == tile21.piece && tile21.piece == tile22.piece)
        {
            tiles_finisheds.push(tile20);
            tiles_finisheds.push(tile21);
            tiles_finisheds.push(tile22);
        }
        tiles_finisheds
    }

    fn verify_horizontal(&self, piece_type: i8) -> Vec<Tile> {
        let mut tiles_finisheds = vec![];
        let tiles = self.tiles.clone();
        let tile00 = *tiles.get(&String::from("0-0")).unwrap();
        let tile10 = *tiles.get(&String::from("1-0")).unwrap();
        let tile20 = *tiles.get(&String::from("2-0")).unwrap();
        if tile00.piece != 0
            && tile00.piece == piece_type
            && (tile00.piece == tile10.piece && tile10.piece == tile20.piece)
        {
            tiles_finisheds.push(tile00);
            tiles_finisheds.push(tile10);
            tiles_finisheds.push(tile20);
        }

        let tile01 = *tiles.get(&String::from("0-1")).unwrap();
        let tile11 = *tiles.get(&String::from("1-1")).unwrap();
        let tile21 = *tiles.get(&String::from("2-1")).unwrap();
        if tile01.piece != 0
            && tile01.piece == piece_type
            && (tile01.piece == tile11.piece && tile11.piece == tile21.piece)
        {
            tiles_finisheds.push(tile01);
            tiles_finisheds.push(tile11);
            tiles_finisheds.push(tile21);
        }

        let tile02 = *tiles.get(&String::from("0-2")).unwrap();
        let tile12 = *tiles.get(&String::from("1-2")).unwrap();
        let tile22 = *tiles.get(&String::from("2-2")).unwrap();
        if tile02.piece != 0
            && tile02.piece == piece_type
            && (tile02.piece == tile12.piece && tile12.piece == tile22.piece)
        {
            tiles_finisheds.push(tile02);
            tiles_finisheds.push(tile12);
            tiles_finisheds.push(tile22);
        }
        tiles_finisheds
    }

    fn verify_diagonal(&self, piece_type: i8) -> Vec<Tile> {
        let mut tiles_finisheds = vec![];
        let tiles = self.tiles.clone();
        let tile00 = *tiles.get(&String::from("0-0")).unwrap();
        let tile11 = *tiles.get(&String::from("1-1")).unwrap();
        let tile22 = *tiles.get(&String::from("2-2")).unwrap();
        if tile00.piece != 0
            && tile00.piece == piece_type
            && (tile00.piece == tile11.piece && tile11.piece == tile22.piece)
        {
            tiles_finisheds.push(tile00);
            tiles_finisheds.push(tile11);
            tiles_finisheds.push(tile22);
        }

        let tile02 = *tiles.get(&String::from("0-2")).unwrap();
        let tile11 = *tiles.get(&String::from("1-1")).unwrap();
        let tile20 = *tiles.get(&String::from("2-0")).unwrap();
        if tile02.piece != 0
            && tile02.piece == piece_type
            && (tile02.piece == tile11.piece && tile11.piece == tile20.piece)
        {
            tiles_finisheds.push(tile02);
            tiles_finisheds.push(tile11);
            tiles_finisheds.push(tile20);
        }
        tiles_finisheds
    }

    pub fn reverse(&self) -> TicTacToeBoard {
        let mut tiles: HashMap<String, Tile> = HashMap::new();
        for x in 0..self.height {
            for y in 0..self.width {
                let id = format!("{}-{}", x, y);
                let current_tile = self.tiles[&id];
                if current_tile.piece == 1 {
                    let mut tile = Tile::new(x, y);
                    tile.add_piece(-1);
                    tiles.insert(id.clone(), tile);
                } else if current_tile.piece == -1 {
                    let mut tile = Tile::new(x, y);
                    tile.add_piece(1);
                    tiles.insert(id.clone(), tile);
                } else {
                    let mut tile = Tile::new(x, y);
                    tile.add_piece(0);
                    tiles.insert(id.clone(), tile);
                }
            }
        }
        TicTacToeBoard {
            tiles,
            width: self.width,
            height: self.height,
        }
    }

    pub fn print(&self) {
        for x in 0..self.height {
            for y in 0..self.width {
                let id = format!("{}-{}", x, y);
                let tile = self.tiles.get(&id).unwrap();
                print!("{} ", tile.piece);
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoundResult {
    A(Vec<Tile>),
    B(Vec<Tile>),
    Draw,
    NoFinished,
}
