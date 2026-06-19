pub mod builtin;

use std::{collections::HashMap, fs::Metadata};
use std::os::unix::fs::PermissionsExt;

use crate::cmd::builtin::{Echo, Exit, ExternalCmd, Pwd, Type};

pub struct Shell {
    builtin_commands: HashMap<String, Box<dyn ShellCmd>>,
}

impl Shell {
    pub fn new() -> Self {
        let builtin_commands: HashMap<String, Box<dyn ShellCmd>> = vec![
            Box::new(Echo) as Box<dyn ShellCmd>,
            Box::new(Exit),
            Box::new(Type),
            Box::new(Pwd)
        ]
        .into_iter()
        .map(|cmd| (cmd.name().to_string(), cmd))
        .collect();

        Self { builtin_commands }
    }

    pub fn execute(&self, command: &str, args: Vec<&str>) -> Result<CommandResult, ShellError> {
        if let Some(cmd) = self.builtin_commands.get(command) {
            cmd.run(args, self)
        } else {
            ExternalCmd::run(command, args)
        }
    }
}

#[derive(Debug)]
pub enum ShellError {
    Exit(i8),
}

impl std::fmt::Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit(code) => write!(f, "{code}"),
        }
    }
}

pub enum CommandResult {
    Output(String),
    Silent,
    Kill,
}

pub trait ShellCmd {
    fn name(&self) -> &str;

    fn run(&self, args: Vec<&str>, shell: &Shell) -> Result<CommandResult, ShellError>;
}

#[inline]
fn is_executable(metadata: &Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}
