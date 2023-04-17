use std::fmt;
use crate::animals::{
    attr::Animal,
    types::AnimalType,
};
use crate::game::context::Context;

#[derive(Clone)]
pub struct Chicken {

}

impl fmt::Display for Chicken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chicken")
    }
}

impl Chicken {
    pub fn new() -> Chicken {
        Chicken {}
    }
}

impl Animal for Chicken {
    fn buy(&self, context: &mut Context) -> bool {
        if context.player.money >= 10 {
            context.player.money -= 10;
            context.add_animal(AnimalType::Chicken(self.clone()))
        } else {
            false
        }
    }
}