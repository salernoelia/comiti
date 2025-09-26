enum Command {
    Init,
    Add,
    List,
    Remove,
    Unknown(String),
}

impl Command {
    fn print_menu() {
        println!("Following options:");
        for cmd in Self::all() {
            println!("{}", cmd.description());
        }
    }

    fn from_str(input: &str) -> Self {
        match input {
            "init" => Command::Init,
            "add" => Command::Add,
            "list" => Command::List,
            "remove" => Command::Remove,
            other => Command::Unknown(other.to_string()),
        }
    }

    fn description(&self) -> String {
        match self {
            Command::Init => "init   - initialize current dir".to_string(),
            Command::Add => "add    - add current dir".to_string(),
            Command::List => "list   - list all dirs with index".to_string(),
            Command::Remove => "remove - remove current or by index".to_string(),
            Command::Unknown(cmd) => format!("Unknown command: {}", cmd),
        }
    }

    fn all() -> Vec<Command> {
        vec![Command::Init, Command::Add, Command::List, Command::Remove]
    }
}

pub fn init() {
    use std::io::{Write, stdin, stdout};
    let mut user_input: String = String::new();

    Command::print_menu();

    let _ = stdout().flush();

    stdin()
        .read_line(&mut user_input)
        .expect("Did not enter a correct string");

    println!("You typed: {}", user_input);

    Command::from_str(user_input.trim());
}
