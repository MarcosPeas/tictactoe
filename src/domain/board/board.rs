use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::tile::tile::{PieceType, Tile};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    tiles: HashMap<String, Tile>,
    width: u8,
    height: u8,
}

impl Board {
    pub fn emtpy_3x3() -> Self {
        Board::from_size(3, 3)
    }

    pub fn from_size(x: u8, y: u8) -> Self {
        let mut board = Board {
            tiles: HashMap::default(),
            width: y,
            height: x,
        };
        for _x in 0..x {
            for _y in 0..y {
                let tile = Tile::new(_x, _y);
                board.do_move(tile);
            }
        }
        board
    }

    pub fn do_move(&mut self, tile: Tile) {
        self.tiles.insert(tile.clone().get_id(), tile);
    }

    pub fn get_result(&self) -> RoundResult {
        let result = self.clone().matches(PieceType::A);
        if !result.is_empty() {
            return RoundResult::A(result);
        }
        let result = self.clone().matches(PieceType::B);
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
                let tile = tiles.get(&id).unwrap().clone();
                if tile.piece == PieceType::None {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_valids_moves(&self) -> Vec<Tile> {
        let tiles = self.tiles.clone();
        let mut free_tiles: Vec<Tile> = Vec::new();
        for _x in 0..self.height {
            for _y in 0..self.width {
                let id = format!("{}-{}", _x, _y);
                let tile = tiles.get(&id).unwrap().clone();
                if tile.piece == PieceType::None {
                    free_tiles.push(tile);
                }
            }
        }
        free_tiles
    }

    pub fn matches(self, piece_type: PieceType) -> Vec<Tile> {
        let mut result = vec![];
        result.append(&mut self.verify_vertical(&piece_type));
        result.append(&mut self.verify_horizontal(&piece_type));
        result.append(&mut self.verify_diagonal(&piece_type));
        result
    }

    fn verify_vertical(&self, piece_type: &PieceType) -> Vec<Tile> {
        let mut tiles_finisheds = vec![];
        let tiles = self.tiles.clone();
        let tile00 = tiles.get(&String::from("0-0")).unwrap().clone();
        let tile01 = tiles.get(&String::from("0-1")).unwrap().clone();
        let tile02 = tiles.get(&String::from("0-2")).unwrap().clone();
        if tile00.piece != PieceType::None
            && tile00.piece == piece_type.clone()
            && (tile00.piece == tile01.piece && tile01.piece == tile02.piece)
        {
            tiles_finisheds.push(tile00);
            tiles_finisheds.push(tile01);
            tiles_finisheds.push(tile02);
        }

        let tile10 = tiles.get(&String::from("1-0")).unwrap().clone();
        let tile11 = tiles.get(&String::from("1-1")).unwrap().clone();
        let tile12 = tiles.get(&String::from("1-2")).unwrap().clone();
        if tile10.piece != PieceType::None
            && tile10.piece == piece_type.clone()
            && (tile10.piece == tile11.piece && tile11.piece == tile12.piece)
        {
            tiles_finisheds.push(tile10);
            tiles_finisheds.push(tile11);
            tiles_finisheds.push(tile12);
        }

        let tile20 = tiles.get(&String::from("2-0")).unwrap().clone();
        let tile21 = tiles.get(&String::from("2-1")).unwrap().clone();
        let tile22 = tiles.get(&String::from("2-2")).unwrap().clone();
        if tile20.piece != PieceType::None
            && tile20.piece == piece_type.clone()
            && (tile20.piece == tile21.piece && tile21.piece == tile22.piece)
        {
            tiles_finisheds.push(tile20);
            tiles_finisheds.push(tile21);
            tiles_finisheds.push(tile22);
        }
        tiles_finisheds
    }

    fn verify_horizontal(&self, piece_type: &PieceType) -> Vec<Tile> {
        let mut tiles_finisheds = vec![];
        let tiles = self.tiles.clone();
        let tile00 = tiles.get(&String::from("0-0")).unwrap().clone();
        let tile10 = tiles.get(&String::from("1-0")).unwrap().clone();
        let tile20 = tiles.get(&String::from("2-0")).unwrap().clone();
        if tile00.piece != PieceType::None
            && tile00.piece == piece_type.clone()
            && (tile00.piece == tile10.piece && tile10.piece == tile20.piece)
        {
            tiles_finisheds.push(tile00);
            tiles_finisheds.push(tile10);
            tiles_finisheds.push(tile20);
        }

        let tile01 = tiles.get(&String::from("0-1")).unwrap().clone();
        let tile11 = tiles.get(&String::from("1-1")).unwrap().clone();
        let tile21 = tiles.get(&String::from("2-1")).unwrap().clone();
        if tile01.piece != PieceType::None
            && tile01.piece == piece_type.clone()
            && (tile01.piece == tile11.piece && tile11.piece == tile21.piece)
        {
            tiles_finisheds.push(tile01);
            tiles_finisheds.push(tile11);
            tiles_finisheds.push(tile21);
        }

        let tile02 = tiles.get(&String::from("0-2")).unwrap().clone();
        let tile12 = tiles.get(&String::from("1-2")).unwrap().clone();
        let tile22 = tiles.get(&String::from("2-2")).unwrap().clone();
        if tile02.piece != PieceType::None
            && tile02.piece == piece_type.clone()
            && (tile02.piece == tile12.piece && tile12.piece == tile22.piece)
        {
            tiles_finisheds.push(tile02);
            tiles_finisheds.push(tile12);
            tiles_finisheds.push(tile22);
        }
        tiles_finisheds
    }

    fn verify_diagonal(&self, piece_type: &PieceType) -> Vec<Tile> {
        let mut tiles_finisheds = vec![];
        let tiles = self.tiles.clone();
        let tile00 = tiles.get(&String::from("0-0")).unwrap().clone();
        let tile11 = tiles.get(&String::from("1-1")).unwrap().clone();
        let tile22 = tiles.get(&String::from("2-2")).unwrap().clone();
        if tile00.piece != PieceType::None
            && tile00.piece == piece_type.clone()
            && (tile00.piece == tile11.piece && tile11.piece == tile22.piece)
        {
            tiles_finisheds.push(tile00);
            tiles_finisheds.push(tile11);
            tiles_finisheds.push(tile22);
        }

        let tile02 = tiles.get(&String::from("0-2")).unwrap().clone();
        let tile11 = tiles.get(&String::from("1-1")).unwrap().clone();
        let tile20 = tiles.get(&String::from("2-0")).unwrap().clone();
        if tile02.piece != PieceType::None
            && tile02.piece == piece_type.clone()
            && (tile02.piece == tile11.piece && tile11.piece == tile20.piece)
        {
            tiles_finisheds.push(tile02);
            tiles_finisheds.push(tile11);
            tiles_finisheds.push(tile20);
        }
        tiles_finisheds
    }

    pub fn reverse(&self) -> Board {
        let mut tiles: HashMap<String, Tile> = HashMap::new();
        for x in 0..self.height {
            for y in 0..self.width {
                let id = format!("{}-{}", x, y);
                let current_tile = self.tiles[&id];
                if current_tile.piece == PieceType::A {
                    let mut tile = Tile::new(x, y);
                    tile.add_piece(PieceType::B);
                    tiles.insert(id.clone(),tile);
                } else if current_tile.piece == PieceType::B {
                    let mut tile = Tile::new(x, y);
                    tile.add_piece(PieceType::A);
                    tiles.insert(id.clone(),tile);
                } else {
                    let mut tile = Tile::new(x, y);
                    tile.add_piece(PieceType::None);
                    tiles.insert(id.clone(),tile);
                }
            }
        }
        let board = Board {
            tiles,
            width: self.width,
            height: self.height,
        };
        board
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoundResult {
    A(Vec<Tile>),
    B(Vec<Tile>),
    Draw,
    NoFinished,
}
