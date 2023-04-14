use crate::game::board::Board;

#[derive(Debug)]
pub struct Context {
    pub board: Board,
}

impl Context {
    pub fn new() -> Context {
        Context {
            board: Board::new(),
        }
    }
}