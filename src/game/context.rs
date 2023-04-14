use crate::game::{
    board::Board,
    player::Player,
};

#[derive(Debug)]
pub struct Context {
    pub board: Board,
    pub player: Player,
}

impl Context {
    pub fn new() -> Context {
        Context {
            board: Board::new(),
            player: Player::new(),
        }
    }
}