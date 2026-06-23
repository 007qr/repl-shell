mod cd;
mod echo;
mod exit;
mod external;
mod pwd;
mod r#type;

use std::os::unix::fs::PermissionsExt;
use std::path::{PathBuf};
use std::rc::Rc;
use std::{collections::HashMap, fs::Metadata};
use std::{env, fs };

use self::{cd::Cd, echo::Echo, exit::Exit, external::ExternalCmd, pwd::Pwd, r#type::Type};

pub struct Shell {
    builtin_commands: HashMap<String, Rc<dyn ShellCmd>>,
    external_commands: HashMap<String, PathBuf>,
    working_dir: PathBuf,
}

impl Shell {
    pub fn new() -> Self {
        let builtin_commands: HashMap<String, Rc<dyn ShellCmd>> = vec![
            Rc::new(Echo) as Rc<dyn ShellCmd>,
            Rc::new(Exit),
            Rc::new(Type),
            Rc::new(Pwd),
            Rc::new(Cd),
        ]
        .into_iter()
        .map(|cmd| (cmd.name().to_string(), cmd))
        .collect();

        let current_dir = env::current_dir().unwrap();
        Self {
            builtin_commands,
            working_dir: current_dir,
            external_commands: HashMap::new(),
        }
    }

    pub fn execute(&mut self, command: &str, args: Vec<&str>) -> Result<CommandResult, ShellError> {
        if let Some(cmd) = self.builtin_commands.get(command).cloned() {
            cmd.run(args, self)
        } else {
            ExternalCmd::run(command, args)
        }
    }

    pub fn builtin_commands(&self) -> &HashMap<String, Rc<dyn ShellCmd>> {
        &self.builtin_commands
    }

    pub fn working_dir(&self) -> &PathBuf {
        &self.working_dir
    }

    pub fn external_commands(&mut self) -> &HashMap<String, PathBuf> {
        if self.external_commands.is_empty() {
            if let Some(path_env) = env::var_os("PATH") {
                for dir_path in env::split_paths(&path_env) {

                    if let Ok(entries) = fs::read_dir(&dir_path) {
                        for entry in entries.flatten() {
                            let path = entry.path();

                            if path.is_file() {
                                if let Some(file_name)  = path.file_name().and_then(|n| n.to_str()) {
                                    self.external_commands.insert(file_name.to_string(), path);
                                }
                            }
                        }
                    }
                }
            }
        }
        &self.external_commands
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
    /// stdout and stderr produced by a command; each stream is routed
    /// independently by the redirection logic.
    Streams {
        stdout: String,
        stderr: String,
    },
    Silent,
    Kill,
}

pub trait ShellCmd {
    fn name(&self) -> &str;

    fn run(&self, args: Vec<&str>, shell: &mut Shell) -> Result<CommandResult, ShellError>;
}

#[inline]
pub fn is_executable(metadata: &Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}
