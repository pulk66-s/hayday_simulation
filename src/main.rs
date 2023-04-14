mod game;
mod objects;
mod crop;
mod types;

use game::context::Context;
use objects::farm::Farm;
use crop::wheat::Wheat;

fn main() {
    let context = Context::new();
    let farm = Farm::new(Wheat::new());

    println!("{:?}", context);
    println!("{:?}", farm);
}
