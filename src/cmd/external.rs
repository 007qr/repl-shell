use std::process::Command;

use crate::cmd::{CommandResult, ShellError};

pub struct ExternalCmd;

impl ExternalCmd {
    pub fn run(command: &str, args: Vec<&str>) -> Result<CommandResult, ShellError> {
        match Command::new(command).args(args).output() {
            Ok(output) => Ok(CommandResult::Streams {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            }),
            Err(_) => Ok(CommandResult::Streams {
                stdout: String::new(),
                stderr: format!("{command}: command not found\n"),
            }),
        }
    }
}
