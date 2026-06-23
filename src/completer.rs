use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};

pub struct ShellCompleter {
    words: Vec<String>,
}

impl ShellCompleter {
    pub fn new(words: Vec<String>) -> Self {
        Self { words }
    }
}

impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>)> {
        let line = &line[..pos];

        if line.contains(' ') {
            return Ok((pos, Vec::new()));
        }

        let mut candidates = Vec::new();
        for word in &self.words {
            if word.starts_with(line) {
                candidates.push(Pair {
                    display: word.clone(),
                    replacement: word.clone(),
                });
            }
        }
        Ok((0, candidates))
    }
}

impl Hinter for ShellCompleter {
    type Hint = String;
}

impl Highlighter for ShellCompleter {}

impl Validator for ShellCompleter {}

impl Helper for ShellCompleter {}
