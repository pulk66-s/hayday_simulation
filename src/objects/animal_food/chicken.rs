use std::fmt;

use super::attr::AnimalFood;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ChickenFood {
    pub time: u32,
}

impl ChickenFood {
    pub fn new() -> ChickenFood {
        ChickenFood {
            time: 1,
        }
    }
}

impl fmt::Display for ChickenFood {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChickenFood")
    }
}

impl AnimalFood for ChickenFood {
    fn time(&self) -> u32 {
        self.time
    }
}