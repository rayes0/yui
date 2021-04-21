use std::{
    env,
    fs::File,
    process::exit,
    borrow::Cow::{ self, Borrowed, Owned },
};

use rustyline::{ Editor, Config, Context };
use rustyline::error::ReadlineError;

use rustyline_derive::Helper;
use rustyline::{
    completion::{ Completer, FilenameCompleter, Pair },
    validate::{ self, MatchingBracketValidator, Validator, ValidationContext },
    highlight::{ Highlighter, MatchingBracketHighlighter },
    hint::{ Hinter, HistoryHinter },
};
use colored::*;

mod spawn;
mod builtins;
mod paths;

#[derive(Helper)]
struct CustomHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    styled_prompt: String,
}

impl Completer for CustomHelper {
    type Candidate = Pair;
    fn complete(&self, line: &str, pos: usize, ctx: &Context<'_>)
        -> Result<(usize, Vec<Pair>), ReadlineError> {
            self.completer.complete(line, pos, ctx)
    }
}

impl Highlighter for CustomHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.styled_prompt)
        } else {
            Borrowed(prompt)
        }
    }
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(hint.green().to_string())
    }
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }
    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Hinter for CustomHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>)
        -> Option<String> {
            self.hinter.hint(line, pos, ctx)
    }
}

impl Validator for CustomHelper {
    fn validate(&self, ctx: &mut ValidationContext<'_>)
        -> rustyline::Result::<validate::ValidationResult> {
            self.validator.validate(ctx)
    }
}

fn main() {
    if let Some(arg) = env::args().nth(1) {
        // Command line arg given, assume its a script file
        let lines = spawn::split_lines(arg);

        for s in lines.iter() {
            if regex::Regex::new(r"^\#.*").unwrap()
                .is_match(&s) { // line is a comment
                    continue;
            }

            let args = spawn::split_to_args(s.to_string());
            spawn::spawn_cmd(&args);
        }
    }
    let homedir: String = env::var("HOME").expect("Could not get your home directory");
    let histpath: String = [homedir, ".yui_history".to_string()].join("/");
    let helper = CustomHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        validator: MatchingBracketValidator::new(),
        hinter: HistoryHinter {},
        styled_prompt: "".to_owned(),
    };

    let mut rl = Editor::with_config(editor_config());
    rl.set_helper(Some(helper));
    if rl.load_history(&histpath).is_err() {
        File::create(&histpath).expect("Could not create history file");
    }

    // REPL
    loop {
        let prompt = get_prompt();
        rl.helper_mut().expect("No helper!").styled_prompt = prompt.italic().blue().to_string();
        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                if line == "exit" {
                    println!("Goodbye!");
                    rl.save_history(&histpath).unwrap();
                    exit(0);
                } else if line.trim() == "" { // line is empty or whitespace only
                    continue;
                } else if regex::Regex::new(r"^\#.*").unwrap()
                    .is_match(&line) { // line is a comment
                        continue;
                }

                let args = spawn::split_to_args(line);
                spawn::spawn_cmd(&args);
            },
            Err(ReadlineError::Interrupted) => {
                println!("^c");
                continue;
            },
            // exit on ^d
            Err(ReadlineError::Eof) => {
                println!("^d... Goodbye!");
                rl.save_history(&histpath).unwrap();
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                continue;
            }
        }

    }
}

// Have to customize these functions once we get a config file

fn get_prompt() -> String {
    let pre = ">> ";
    let cwd = paths::condense_home(&env::current_dir().unwrap().display().to_string());
    let post = " $  ";
    return [pre, &cwd[..], post].join("");
}

fn editor_config() -> Config {
    Config::builder()
        .history_ignore_space(true)
        //.completion_type(config::CompletionType::List)
        //.edit_mode(config::EditMode::Vi)
        .build()
}
