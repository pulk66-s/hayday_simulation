use std::fmt;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Wheat {
    pub name: String,
    pub price: u32,
    pub duration: u32,
}

impl Wheat {
    pub fn new() -> Wheat {
        Wheat {
            name: "Wheat".to_string(),
            price: 10,
            duration: 10,
        }
    }
}

impl fmt::Display for Wheat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, price: {}", self.name, self.price)
    }
}