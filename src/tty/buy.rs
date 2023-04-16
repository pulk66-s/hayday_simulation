use crate::game::context::Context;
use crate::objects::build::Building;

fn buy_help() {
    println!("Usage: buy <category> <item>");
    println!("Categories: farm");
}

fn buy_help_category(category: String) {
    match category.as_str() {
        "farm" => {
            println!("Usage: buy farm <item>");
            println!("Items: land");
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

fn buy_item(category: String, item: String, context: &mut Context) {
    match category.as_str() {
        "farm" => {
            match item.as_str() {
                "land" => buy_farmland(context),
                _ => println!("Unknown item: {}", item),
            }
        }
        _ => println!("Unknown category: {}", category),
    }
}

pub fn buy_cmd(cmd: String, context: &mut Context) {
    let words = cmd.split_whitespace().collect::<Vec<&str>>();

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