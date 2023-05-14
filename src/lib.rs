#![cfg(target_os="android")]
#![allow(non_snake_case)]
#![no_main]
use std::ffi::{CString, CStr};
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jstring};

mod domain;
mod infra;

use domain::tile::tic_tac_toe_tile::Tile;
use infra::presenter::min_max_input::MiniMaxInput;

use crate::domain::{
    minmax::mini_max_v3::MiniMax,
};

#[no_mangle] //com.helmetsoft.tictactoe
pub unsafe extern fn Java_com_helmetsoft_tictactoe_tictactoe_MainActivity_execute(env: JNIEnv, _: JObject, j_recipient: JString) -> jstring {
    let recipient = CString::from(
        CStr::from_ptr(
            env.get_string(j_recipient).unwrap().as_ptr()
        )
    );
    let json_input = recipient.to_str().unwrap();
    let input = serde_json::from_str::<MiniMaxInput>(json_input).unwrap();
    let tile = find_move(input);
    let json_output = serde_json::to_string(&tile).unwrap();
    let output = env.new_string(json_output.as_str()).unwrap();
    output.into_inner()
}

fn find_move(input: MiniMaxInput) -> Tile {
    let piece = input.piece;
    let mut board = input.board;
    if piece == -1 {
        board = board.reverse();
    }
    let mut min_max = MiniMax::new(board);
    let result = min_max.execute();
    return result;
}

