#[derive(Debug, Clone)]
pub struct Player {
    pub money: u32,
    pub xp: u32,
    pub time: u32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            money: 200,
            xp: 0,
            time: 0,
        }
    }
}