mod game;
mod objects;
mod crop;
mod types;
mod menus;
mod tty;
mod animals;

use game::context::Context;
use tty::terminal::{Terminal, update};
use objects::{
    farm::Farm,
    silo::Silo,
    barn::Barn,
    coop::chicken::ChickenCoop,
    types::{BuildingType, BarnContent}, animal_food::chicken::ChickenFood,
};
use animals::{
    types::AnimalType,
    chicken::Chicken,
};

fn init_barn() -> BuildingType {
    let mut barn = Barn::new();

    barn.add(BarnContent::ChickenFood(ChickenFood::new()), 3);
    return BuildingType::Barn(barn);
}

fn init_context() -> Context {
    let mut context = Context::new();

    for _ in 0..6 {
        context.board.farms.push(Farm::new());
    }
    context.board.buildings.push(BuildingType::Silo(Silo::new()));
    context.board.buildings.push(init_barn());
    context.board.buildings.push(BuildingType::ChickenCoop(ChickenCoop::new()));
    for _ in 0..3 {
        context.add_animal(AnimalType::Chicken(Chicken::new()));
    }
    return context;
}

fn main() {
    let mut context = init_context();
    let mut terminal = Terminal::new();

    while terminal.want_leave == false {
        update(&mut terminal, &mut context);
    }
}
