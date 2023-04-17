use std::fmt;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ChickenFood {

}

impl ChickenFood {
    pub fn new() -> ChickenFood {
        ChickenFood {}
    }
}

impl fmt::Display for ChickenFood {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChickenFood")
    }
}