use crate::game::{
    board::Board,
    player::Player,
};
use crate::menus::market::Market;

#[derive(Debug)]
pub struct Context {
    pub board: Board,
    pub player: Player,
    pub market: Market,
}

impl Context {
    pub fn new() -> Context {
        Context {
            board: Board::new(),
            player: Player::new(),
            market: Market::new(),
        }
    }
}