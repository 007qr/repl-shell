use std::{env, path::Path, process::Command};

use crate::cmd::{CommandResult, Shell, ShellCmd, ShellError, is_executable};

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

pub struct Pwd;

impl ShellCmd for Pwd {
    fn name(&self) -> &str {
        "pwd"
    }

    fn run(&self, _args: Vec<&str>, _shell: &Shell) -> Result<CommandResult, ShellError> {
        match env::current_dir() {
            Ok(path) => Ok(CommandResult::Output(format!("{}", path.display()))),
            Err(e) => Ok(CommandResult::Output(format!("{}", e)))
        }
    }
}

pub struct ExternalCmd;

impl ExternalCmd {
    pub fn run(command: &str, args: Vec<&str>) -> Result<CommandResult, ShellError> {
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
