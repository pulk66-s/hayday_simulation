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
    types::BuildingType,
};

fn init_context() -> Context {
    let mut context = Context::new();

    for _ in 0..6 {
        context.board.farms.push(Farm::new());
    }
    context.board.buildings.push(BuildingType::Silo(Silo::new()));
    context.board.buildings.push(BuildingType::Barn(Barn::new()));
    context.board.buildings.push(BuildingType::ChickenCoop(ChickenCoop::new()));
    return context;
}

fn main() {
    let mut context = init_context();
    let mut terminal = Terminal::new();

    while terminal.want_leave == false {
        update(&mut terminal, &mut context);
    }
}
