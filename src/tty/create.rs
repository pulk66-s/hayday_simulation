use crate::game::context::Context;
use crate::objects::animal_food::chicken::ChickenFood;
use crate::objects::types::BuildingType;
use crate::objects::provenderie::{ProvenderieFood, Provenderie};
use crate::objects::build::Building;

fn create_help() {
    println!("Usage: create <category> <item> <amount>");
    println!("Categories: animal_food");
}

fn animal_food_help() {
    println!("Usage: create animal_food <item>");
    println!("Items: chicken");
}

fn create_chicken_food(context: &mut Context, amount: u32) {
    let mut buildings = context.board.buildings.iter_mut().filter(|b| b.name() == "Provenderie").collect::<Vec<&mut BuildingType>>();
    let provenderies = buildings.iter_mut().map(|b| match b {
        BuildingType::Provenderie(provenderie) => provenderie,
        _ => panic!("Provenderie not found"),
    }).collect::<Vec<&mut Provenderie>>();

    for provenderie in provenderies {
        if provenderie.have_enough_space(amount) {
            provenderie.create_food(ProvenderieFood::Chicken(ChickenFood::new()));
            return;
        }
    }
}

pub fn create_cmd(cmd: String, context: &mut Context) {
    let words = cmd.split_whitespace().collect::<Vec<&str>>();

    match words.len() {
        1 => create_help(),
        2 => match words[1] {
            "animal_food" => animal_food_help(),
            _ => println!("Unknown category: {}", words[1]),
        },
        3 => match words[1] {
            "animal_food" => match words[2] {
                "chicken" => create_chicken_food(context, 1),
                _ => println!("Unknown item: {}", words[2]),
            },
            _ => println!("Unknown category: {}", words[1]),
        },
        _ => println!("Usage: create <category> <item>"),
    }
}