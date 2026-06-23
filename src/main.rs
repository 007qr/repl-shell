pub mod cmd;
pub mod shell_completer;

use std::fs::OpenOptions;
use std::io::Write;

use rustyline::Editor;

use crate::cmd::{CommandResult, Shell};
use crate::shell_completer::ShellCompleter;

struct TokenizedInput {
    command: String,
    args: Vec<String>,
}

struct Redirection {
    error: bool,
    file_name: String,
    append_only: bool,
}

fn main() {
    let mut shell = Shell::new();
    let config = rustyline::Config::builder()
        .history_ignore_space(true)
        .completion_type(rustyline::CompletionType::List)
        .bell_style(rustyline::config::BellStyle::Audible)
        .build();

    let mut cmd_names: Vec<String> = shell
        .builtin_commands()
        .keys()
        .into_iter()
        .map(String::from)
        .collect();

    let mut external_cmd_names: Vec<String> =
        shell.external_commands().keys().map(String::from).collect();

    cmd_names.append(&mut external_cmd_names);


    let h = ShellCompleter::new(cmd_names);

    let mut rl = Editor::with_config(config).expect("failed to create rustyline editor");
    rl.set_helper(Some(h));

    loop {
        match rl.readline("$ ") {
            Ok(input) => {
                let _ = rl.add_history_entry(&input);

                let input = input.trim();

                let (redirection, parsed_input) = parse_input(&input);
                let Some(tokenized) = tokenize_input(parsed_input) else {
                    continue; // empty line — just re-prompt
                };

                let args: Vec<&str> = tokenized.args.iter().map(|s| s.as_str()).collect();

                let redirect_stdout = !redirection.file_name.is_empty() && !redirection.error;
                let redirect_stderr = !redirection.file_name.is_empty() && redirection.error;

                match shell.execute(&tokenized.command, args) {
                    Ok(CommandResult::Kill) => break,
                    Ok(CommandResult::Streams { stdout, stderr }) => {
                        emit(
                            &stdout,
                            redirect_stdout,
                            &redirection.file_name,
                            redirection.append_only,
                        );
                        emit(
                            &stderr,
                            redirect_stderr,
                            &redirection.file_name,
                            redirection.append_only,
                        );
                    }
                    Ok(CommandResult::Silent) => {}
                    Err(e) => println!("error: {e}"),
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => break,
            Err(rustyline::error::ReadlineError::Eof) => break,
            Err(err) => {
                println!("error: {err}");
                break;
            }
        }
    }
}

/// Send a single output stream to its destination: the redirection file when
/// `to_file` is set, otherwise the terminal (skipping empty terminal output).
fn emit(stream: &str, to_file: bool, file_name: &str, append_only: bool) {
    if to_file {
        if !append_only {
            if let Err(e) = std::fs::write(file_name, stream) {
                println!("error: {e}");
            }
        } else {
            match OpenOptions::new().append(true).create(true).open(file_name) {
                Ok(mut file) => {
                    if let Err(e) = write!(file, "{}", stream) {
                        println!("error: {e}");
                    }
                }
                Err(e) => {
                    println!("error: {e}");
                }
            }
        }
    } else if !stream.is_empty() {
        print!("{stream}");
        if !stream.ends_with('\n') {
            println!();
        }
    }
}

fn parse_input(input: &str) -> (Redirection, Vec<String>) {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut file_name = String::new();
    let mut in_token = false;
    let mut stderr_output = false;
    let mut append_only = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                in_token = true;
                // Inside single quotes everything is literal until the next '
                while let Some(c) = chars.next() {
                    if c == '\'' {
                        break;
                    }
                    current.push(c);
                }
            }
            '"' => {
                in_token = true;
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    }
                    if c == '\\' {
                        if let Some(&next) = chars.peek() {
                            if matches!(next, '"' | '\\' | '$' | '`') {
                                current.push(chars.next().unwrap());
                                continue;
                            }
                        }
                    }
                    current.push(c);
                }
            }
            '\\' => {
                in_token = true;
                if let Some(&_next) = chars.peek() {
                    current.push(chars.next().unwrap());
                    continue;
                }
                current.push(c);
            }
            '>' => {
                // Flush a real pending token (e.g. `echo hi>file`), but not the
                // empty `current` left behind by a `1>`/`2>` fd prefix.
                if in_token && !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
                in_token = false;

                // Remove space between redirection and filename

                if let Some(&next) = chars.peek() {
                    if matches!(next, '>') {
                        chars.next();
                        append_only = true;
                    }
                }

                if let Some(&next) = chars.peek() {
                    if matches!(next, ' ') {
                        chars.next();
                    }
                }

                while let Some(c) = chars.next() {
                    if c == ' ' {
                        break;
                    }

                    file_name.push(c);
                }
            }
            c if c.is_whitespace() => {
                if in_token {
                    tokens.push(std::mem::take(&mut current));
                    in_token = false;
                }
            }
            c => {
                if c == '~' && !in_token && matches!(chars.peek(), None | Some('/')) {
                    if let Ok(home) = std::env::var("HOME") {
                        current.push_str(&home);
                        in_token = true;
                        continue;
                    }
                }

                if c == '1' && !in_token && matches!(chars.peek(), Some('>')) {
                    in_token = true;
                    continue;
                }

                if c == '2' && !in_token && matches!(chars.peek(), Some('>')) {
                    in_token = true;
                    stderr_output = true;
                    continue;
                }

                in_token = true;
                current.push(c);
            }
        }
    }

    if in_token {
        tokens.push(current);
    }

    (
        Redirection {
            error: stderr_output,
            file_name,
            append_only,
        },
        tokens,
    )
}

fn tokenize_input(input: Vec<String>) -> Option<TokenizedInput> {
    let (command, args) = input.split_first()?;

    Some(TokenizedInput {
        command: command.clone(),
        args: args.to_vec(),
    })
}
