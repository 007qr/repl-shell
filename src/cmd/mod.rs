pub mod builtin;

use std::env;
use std::path::PathBuf;
use std::rc::Rc;
use std::{collections::HashMap, fs::Metadata};
use std::os::unix::fs::PermissionsExt;

use crate::cmd::builtin::{Cd, Echo, Exit, ExternalCmd, Pwd, Type};

pub struct Shell {
    builtin_commands: HashMap<String, Rc<dyn ShellCmd>>,
    working_dir: PathBuf
}

impl Shell {
    pub fn new() -> Self {
        let builtin_commands: HashMap<String, Rc<dyn ShellCmd>> = vec![
            Rc::new(Echo) as Rc<dyn ShellCmd>,
            Rc::new(Exit),
            Rc::new(Type),
            Rc::new(Pwd),
            Rc::new(Cd)
        ]
        .into_iter()
        .map(|cmd| (cmd.name().to_string(), cmd))
        .collect();

        let current_dir = env::current_dir().unwrap();
        Self { builtin_commands, working_dir: current_dir }
    }

    pub fn execute(&mut self, command: &str, args: Vec<&str>) -> Result<CommandResult, ShellError> {
        if let Some(cmd) = self.builtin_commands.get(command).cloned() {
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
    ErrOutput(String),
    Silent,
    Kill,
}

pub trait ShellCmd {
    fn name(&self) -> &str;

    fn run(&self, args: Vec<&str>, shell: &mut Shell) -> Result<CommandResult, ShellError>;
}

#[inline]
fn is_executable(metadata: &Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}
