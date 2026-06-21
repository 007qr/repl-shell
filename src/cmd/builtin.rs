use std::{
    path::Path,
    process::Command,
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

                Ok(CommandResult::ErrOutput(format!("{command}: not found\n")))
            }
            Err(e) => Ok(CommandResult::ErrOutput(format!("error: {e}\n"))),
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
        // Tilde expansion already happened in the tokenizer for unquoted ~,
        // so cd only needs HOME for the no-argument case.
        let target = match args.first() {
            Some(dir) => dir.to_string(),
            None => match std::env::var("HOME") {
                Ok(home) => home,
                Err(_) => {
                    return Ok(CommandResult::ErrOutput("cd: HOME not set".to_string()));
                }
            },
        };

        match std::env::set_current_dir(&target) {
            Ok(_) => {
                shell.working_dir = std::env::current_dir().unwrap();
                Ok(CommandResult::Silent)
            }
            Err(e) => {
                let msg = match e.kind() {
                    std::io::ErrorKind::NotFound => "No such file or directory",
                    std::io::ErrorKind::PermissionDenied => "Permission denied",
                    _ => "Unknown error",
                };

                Ok(CommandResult::ErrOutput(format!("cd: {}: {}", target, msg)))
            }
        }
    }
}

pub struct ExternalCmd;

impl ExternalCmd {
    pub fn run(command: &str, args: Vec<&str>) -> Result<CommandResult, ShellError> {
        match Command::new(command).args(args).output() {
            Ok(output) => Ok(CommandResult::Streams {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            }),
            Err(_) => Ok(CommandResult::ErrOutput(format!(
                "{command}: command not found\n"
            ))),
        }
    }
}
