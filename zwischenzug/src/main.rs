// This a work in progress refactoring of the chess logic
// Please check out the other Rust files for the current logic implementation progress
// The main function currently does not do anything

#![allow(nonstandard_style)]
#![allow(dead_code)]
#![allow(unused)]
#![allow(non_camel_case_types)]

use board::piece::chess_core::*;
use board::piece::*;

mod board;

fn main()
{
    println!("Hello, world!");
    let new_board = board::Chess_Board::new(Player::White);
}
