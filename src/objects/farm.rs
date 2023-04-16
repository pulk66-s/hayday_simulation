use crate::crop::attr::Crop;
use crate::game::context::Context;
use crate::objects::build::Building;
use crate::types::Pos;

pub struct Farm {
    pub crop: Option<Box<dyn Crop>>,
    pub price: u32,
    pub size: Pos,
}

impl Farm {
    pub fn new() -> Farm {
        Farm {
            crop: None,
            price: 100,
            size: Pos { x: 1, y: 1 },
        }
    }
}

impl Building for Farm {
    fn build(&self, ctx: &mut Context) -> bool {
        if ctx.player.money < self.price {
            return false;
        }
        ctx.player.money -= self.price;
        return true;
    }
}