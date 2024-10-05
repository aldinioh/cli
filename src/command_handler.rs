use std::{io, process::Output};

pub enum CommandType {
    Echo,
    Cat,
    Ls,
    Find,
    Group,
    Help
}

pub struct CliCommand {
    input: String,
    command: Option<CommandType>,
}

impl CliCommand {
    fn run_command(&mut self) {
        self.command = Self::get_commmand(&self);
    }
    
    fn get_commmand(&self) -> Option<CommandType> {
        let input_iter = self.input.split_whitespace().next().unwrap();
        match input_iter {
            "echo" => return Some(CommandType::Echo),
            "cat" => return Some(CommandType::Cat),
            "ls" => return Some(CommandType::Ls),
            "find" => return Some(CommandType::Find),
            "group" => return Some(CommandType::Group),
            "help" => return Some(CommandType::Help),
            _ => {
                self.print_to_console(std::format!(
r#"----------------------------------------
`{}` is an invalid command! 

Use `help` to see available commands.
----------------------------------------"#, 
                    input_iter));

                return None;
            }
        }
    }

    fn print_to_console(&self, message: String) {
        if !message.is_empty() {
            println!("{}", message);
        }
        
        println!()   
    }
}

pub fn capture_cli() {
    let mut command = CliCommand {
        input: String::from(get_input()),
        command: None
    };

    if command.input.is_empty() {
        command.print_to_console(String::from("No input found."));
    }

    command.run_command();
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid");
    return input;
}