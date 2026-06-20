pub mod cmd;

use std::io::{self, Write};

use crate::cmd::{CommandResult, Shell};


fn main() {
    let mut shell = Shell::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                let result = parse_quotes(&input);
                let Some((command, args)) = tokenize_input(result) else {
                    continue; // empty line — just re-prompt
                };
                let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

                match shell.execute(&command, args) {
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

fn parse_quotes(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_token = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                in_token = true;
                // Inside single quotes everything is literal until the next '
                for c in chars.by_ref() {
                    if c == '\'' {
                        break;
                    }
                    current.push(c);
                }
            },
            '"' => {
                in_token = true;
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    }
                    if c == '\\' {
                        if let Some(&next) = chars.peek() {
                            if matches!(next, '\\' | '"' | '$' | '`' | '~') {
                                current.push(chars.next().unwrap());
                                continue;
                            }
                        }
                    }
                    current.push(c);
                }
            }
            c if c.is_whitespace() => {
                if in_token {
                    tokens.push(std::mem::take(&mut current));
                    in_token = false;
                }
            }
            c => {
                in_token = true;
                current.push(c);
            }
        }
    }

    if in_token {
        tokens.push(current);
    }

    tokens
}

fn tokenize_input(input: Vec<String>) -> Option<(String, Vec<String>)> {
    let (command, args) = input.split_first()?;
    Some((command.clone(), args.to_vec()))
}
