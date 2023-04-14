const WIDTH: usize = 8;
const HEIGHT: usize = 8;

#[derive(Debug)]
pub struct Board {
    pub cells: [[u8; WIDTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [[0; WIDTH]; HEIGHT],
        }
    }
}