use std::io::Write;

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
    println!("Available commands: exit, help");
}

fn parse_command(command: String, terminal: &mut Terminal) {
    let command = command.trim();

    match command {
        "exit" => terminal.want_leave = true,
        "help" => print_help(),
        _ => println!("Unknown command: {}", command),
    }
}

pub fn update(terminal: &mut Terminal) {
    print!("{}", terminal.prompt);
    std::io::stdout().flush().expect("Failed to flush output"); // flush the output to make sure it appears before reading input

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    parse_command(input, terminal);
}