use crate::crop::types::CropType;
use crate::game::context::Context;
use crate::objects::build::Building;
use crate::types::Pos;
use std::fmt;

#[derive(Clone)]
pub struct Farm {
    pub crop: Option<CropType>,
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

    pub fn new_with_crop(crop: CropType) -> Farm {
        Farm {
            crop: Some(crop),
            price: 100,
            size: Pos { x: 1, y: 1 },
        }
    }

    pub fn plant(&mut self, crop: CropType) {
        self.crop = Some(crop);
    }

    pub fn collect(&mut self) -> Option<CropType> {
        let crop = self.crop.clone();
        self.crop = None;
        return crop;
    }
}

impl fmt::Display for Farm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.crop {
            Some(crop) => write!(f, "Farm: crop: {}, price: {}, size: {}", crop, self.price, self.size),
            None => write!(f, "Farm: crop: None, price: {}, size: {}", self.price, self.size),
        }
    }
}

impl Building for Farm {
    fn build(&self, ctx: &mut Context) -> bool {
        if ctx.player.money < self.price {
            return false;
        }
        ctx.player.money -= self.price;
        ctx.board.farms.push(self.clone());
        return true;
    }
}