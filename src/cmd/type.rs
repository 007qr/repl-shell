use std::path::Path;

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

        match std::env::var("PATH") {
            Ok(path_var) => {
                for dir in path_var.split(':') {
                    let candidate = Path::new(dir).join(command);

                    if let Ok(metadata) = candidate.metadata() {
                        if metadata.is_file() && is_executable(&metadata) {
                            return Ok(CommandResult::Streams {
                                stdout: format!("{command} is {}\n", candidate.display()),
                                stderr: String::new(),
                            });
                        }
                    }
                }

                Ok(CommandResult::Streams {
                    stdout: String::new(),
                    stderr: format!("{command}: not found\n"),
                })
            }
            Err(e) => Ok(CommandResult::Streams {
                stdout: String::new(),
                stderr: format!("error: {e}\n"),
            }),
        }
    }
}
