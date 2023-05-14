use crate::domain::{board::tic_tac_toe_board::RoundResult, trail::trail::Trail, tile::tic_tac_toe_tile::Tile};

pub trait _Board {
    fn get_valids_moves(&self) -> Vec<Trail>;
    fn do_move(&mut self, x: u8, y: u8, piece: i8);
    fn get_result(&self) -> RoundResult;
    fn clone(&self) -> Box<dyn _Board + Sync>;
    fn matches(&self, piece_type: i8) -> Vec<Tile>;
}
