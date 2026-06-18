#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input: Vec<&str> = input.trim().split(" ").collect();
                let command = input[0];
                let args = input[1..].join(" ");
                match eval(command, &args) {
                    Ok(res) => {
                        println!("{res}");
                    },
                    Err(ShellError::Exit(code)) => {
                        if code == 1 {
                            break;
                        } else {
                            println!("{command}: command not found");
                        }
                    }
                }

            }
            Err(err) => {
                println!("error: {err}");
                break;
            }
        }
    }
}

enum ShellError {
    Exit(i8)
}

fn eval(command: &str, args: &str) -> Result<String, ShellError> {
    if command == "exit" {
        return Err(ShellError::Exit(1));
    }

    if command == "echo" {
        return Ok(args.to_string());
    }

    Err(ShellError::Exit(0))
}
