use crate::cmd::{CommandResult, Shell, ShellCmd, ShellError};

pub struct Complete;

impl ShellCmd for Complete {
    fn name(&self) -> &str {
        "complete"
    }

    fn run(&self, args: Vec<&str>, shell: &mut Shell) -> Result<CommandResult, ShellError> {
        Ok(CommandResult::Silent) 
    }
}


