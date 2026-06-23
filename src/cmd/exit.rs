use crate::cmd::{CommandResult, Shell, ShellCmd, ShellError};

pub struct Exit;

impl ShellCmd for Exit {
    fn name(&self) -> &str {
        "exit"
    }

    fn run(&self, _args: Vec<&str>, _shell: &mut Shell) -> Result<CommandResult, ShellError> {
        Ok(CommandResult::Kill)
    }
}
