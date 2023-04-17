use crate::game::context::Context;
use crate::objects::build::Building;
use crate::animals::attr::Animal;

fn buy_help() {
    println!("Usage: buy <category> <item>");
    println!("Categories: farm, animal, building");
}

fn buy_help_category(category: String) {
    match category.as_str() {
        "farm" => {
            println!("Usage: buy farm <item>");
            println!("Items: land, chicken");
        }
        _ => println!("Unknown category: {}", category),
    }
}

fn buy_farmland(context: &mut Context) {
    let farm = context.market.farming.get_farm();

    if farm.build(context) {
        println!("Bought farmland!");
    } else {
        println!("Not enough money!");
    }
}

fn buy_chicken_coop(context: &mut Context) {
    let coop = context.market.farming.get_chicken_coop();

    if coop.build(context) {
        println!("Bought chicken coop!");
    } else {
        println!("Not enough money!");
    }
}

fn buy_chicken(context: &mut Context) {
    let chicken = context.market.animals.get_chicken();

    if chicken.buy(context) {
        println!("Bought chicken!");
    } else {
        println!("Not enough money!");
    }
}

fn buy_provenderie(context: &mut Context) {
    let provenderie = context.market.buildings.get_provenderie();

    if provenderie.build(context) {
        println!("Bought provenderie!");
    } else {
        println!("Not enough money!");
    }
}

fn buy_item(category: String, item: String, context: &mut Context) {
    match category.as_str() {
        "farm" => match item.as_str() {
            "land" => buy_farmland(context),
            "chicken" => buy_chicken_coop(context),
            _ => println!("Unknown item: {}", item),
        },
        "animal" => match item.as_str() {
            "chicken" => buy_chicken(context),
            _ => println!("Unknown item: {}", item),
        },
        "building" => match item.as_str() {
            "provenderie" => buy_provenderie(context),
            _ => println!("Unknown item: {}", item),
        },
        _ => println!("Unknown category: {}", category),
    }
}

pub fn buy_cmd(cmd: String, context: &mut Context) {
    let words = cmd.split_whitespace().collect::<Vec<&str>>();

    if words.len() == 1 {
        buy_help();
        return;
    }
    if words.len() < 3 {
        if words[1] == "help" {
            buy_help();
            return;
        }
        println!("Usage: buy <category> <item>");
        return;
    }
    if words[2] == "help" {
        buy_help_category(words[1].to_string());
        return;
    }
    buy_item(words[1].to_string(), words[2].to_string(), context);
}