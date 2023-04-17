use crate::crop::types::CropType;
use crate::crop::types::Crop;
use crate::game::context::Context;
use crate::game::board::find_building;
use crate::objects::{
    build::Building,
    types::{BuildingType, SiloContent},
    silo::Silo,
};
use crate::types::Pos;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Farm {
    pub crop: Option<CropType>,
    pub price: u32,
    pub size: Pos,
    pub name: String,
}

impl Farm {
    pub fn new() -> Farm {
        Farm {
            crop: None,
            price: 1,
            size: Pos { x: 1, y: 1 },
            name: "Farm".to_string(),
        }
    }

    pub fn new_with_crop(crop: CropType) -> Farm {
        Farm {
            crop: Some(crop),
            price: 100,
            size: Pos { x: 1, y: 1 },
            name: "Farm".to_string(),
        }
    }

    pub fn plant(&mut self, crop: CropType) {
        self.crop = Some(crop);
    }

    pub fn collect(&mut self, ctx: &mut Context) -> Option<CropType> {
        let crop = self.crop.clone();
        let silo = match find_building(BuildingType::Silo(Silo::new()), ctx) {
            Some(BuildingType::Silo(silo)) => silo,
            _ => {
                println!("No silo found");
                return None;
            },
        };

        if silo.capacity - silo.capacity_used < 2 {
            return None;
        }
        self.crop = None;
        if crop.is_some() {
            let silo_content = match crop.clone().unwrap() {
                CropType::Wheat(wheat) => SiloContent::Wheat(wheat),
                _ => {
                    println!("Not implemented yet");
                    return None;
                },
            };

            silo.add(silo_content, 2);
            ctx.player.time += crop.clone().unwrap().duration();
        }
        return crop;
    }
}

impl fmt::Display for Farm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.crop {
            Some(crop) => write!(f, "Farm {{ crop: {}, price: {}, size: {} }}", crop, self.price, self.size),
            None => write!(f, "Farm {{ crop: None, price: {}, size: {} }}", self.price, self.size),
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

    fn name(&self) -> String {
        return self.name.clone();
    }
}