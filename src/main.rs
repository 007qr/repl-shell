#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                let result = eval(input);

                if result == 1 {
                    break;
                }

                println!("{input}: command not found");
            }
            Err(err) => {
                println!("error: {err}");
                break;
            }
        }
    }
}

fn eval(input: &str) -> i8 {
    if input == "exit" {
        return 1;
    }

    return 0;
}
