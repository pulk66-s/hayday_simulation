use crate::game::context::Context;

use super::{build::Building, types::BuildingType, animal_food::chicken::ChickenFood};

use std::fmt;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ProvenderieFood {
    Chicken(ChickenFood)
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Provenderie {
    pub name: String,
    pub price: u32,
    pub queue: Vec<ProvenderieFood>,
    pub queue_max_size: u32,
}

impl Provenderie {
    pub fn new() -> Provenderie {
        Provenderie {
            name: "Provenderie".to_string(),
            price: 100,
            queue: Vec::new(),
            queue_max_size: 3,
        }
    }

    pub fn create_food(&mut self, food_type: ProvenderieFood) -> bool {
        if self.queue.len() < self.queue_max_size as usize {
            self.queue.push(food_type);
            return true;
        }
        return false;
    }

    pub fn have_enough_space(&self, amount: u32) -> bool {
        return self.queue.len() + amount as usize <= self.queue_max_size as usize;
    }
}

impl Building for Provenderie {
    fn build(&self, ctx: &mut Context) -> bool {
        if ctx.player.money < self.price {
            return false;
        }
        ctx.player.money -= self.price;
        ctx.board.buildings.push(BuildingType::Provenderie(self.clone()));
        return true;
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl fmt::Display for Provenderie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Provenderie {{ name: {}, price: {}, self.queue_max_size: {}, contennt: {:?} }}", self.name, self.price, self.queue_max_size, self.queue)
    }
}
