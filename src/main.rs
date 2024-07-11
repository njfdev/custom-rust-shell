#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // remove newline
        input.pop();

        // get the command name (1st argument)
        let command_name = input.split(' ').nth(0);

        // If command is not handled, print out a message
        if command_name.is_some() && command_name.unwrap().len() > 0 {
            println!("{}: command not found", command_name.unwrap());
        }
    }
}
