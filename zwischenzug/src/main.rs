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
