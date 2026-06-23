use std::{env, path::PathBuf};

use crate::cmd::{CommandResult, Shell, ShellCmd, ShellError, is_executable};

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
            return Ok(CommandResult::Streams {
                stdout: format!("{command} is a shell builtin\n"),
                stderr: String::new(),
            });
        }

        if let Some(path_env) = env::var_os("PATH") {
            for dir in env::split_paths(&path_env) {
                let candidate = PathBuf::from(dir).join(command);

                if candidate.is_file() && is_executable(&candidate) {
                    return Ok(CommandResult::Streams {
                        stdout: format!("{command} is {}\n", candidate.display()),
                        stderr: String::new(),
                    });
                }
            }

            Ok(CommandResult::Streams {
                stdout: String::new(),
                stderr: format!("{command}: not found\n"),
            })
        } else {
            return Ok(CommandResult::Streams {
                stdout: String::new(),
                stderr: format!("$PATH not set"),
            });
        }
    }
}
