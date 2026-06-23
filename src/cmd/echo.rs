use crate::cmd::{CommandResult, Shell, ShellCmd, ShellError};

pub struct Echo;

impl ShellCmd for Echo {
    fn name(&self) -> &str {
        "echo"
    }

    fn run(&self, args: Vec<&str>, _shell: &mut Shell) -> Result<CommandResult, ShellError> {
        Ok(CommandResult::Streams {
            stdout: format!("{}\n", args.join(" ")),
            stderr: String::new(),
        })
    }
}
