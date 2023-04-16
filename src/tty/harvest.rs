use crate::game::context::Context;
use crate::crop::types::Crop;

fn harvest_help() {
    println!("harvest [plant | all]");
}

fn harvest(ctx: &mut Context, plant: &str, max_qte: Option<u32>) {
    let plant = plant.to_string();
    let mut plant_found = false;
    let mut farms = ctx.board.farms.clone();
    let mut qte = 0;

    for farm in farms.iter_mut() {
        let crop = farm.crop.clone();

        if crop.is_none() {
            continue;
        }
        if crop.unwrap().name().to_lowercase() == plant.to_lowercase() {
            plant_found = true;
            farm.collect(ctx);
        }
        qte += 1;
        match max_qte {
            Some(max_qte) => if qte >= max_qte {
                break;
            },
            _ => (),
        }
    }
    if !plant_found {
        println!("No {} found", plant);
    }
    ctx.board.farms = farms;
}

fn harvest_all(ctx: &mut Context) {
    let mut farms = ctx.board.farms.clone();

    for farm in farms.iter_mut() {
        if farm.crop.is_some() {
            farm.collect(ctx);
        }
    }
    for farm in farms.iter_mut() {
        if farm.crop.is_some() {
            println!("You can't harvest all plants at once");
        }
    }
    ctx.board.farms = farms;
}

pub fn harvest_cmd(command: String, ctx: &mut Context) {
    let words = command.split_whitespace().collect::<Vec<&str>>();

    match words.len() {
        1 => harvest_help(),
        2 => match words[1] {
            "all" => harvest_all(ctx),
            "help" => harvest_help(),
            plant => harvest(ctx, plant, None),
        },
        _ => harvest_help(),
    }
}