use crate::game::context::Context;

fn inv_help() {
    println!("Usage: inv <category> <item>");
    println!("Categories: farm");
}

fn farm_inv(ctx: &mut Context) {
    println!("Farm Inventory:");
    for farm in &ctx.board.farms {
        println!("Farm: {}", farm);
    }
}

fn building_inv(ctx: &mut Context) {
    println!("Building Inventory:");
    for building in &ctx.board.buildings {
        println!("Building: {}", building);
    }
}

fn inv_category(category: &str, ctx: &mut Context) {
    match category {
        "farm" => farm_inv(ctx),
        "building" => building_inv(ctx),
        _ => println!("Unknown category: {}", category),
    }
}

fn all_inv(ctx: &mut Context) {
    println!("All Inventory:");
    inv_category("farm", ctx);
    inv_category("building", ctx)
}

pub fn inv_cmd(command: String, ctx: &mut Context) {
    let words = command.split_whitespace().collect::<Vec<&str>>();

    match words.len() {
        1 => all_inv(ctx),
        2 => {
            match words[1] {
                "help" => inv_help(),
                category => inv_category(category, ctx),
            }
        },
        _ => println!("Too many arguments"),
    }
}