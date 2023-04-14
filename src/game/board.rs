use crate::{objects::farm::Farm, crop::wheat::Wheat};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

#[derive(Debug)]
pub struct Farms {
    wheat: Vec<Farm<Wheat>>,
}

#[derive(Debug)]
pub struct Board {
    pub cells: [[u8; WIDTH]; HEIGHT],
    pub farms: Farms,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [[0; WIDTH]; HEIGHT],
            farms: Farms::new(),
        }
    }
}

impl Farms {
    pub fn new() -> Farms {
        Farms {
            wheat: Vec::new(),
        }
    }
}