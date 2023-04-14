use crate::crop::attr::Crop;
use crate::game::context::Context;

#[derive(Debug)]
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

impl Crop for Wheat {
    fn harvest(&self, context: &mut Context) -> bool {
        context.player.money += self.price;
        true
    }

    fn new() -> Wheat {
        Wheat::new()
    }
}