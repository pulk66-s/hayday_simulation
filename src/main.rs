mod game;
mod objects;
mod crop;
mod types;
mod menus;
mod tty;

use game::context::Context;
use objects::farm::Farm;
use tty::terminal::{Terminal, update};

fn main() {
    let mut context = Context::new();
    let mut terminal = Terminal::new();

    while terminal.want_leave == false {
        update(&mut terminal, &mut context);
    }
}
