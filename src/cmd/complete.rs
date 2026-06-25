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
                    return Ok(CommandResult::Streams { stdout: String::new(), stderr: format!("complete: {value}: no completion specification") });
                } else {
                    return Ok(CommandResult::Silent);
                }
            }
        }

        Ok(CommandResult::Silent)
    }
}
