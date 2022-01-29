use std::{
    io::{stdin, stdout, Write},
    process::Command,
};

fn main() {
    loop {
        print!(">> ");
        stdout().flush().expect("Cant flush stdout buffer");
        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Unable to read user input");

        let command_to_execute = user_input.trim();
        let command_args: Vec<&str> = command_to_execute.split_whitespace().collect();

        if command_args.len() <= 0 {
            continue;
        }

        let mut child = Command::new(command_args[0])
            .args(&command_args[1..])
            .spawn()
            .expect("Unable to execute command");

        child.wait().expect("Abnormal process termination");
    }
}
