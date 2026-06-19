use std::{
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use crate::cmd::{CommandResult, Shell, ShellCmd, ShellError, is_executable};

pub struct Echo;

impl ShellCmd for Echo {
    fn name(&self) -> &str {
        "echo"
    }

    fn run(&self, args: Vec<&str>, _shell: &mut Shell) -> Result<CommandResult, ShellError> {
        Ok(CommandResult::Output(format!("{}\n", args.join(" "))))
    }
}

pub struct Exit;

impl ShellCmd for Exit {
    fn name(&self) -> &str {
        "exit"
    }

    fn run(&self, _args: Vec<&str>, _shell: &mut Shell) -> Result<CommandResult, ShellError> {
        Ok(CommandResult::Kill)
    }
}

pub struct Type;

impl ShellCmd for Type {
    fn name(&self) -> &str {
        "type"
    }

    fn run(&self, args: Vec<&str>, shell: &mut Shell) -> Result<CommandResult, ShellError> {
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

    fn run(&self, _args: Vec<&str>, shell: &mut Shell) -> Result<CommandResult, ShellError> {
        Ok(CommandResult::Output(format!(
            "{}",
            shell.working_dir.display()
        )))
    }
}

pub struct Cd;

impl ShellCmd for Cd {
    fn name(&self) -> &str {
        "cd"
    }

    fn run(&self, args: Vec<&str>, shell: &mut Shell) -> Result<CommandResult, ShellError> {
        let target = match args.first() {
            Some(dir) => *dir,
            None => "~",
        };

        let path = if target == "~" {
            match std::env::var("HOME") {
                Ok(home) => PathBuf::from(home),
                Err(_) => {
                    return Ok(CommandResult::Output("cd: HOME not set".to_string()));
                }
            }
        } else if let Some(rest) = target.strip_prefix("~/") {
            match std::env::var("HOME") {
                Ok(home) => PathBuf::from(home).join(rest),
                Err(_) => {
                    return Ok(CommandResult::Output("cd: HOME not set".to_string()));
                }
            }
        } else {
            PathBuf::from(target)
        };

        match std::env::set_current_dir(&path) {
            Ok(_) => {
                if let Ok(current_dir) = std::env::current_dir() {
                    shell.working_dir = current_dir;
                }

                Ok(CommandResult::Silent)
            }
            Err(e) => Ok(CommandResult::Output(format!("cd: {}: {}", target, e))),
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
