use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};

pub struct ShellCompleter {
    command_completer: Vec<String>,
    file_completer: FilenameCompleter,
}
impl ShellCompleter {
    pub fn new(words: Vec<String>) -> Self {
        Self {
            command_completer: words,
            file_completer: FilenameCompleter::default(),
        }
    }
}

impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let line = &line[..pos];

        if line.contains(' ') {
            let (start, candidates) = self.file_completer.complete(line, pos, ctx)?;
            let candidates = candidates
                .into_iter()
                .map(|pair| {
                    if pair.replacement.ends_with('/') {
                        pair
                    } else {
                        Pair {
                            replacement: format!("{} ", pair.replacement),
                            ..pair
                        }
                    }
                })
                .collect::<Vec<Pair>>();

            if candidates.len() > 1 {
                print!("\x07");
                use std::io::{self, Write};
                let _ = io::stdout().flush();
            }

            Ok((start, candidates))
        } else {
            let mut candidates = Vec::new();
            let mut words = self.command_completer.clone();
            words.sort();

            for word in &words {
                if word.starts_with(line) {
                    candidates.push(Pair {
                        display: word.clone(),
                        replacement: format!("{} ", word),
                    });
                }
            }

            Ok((0, candidates))
        }
    }
}

impl Hinter for ShellCompleter {
    type Hint = String;
}

impl Highlighter for ShellCompleter {}

impl Validator for ShellCompleter {}

impl Helper for ShellCompleter {}
