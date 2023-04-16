#[derive(Clone)]
pub struct Wheat {
    pub name: String,
    pub price: u32,
}

impl Wheat {
    pub fn new() -> Wheat {
        Wheat {
            name: "Wheat".to_string(),
            price: 10,
        }
    }
}
