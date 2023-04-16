use crate::game::context::Context;
use crate::crop::types::CropType;
use crate::crop::wheat::Wheat;

fn plant_help() {
    println!("Usage: plant [help]");
}

fn find_and_plant(ctx: &mut Context, crop: CropType) {
    for farm in ctx.board.farms.iter_mut() {
        if farm.crop.is_none() {
            farm.plant(crop);
            return;
        }
    }
    println!("No free farm found");
}

fn plant(ctx: &mut Context, v: &str) {
    match v.to_lowercase().as_str() {
        "wheat" => find_and_plant(ctx, CropType::Wheat(Wheat::new())),
        _ => println!("Unknown crop: {}", v),
    }
}

pub fn plant_cmd(command: String, ctx: &mut Context) {
    let words = command.split_whitespace().collect::<Vec<&str>>();

    match words.len() {
        1 => plant_help(),
        2 => {
            match words[1] {
                "help" => plant_help(),
                v => plant(ctx, v),
            }
        },
        _ => println!("Too many arguments"),
    }
}