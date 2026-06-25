use std::path::PathBuf;

use crate::cmd::{CommandResult, Shell, ShellCmd, ShellError};

pub struct Complete;

impl ShellCmd for Complete {
    fn name(&self) -> &str {
        "complete"
    }

    fn run(&self, args: Vec<&str>, shell: &mut Shell) -> Result<CommandResult, ShellError> {
        let mut args_iter = args.iter().peekable();
        if let Some(arg) = args_iter.peek() {
            if **arg == "-p" {
                args_iter.next();

                if let Some(value) = args_iter.peek() {
                    if shell.completer().contains_key(**value) {
                        let path = shell.completer().get(&value.to_string());
                        if let Some(path) = path {
                            return Ok(CommandResult::Streams {
                                stdout: format!("complete -C '{}' {value}", &path.display()),
                                stderr: String::new(),
                            });
                        }
                    }

                    return Ok(CommandResult::Streams {
                        stdout: String::new(),
                        stderr: format!("complete: {value}: no completion specification"),
                    });
                } else {
                    return Ok(CommandResult::Silent);
                }
            }

            if **arg == "-C" {
                args_iter.next();

                if let Some(path) = args_iter.peek() {
                    let path = PathBuf::from(*path);
                    args_iter.next();

                    if let Some(cmd) = args_iter.peek() {
                        shell.set_completer(cmd.to_string(), path);
                        return Ok(CommandResult::Silent);
                    } else {
                        return Ok(CommandResult::Silent);
                    }
                } else {
                    return Ok(CommandResult::Silent);
                }
            }
        }

        Ok(CommandResult::Silent)
    }
}
