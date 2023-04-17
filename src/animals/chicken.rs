use std::fmt;
use crate::animals::{
    attr::Animal,
    types::AnimalType,
};
use crate::game::{
    context::Context,
    board::find_building,
};
use crate::objects::{
    types::BuildingType,
    types::BarnContent,
    animal_food::chicken::ChickenFood,
    barn::Barn
};

#[derive(Clone, Debug)]
pub struct Chicken {
    pub price: u32,
    pub feed: bool,
    pub receive_time: u32,
}

impl fmt::Display for Chicken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chicken {{ price: {} }}", self.price)
    }
}

impl Chicken {
    pub fn new() -> Chicken {
        Chicken {
            price: 10,
            feed: false,
            receive_time: 0,
        }
    }
}

impl Animal for Chicken {
    fn buy(&self, context: &mut Context) -> bool {
        if context.player.money >= self.price {
            if context.add_animal(AnimalType::Chicken(self.clone())) {
                context.player.money -= self.price;
                return true;
            }
            return false
        } else {
            return false
        }
    }

    fn feed(&mut self, ctx: &mut Context) -> bool {
        let building = match find_building(BuildingType::Barn(Barn::new()), ctx) {
            Some(barn) => barn,
            None => return false,
        };
        let barn = match building {
            BuildingType::Barn(ref mut barn) => barn,
            _ => return false,
        };

        if barn.have(BarnContent::ChickenFood(ChickenFood::new()), 1) {
            barn.remove(BarnContent::ChickenFood(ChickenFood::new()), 1);
            self.feed = true;
            return true;
        }
        return false;
    }

    fn receive(&mut self, ctx: &mut Context) -> bool {
        false
    }
}