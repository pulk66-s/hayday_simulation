use crate::game::context::Context;

fn feed_help() {
    println!("Usage: feed <animal> <amount>");
    println!("Animal: chicken");
}

pub fn feed_cmd(cmd: String, ctx: &mut Context) {
    let words = cmd.split_whitespace().collect::<Vec<&str>>();

    match words.len() {
        1 => feed_help(),
        2 => match words[1] {
            "help" => feed_help(),
            animal => ctx.feed_animal(animal),
        },
        _ => println!("Too many arguments"),
    }
}