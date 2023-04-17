use crate::game::{
    board::Board,
    player::Player,
};
use crate::animals::{
    types::AnimalType,
    chicken::Chicken,
};
use crate::menus::market::Market;
use crate::objects::{
    types::BuildingType,
    build::Building
};

#[derive(Clone)]
pub struct Context {
    pub board: Board,
    pub player: Player,
    pub market: Market,
}

fn add_chicken(ctx: &mut Context, chicken: Chicken) -> bool {
    let buildings = &mut ctx.board.buildings;
    let coops = buildings.iter_mut().filter(|b| b.name() == "Chicken Coop").collect::<Vec<&mut BuildingType>>();

    for coop in coops {
        if let BuildingType::ChickenCoop(chicken_coop) = coop {
            if chicken_coop.curr_capacity < chicken_coop.max_capacity {
                chicken_coop.chickens.push(chicken);
                chicken_coop.curr_capacity += 1;
                return true;
            }
        }
    }
    return false;
}

impl Context {
    pub fn new() -> Context {
        Context {
            board: Board::new(),
            player: Player::new(),
            market: Market::new(),
        }
    }

    pub fn add_animal(&mut self, animal: AnimalType) -> bool {
        match animal {
            AnimalType::Chicken(chicken) => add_chicken(self, chicken),
            _ => false,
        }
    }

    pub fn feed_animal(&mut self, animal: &str) {
        match animal {
            "chicken" => println!("Feeding chicken"),
            _ => println!("Unknown animal"),
        }
    }
}