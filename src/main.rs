mod game;

use game::context::Context;

fn main() {
    let context = Context::new();

    println!("{:?}", context);
}