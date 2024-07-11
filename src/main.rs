use std::io::{self, Write};
use itertools::Itertools;

fn main() {
    let mut status_code: Option<i32> = None;

    while status_code.is_none() {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // remove newline
        input.pop();

        handle_command(&mut input, &mut status_code);
    }

    std::process::exit(if status_code.is_some() { status_code.unwrap() } else { 0 });
}


fn handle_command(input: &mut String, return_status_code: &mut Option<i32>) {
    // get the command name and arguments
    let command_name = input.split(' ').nth(0);
    let mut arguments = input.split(' ').skip(1);

    if command_name.is_none() || command_name.unwrap().len() == 0 {
        return;
    }

    match command_name.unwrap() {
        "exit" => {
            let status_code = arguments.nth(0);

            if status_code.is_none() || status_code.unwrap().parse::<i32>().is_err() {
                *return_status_code = Some(0);
            } else {
                *return_status_code = Some(status_code.unwrap().parse::<i32>().unwrap());
            }
        },
        "echo" => {
            println!("{}", arguments.join(" "));
        },
        "type" => {
            for argument in arguments {
                if ["exit", "echo", "type"].contains(&argument) {
                    println!("{} is a shell builtin", argument);
                } else {
                    println!("{}: not found", argument);
                }
            }
        }
        _ => {
            println!("{}: command not found", command_name.unwrap());
        }
    }
}