pub enum CommandType {
    Echo,
    Cat,
    Ls,
    Find,
    Group
}

pub struct CliCommand {
    command: CommandType,
    input: String,
    output: ()
}

impl CliCommand {
    fn run_command(&self) {
        
    }
}