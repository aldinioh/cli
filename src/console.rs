use std::io;
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
        self.command = Self::get_command(&self);
    }
    
    fn get_command(&self) -> Option<CommandType> {
        match self.input.trim().split_whitespace().next() {
            Some(command) => {
                match command {
                    "echo" => {
                        command_executer::echo_command(&self);
                        Some(CommandType::Echo) 
                    },
                    "cat" => {

                        Some(CommandType::Cat) 
                    }
                    "ls" => {

                        Some(CommandType::Ls)
                    },
                    "find" => {

                        Some(CommandType::Find)
                    },
                    "group" => {

                        Some(CommandType::Group)
                    },
                    "help" => {
                        command_executer::help_command(&self);
                        Some(CommandType::Help)
                    },
                    _ => {
                        // Invalid command handling
                        self.print_to_console(format!(
r#"----------------------------------------
`{}` is an invalid command! 
                        
Use `help` to see available commands.
----------------------------------------"#,
                            command
                        ));
                        return None;
                    }
                }
            }
            None => return None
        }
    }
    
    fn print_to_console(&self, message: String) {
        if !message.is_empty() {
            println!("{}", message);
        }
        
        println!()   
    }

    pub fn get_input() -> String {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(_) => return String::new()
        }
    }
}

pub fn capture_cli() {
    let command = analyse_command();
    command.get_command();
    
    if command.input.is_empty() {
        command.print_to_console(String::from(
r#"----------------------------------------
No input found!

Use `help` to see available commands.
----------------------------------------"#, 
        ));
        return;
    }
}

fn analyse_command() -> CliCommand {
    let input = CliCommand::get_input();
    CliCommand {
        input,
        command: None, // Command will be determined later
    }
}

pub mod command_executer {
    use super::CliCommand;

    pub fn echo_command(command: &CliCommand) {
        let new_output = command.input.split_whitespace().filter(|&element| element != "echo").collect::<Vec<&str>>().join(" ");
        command.print_to_console(format!("{}", new_output));
    }

    pub fn help_command(command: &CliCommand) {
        command.print_to_console(String::from(
r#"----------------------------------------
Available commands:

`echo` - repeats the input provided
`cat` - concatenates files together
`ls` - lists directories 
`find` - locates files or directories provided
`grep` - matches text in files provided
`help` - console help text 
----------------------------------------"#, 
        ));
    }
}