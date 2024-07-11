use std::{
    env, fs, io::{self, Write}, process::Command
};
use itertools::Itertools;

fn main() {
    let mut status_code: Option<i32> = None;

    let path = decode_path();

    while status_code.is_none() {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // remove newline
        input.pop();

        handle_command(&mut input, &path, &mut status_code);
    }

    std::process::exit(if status_code.is_some() { status_code.unwrap() } else { 0 });
}


fn handle_command(input: &mut String, path: &Vec<String>, return_status_code: &mut Option<i32>) {
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
                if ["exit", "echo", "type", "pwd", "cd"].contains(&argument) {
                    println!("{} is a shell builtin", argument);
                    continue;
                }

                let executable_path = search_for_executable(path, argument.to_owned());

                if executable_path.is_some() {
                    println!("{} is {}", argument, executable_path.unwrap());
                } else {
                    println!("{}: not found", argument)
                }
            }
        },
        "pwd" => {
            println!("{}", env::current_dir().unwrap().display());
        },
        "cd" => {
            let new_working_dir = arguments.nth(0).expect("cd requires a path. Syntax: cd <path>");

            let set_working_dir_result = env::set_current_dir(new_working_dir);

            if set_working_dir_result.is_err() {
                println!("cd: {}: No such file or directory", new_working_dir);
            }
        },
        _ => {
            let executable_path = search_for_executable(path, command_name.unwrap().to_owned());

            if executable_path.is_some() {
                let mut command_object = Command::new(executable_path.unwrap());

                command_object.args(arguments);

                let output = command_object.output().expect("There was an error");

                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            } else {
                println!("{}: command not found", command_name.unwrap());
            }
        }
    }
}


fn decode_path() -> Vec<String> {
    let path = env::var("PATH");

    if path.is_err() {
        return vec![];
    }

    path.unwrap().split(":").map(|s| s.to_string()).collect()
}


fn search_for_executable(paths: &Vec<String>, executable_name: String) -> Option<String> {
    for path in paths {
        let file_paths_result = fs::read_dir(path);

        if file_paths_result.is_err() {
            continue;
        }

        let file_paths = file_paths_result.unwrap();

        for file_path_result in file_paths {
            let file_path = file_path_result.unwrap().path();
            if file_path.display().to_string().split("/").last().unwrap() == executable_name {
                return Some(file_path.display().to_string());
            }
        }
    }

    None
}