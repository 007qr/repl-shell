pub mod cmd;

use std::io::{self, Write};

use crate::cmd::{CommandResult, Shell};


fn main() {
    let shell = Shell::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input: Vec<&str> = input.trim().split_whitespace().collect();

                if input.is_empty() {
                    continue;
                }

                let command = input[0];
                let args = input[1..].to_vec();

                match shell.execute(command, args) {
                    Ok(CommandResult::Kill) => break,
                    Ok(CommandResult::Output(s)) => {
                        if !s.is_empty() {
                            print!("{s}");
                            if !s.ends_with('\n') {
                                println!();
                            }
                        }
                    }
                    Ok(CommandResult::Silent) => {}
                    Err(e) => println!("error: {e}"),
                }
            }
            Err(err) => {
                println!("error: {err}");
                break;
            }
        }
    }
}
