mod game;
mod objects;
mod crop;
mod types;
mod menus;
mod tty;

use game::context::Context;
use objects::farm::Farm;
use crop::wheat::Wheat;
use tty::terminal::{Terminal, update};

fn main() {
    let context = Context::new();
    let farm = Farm::new(Wheat::new());
    let mut terminal = Terminal::new();

    while terminal.wantLeave == false {
        update(&mut terminal);
    }
}
