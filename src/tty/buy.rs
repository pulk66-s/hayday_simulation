use crate::game::context::Context;

fn buy_help() {
    println!("Usage: buy <category> <item>");
    println!("Categories: farming");
}

fn buy_help_category(category: String) {
    match category.as_str() {
        "farming" => {
            println!("Usage: buy farming <item>");
            println!("Items: land");
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
}