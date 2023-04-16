use crate::objects::farm::Farm;

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

pub struct Board {
    pub cells: [[u8; WIDTH]; HEIGHT],
    pub farms: Vec<Farm>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [[0; WIDTH]; HEIGHT],
            farms: Vec::new(),
        }
    }
}
