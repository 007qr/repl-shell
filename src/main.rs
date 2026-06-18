#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            println!("{input}: command not found");
        },
        Err(err) => {
            println!("error: {err}");
        }
    }
}
