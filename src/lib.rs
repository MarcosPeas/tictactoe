#![cfg(target_os="android")]
#![allow(non_snake_case)]
#![no_main]

use std::ffi::{CString, CStr};
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jstring};

use domain::{tile::tile::Tile, minmax::min_max_v2::MinMax};
use infra::presenter::min_max_input::MinMaxInput;

use crate::domain::tile::tile::PieceType;

mod domain;
mod infra;

#[no_mangle]
pub unsafe extern fn Java_com_peas_tictactoe_flutter_1project_FlutterProjectPlugin_execute(env: JNIEnv, _: JObject, j_recipient: JString) -> jstring {
    let recipient = CString::from(
        CStr::from_ptr(
            env.get_string(j_recipient).unwrap().as_ptr()
        )
    );
    let json_input = recipient.to_str().unwrap();
    let input = serde_json::from_str::<MinMaxInput>(json_input).unwrap();
    let tile = find_move(input);
    let json_output = serde_json::to_string(&tile).unwrap();
    let output = env.new_string(json_output.as_str()).unwrap();
    output.into_inner()
}

fn find_move(input: MinMaxInput) -> Tile {
    let piece = input.piece;
    let mut board = input.board;
    if piece == PieceType::B {
        board = board.reverse();
    }
    let min_max = MinMax::new();
    let result = min_max.execute(board, PieceType::A);
    return result;
}

