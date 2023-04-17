use crate::game::context::Context;
use crate::objects::types::BuildingType;

fn collect_help() {
    println!("Usage: collect <category>");
    println!("Category: provenderie");
}

fn collect_provenderie(ctx: &mut Context) {
    ctx.board.buildings = ctx.board.buildings.clone().iter_mut()
        .map(|b| match b {
            BuildingType::Provenderie(provenderie) => {
                provenderie.collect_one(ctx);
                BuildingType::Provenderie(provenderie.clone())
            },
            b => b.clone(),
        })
        .collect::<Vec<BuildingType>>();
}

pub fn collect_cmd(command: String, ctx: &mut Context) {
    let words = command.split_whitespace().collect::<Vec<&str>>();

    match words.len() {
        1 => collect_help(),
        2 => match words[1] {
            "help" => collect_help(),
            "provenderie" => collect_provenderie(ctx),
            _ => println!("Unknown command: {}", command),
        },
        _ => println!("Unknown command: {}", command),
    }
}