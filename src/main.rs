use console::capture_cli;

pub mod console;

fn main() {
    println!("Command Prompt");
    loop { 
        capture_cli();
    }
}