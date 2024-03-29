use crate::animals::chicken::Chicken;
use crate::objects::{
    build::Building,
    types::BuildingType,
};
use std::fmt;
use crate::types::Pos;
use crate::game::context::Context;

#[derive(Clone, Debug)]
pub struct ChickenCoop {
    pub name: String,
    pub level: u8,
    pub max_capacity: u8,
    pub curr_capacity: u8,
    pub price: u32,
    pub size: Pos,
    pub chickens: Vec<Chicken>,
}

impl ChickenCoop {
    pub fn new() -> ChickenCoop {
        ChickenCoop {
            name: "Chicken Coop".to_string(),
            level: 1,
            max_capacity: 6,
            curr_capacity: 0,
            price: 100,
            size: Pos { x: 2, y: 2 },
            chickens: Vec::new(),
        }
    }
}

impl Building for ChickenCoop {
    fn build(&self, ctx: &mut Context) -> bool {
        if ctx.player.money < self.price {
            return false;
        }
        ctx.player.money -= self.price;
        ctx.board.buildings.push(BuildingType::ChickenCoop(self.clone()));
        return true;
    }

    fn name(&self) -> String {
        return self.name.clone();
    }
}

impl fmt::Display for ChickenCoop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut content = format!("Chicken Coop: {{ level: {}, max_capacity: {}, curr_capacity: {}, price: {}, size: {}, chickens: {{ ", self.level, self.max_capacity, self.curr_capacity, self.price, self.size);

        for chicken in &self.chickens {
            content += &format!("{}, ", chicken);
        }
        content += "} }";
        write!(f, "{}", content)
    }
}