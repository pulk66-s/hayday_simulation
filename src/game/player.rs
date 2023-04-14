#[derive(Debug)]
pub struct Player {
    pub money: u32,
    pub xp: u32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            money: 200,
            xp: 0,
        }
    }
}