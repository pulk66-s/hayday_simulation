use std::io::Write;
use crate::game::context::Context;
use crate::tty::{
    buy::buy_cmd,
    inv::inv_cmd,
    feed::feed_cmd,
    plant::plant_cmd,
    create::create_cmd,
    harvest::harvest_cmd,
    collect::collect_cmd,
};

#[derive(Debug)]
pub struct Terminal {
    pub want_leave: bool,
    pub prompt: String,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            want_leave: false,
            prompt: "> ".to_string(),
        }
    }
}

fn print_help() {
    println!("Available commands: exit, help, status, buy, inv, plant, harverst, feed, create");
}

fn print_player_status(context: &mut Context) {
    println!("Player status: {:?}", context.player);
}

fn parse_command(command: String, terminal: &mut Terminal, context: &mut Context) {
    let command = command.trim();
    let first_word = command.split_whitespace().next().unwrap_or("");

    match first_word {
        "exit" => terminal.want_leave = true,
        "help" => print_help(),
        "status" => print_player_status(context),
        "buy" => buy_cmd(command.to_string(), context),
        "inv" => inv_cmd(command.to_string(), context),
        "plant" => plant_cmd(command.to_string(), context),
        "harvest" => harvest_cmd(command.to_string(), context),
        "feed" => feed_cmd(command.to_string(), context),
        "create" => create_cmd(command.to_string(), context),
        "collect" => collect_cmd(command.to_string(), context),
        _ => println!("Unknown command: {}", command),
    }
}

pub fn update(terminal: &mut Terminal, context: &mut Context) {
    print!("{}", terminal.prompt);
    std::io::stdout().flush().expect("Failed to flush output"); // flush the output to make sure it appears before reading input

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    parse_command(input, terminal, context);
}