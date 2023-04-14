use crate::crop::attr::Crop;
use crate::game::context::Context;
use crate::objects::build::Building;
use crate::types::Pos;

#[derive(Debug)]
pub struct Farm<T: Crop> {
    pub crop: T,
    pub price: u32,
    pub size: Pos,
}

impl<T: Crop> Farm<T> {
    pub fn new(crop: T) -> Farm<T> {
        Farm {
            crop,
            price: 100,
            size: Pos { x: 1, y: 1 },
        }
    }
}

impl<T: Crop> Building for Farm<T> {
    fn build(&self, ctx: &mut Context) -> bool {
        if ctx.player.money < self.price {
            return false;
        }
        ctx.player.money -= self.price;
        return true;
    }
}