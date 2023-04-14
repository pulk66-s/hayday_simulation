use std::io::Write;

#[derive(Debug)]
pub struct Terminal {
    pub wantLeave: bool,
    pub prompt: String,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            wantLeave: false,
            prompt: "> ".to_string(),
        }
    }
}

fn printHelp() {
    println!("Available commands: exit, help");
}

fn parseCommand(command: String, terminal: &mut Terminal) {
    let command = command.trim();

    match command {
        "exit" => terminal.wantLeave = true,
        "help" => printHelp(),
        _ => println!("Unknown command: {}", command),
    }
}

pub fn update(terminal: &mut Terminal) {
    print!("{}", terminal.prompt);
    std::io::stdout().flush().expect("Failed to flush output"); // flush the output to make sure it appears before reading input

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    parseCommand(input, terminal);
}