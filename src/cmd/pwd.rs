use crate::cmd::{CommandResult, Shell, ShellCmd, ShellError};

pub struct Pwd;

impl ShellCmd for Pwd {
    fn name(&self) -> &str {
        "pwd"
    }

    fn run(&self, _args: Vec<&str>, shell: &mut Shell) -> Result<CommandResult, ShellError> {
        Ok(CommandResult::Streams {
            stdout: format!("{}", shell.working_dir.display()),
            stderr: String::new(),
        })
    }
}
