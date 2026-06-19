use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

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

struct Shell {
    builtin_commands: HashMap<String, Box<dyn ShellCmd>>,
}

impl Shell {
    fn new() -> Self {
        let builtin_commands: HashMap<String, Box<dyn ShellCmd>> = vec![
            Box::new(Echo) as Box<dyn ShellCmd>,
            Box::new(Exit),
            Box::new(Type),
        ]
        .into_iter()
        .map(|cmd| (cmd.name().to_string(), cmd))
        .collect();

        Self { builtin_commands }
    }

    fn execute(&self, command: &str, args: Vec<&str>) -> Result<CommandResult, ShellError> {
        if let Some(cmd) = self.builtin_commands.get(command) {
            cmd.run(args, self)
        } else {
            ExternalCmd::run(command, args)
        }
    }
}

#[derive(Debug)]
enum ShellError {
    Exit(i8),
}

impl std::fmt::Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit(code) => write!(f, "{code}"),
        }
    }
}

enum CommandResult {
    Output(String),
    Silent,
    Kill,
}

trait ShellCmd {
    fn name(&self) -> &str;

    fn run(&self, args: Vec<&str>, shell: &Shell) -> Result<CommandResult, ShellError>;
}

pub struct Echo;

impl ShellCmd for Echo {
    fn name(&self) -> &str {
        "echo"
    }

    fn run(&self, args: Vec<&str>, _shell: &Shell) -> Result<CommandResult, ShellError> {
        Ok(CommandResult::Output(format!("{}\n", args.join(" "))))
    }
}

pub struct Exit;

impl ShellCmd for Exit {
    fn name(&self) -> &str {
        "exit"
    }

    fn run(&self, _args: Vec<&str>, _shell: &Shell) -> Result<CommandResult, ShellError> {
        Ok(CommandResult::Kill)
    }
}

pub struct Type;

impl ShellCmd for Type {
    fn name(&self) -> &str {
        "type"
    }

    fn run(&self, args: Vec<&str>, shell: &Shell) -> Result<CommandResult, ShellError> {
        if args.is_empty() {
            return Ok(CommandResult::Silent);
        }

        let command = args[0];

        if shell.builtin_commands.contains_key(command) {
            return Ok(CommandResult::Output(format!(
                "{command} is a shell builtin\n"
            )));
        }

        match std::env::var("PATH") {
            Ok(path_var) => {
                for dir in path_var.split(':') {
                    let candidate = Path::new(dir).join(command);

                    if let Ok(metadata) = candidate.metadata() {
                        if metadata.is_file() && is_executable(&metadata) {
                            return Ok(CommandResult::Output(format!(
                                "{command} is {}\n",
                                candidate.display()
                            )));
                        }
                    }
                }

                Ok(CommandResult::Output(format!("{command}: not found\n")))
            }
            Err(e) => Ok(CommandResult::Output(format!("error: {e}\n"))),
        }
    }
}

struct ExternalCmd;

impl ExternalCmd {
    fn run(command: &str, args: Vec<&str>) -> Result<CommandResult, ShellError> {
        match Command::new(command).args(args).output() {
            Ok(output) => {
                if !output.stderr.is_empty() {
                    return Ok(CommandResult::Output(
                        String::from_utf8_lossy(&output.stderr).to_string(),
                    ));
                }

                Ok(CommandResult::Output(
                    String::from_utf8_lossy(&output.stdout).to_string(),
                ))
            }
            Err(_) => Ok(CommandResult::Output(format!(
                "{command}: command not found\n"
            ))),
        }
    }
}

fn is_executable(metadata: &fs::Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}
