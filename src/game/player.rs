#[derive(Debug)]
pub struct Player {
    pub money: i32,
    pub xp: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            money: 0,
            xp: 0,
        }
    }
}