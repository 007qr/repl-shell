#[allow(unused_imports)]
use std::io::{self, Write};
use std::ops::Deref;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        let echo = Box::new(Echo);
        let exit = Box::new(Exit);
        let type_cmd = Box::new(Type);

        let builtin_commands: Vec<Box<dyn BuiltinCommand>> = vec![echo, exit, type_cmd];

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input: Vec<&str> = input.trim().split(" ").collect();
                let command = input[0];
                let args = input[1..].to_vec();
                let cmd = builtin_commands.iter().find(|v| v.name() == command);

                match cmd {
                    Some(cmd) => {
                        let builtin_names: Vec<&str> = builtin_commands.iter().map(|c| c.name()).collect();
                        let result = cmd.deref().run(
                            args,
                            &Shell {
                                builtin_names:  builtin_names
                            },
                        );
                        match result {
                            Ok(CommandResult::Kill) => {
                                break;
                            }
                            Ok(CommandResult::Output(s)) => {
                                println!("{s}");
                            }
                            Ok(CommandResult::Silent) => {
                                continue;
                            }
                            Err(e) => {
                                println!("error: {e}");
                            }
                        }
                    }
                    None => {
                        println!("{command}: command not found");
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

#[derive(Debug)]
enum ShellError {
    Exit(i8),
}

impl std::fmt::Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Exit(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

struct Shell<'a> {
    builtin_names: Vec<&'a str>,
}

enum CommandResult {
    Output(String),
    Silent,
    Kill,
}

trait BuiltinCommand {
    fn name(&self) -> &str;
    fn run(&self, args: Vec<&str>, shell: &Shell) -> Result<CommandResult, ShellError>;
}

pub struct Echo;

impl BuiltinCommand for Echo {
    fn name(&self) -> &str {
        "echo"
    }

    fn run(&self, args: Vec<&str>, _shell: &Shell) -> Result<CommandResult, ShellError> {
        Ok(CommandResult::Output(args.join(" ")))
    }
}

pub struct Exit;

impl BuiltinCommand for Exit {
    fn name(&self) -> &str {
        "exit"
    }

    fn run(&self, _args: Vec<&str>, _shell: &Shell) -> Result<CommandResult, ShellError> {
        Ok(CommandResult::Kill)
    }
}

pub struct Type;

impl BuiltinCommand for Type {
    fn name(&self) -> &str {
        "type"
    }

    fn run(&self, args: Vec<&str>, shell: &Shell) -> Result<CommandResult, ShellError> {
        if shell.builtin_names.contains(&args[0]) {
            Ok(CommandResult::Output(format!(
                "{} is a shell builtin",
                args[0]
            )))
        } else {
            Ok(CommandResult::Output(format!(
                "{}: command not found",
                args[0]
            )))
        }
    }
}
