use crate::cmd::{CommandResult, Shell, ShellCmd, ShellError};

pub struct Cd;

impl ShellCmd for Cd {
    fn name(&self) -> &str {
        "cd"
    }

    fn run(&self, args: Vec<&str>, shell: &mut Shell) -> Result<CommandResult, ShellError> {
        let target = match args.first() {
            Some(dir) => dir.to_string(),
            None => match std::env::var("HOME") {
                Ok(home) => home,
                Err(_) => {
                    return Ok(CommandResult::Streams {
                        stdout: String::new(),
                        stderr: "cd: HOME not set".to_string(),
                    });
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

                Ok(CommandResult::Streams {
                    stdout: String::new(),
                    stderr: format!("cd: {}: {}", target, msg),
                })
            }
        }
    }
}
