use std::{
    env,
    fs::File,
    process::exit,
    borrow::Cow::{ self, Borrowed, Owned },
};
use regex::Regex;

use std::sync::Mutex;
use lazy_static::lazy_static;

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

mod parser;
mod spawn;
mod builtins;
mod paths;
mod config;

// Initialize global config
use config::YuiConfig;
lazy_static! {
    static ref CONFIG: Mutex<YuiConfig> = Mutex::new(<YuiConfig as Default>::default());
}

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
        Owned(hint.color(CONFIG.lock().unwrap().hinting_color).to_string())
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
        match arg.as_str() {
            "-c" => {
                let mut to_run = String::new();
                for (i, arg) in env::args().enumerate() {
                    if i == 0 || arg == "-c" {
                        continue;
                    } else if to_run == "" {
                        to_run.push_str(arg.as_str());
                    } else {
                        to_run.push(' ');
                        to_run.push_str(arg.as_str());
                    };
                };
                spawn::choose_and_run(parser::split_to_args(to_run));
            },
            _ => {
                let re = Regex::new(r"-.*").unwrap();
                if re.is_match(&arg) {
                    eprintln!("Invalid arg: {}", arg);
                    return;
                } else {
                    parser::parse_file(arg);
                }
            },
        }
    }

    if let Some(f) = paths::get_user_config() {
        parser::parse_file(f);
    }

    let histpath: String = [paths::get_user_home(), ".yui_history".to_string()].join("/");
    loop {
        if repl(&histpath) == true {
            continue;
        } else {
            break;
        }
    }
}

fn repl(hist: &String) -> bool {
    let helper = CustomHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        validator: MatchingBracketValidator::new(),
        hinter: HistoryHinter {},
        styled_prompt: "".to_owned(),
    };

    let mut rl = Editor::with_config(editor_config());
    rl.set_helper(Some(helper));
    if rl.load_history(hist).is_err() {
        File::create(hist).expect("Could not create history file");
    }

    // REPL
    let ret: bool = loop {
        let prompt = get_prompt();
        rl.helper_mut().expect("No helper!").styled_prompt = prompt.italic().blue().to_string();
        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                if line.trim() == "exit" {
                    println!("Goodbye!");
                    rl.save_history(hist).unwrap();
                    exit(0);
                } else if line.trim() == "" { // line is empty or whitespace only
                    continue;
                } else if regex::Regex::new(r"^\#.*").unwrap()
                    .is_match(&line) { // line is a comment
                        continue;
                //TODO: Make this more reliable by matching later
                } else if regex::Regex::new(r"^set\s.*").unwrap() 
                    .is_match(&line.trim()) {
                        spawn::choose_and_run(parser::split_to_args(line));
                        break true // need to reload the line editor
                }

                spawn::choose_and_run(parser::split_to_args(line));
            },
            Err(ReadlineError::Interrupted) => {
                println!("^c");
            },
            // exit on ^d
            Err(ReadlineError::Eof) => {
                println!("^d... Goodbye!");
                rl.save_history(hist).unwrap();
                break false
            },
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    };
    if ret == true {
        true
    } else {
        false
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
    let conf = CONFIG.lock().unwrap();
    Config::builder()
        .max_history_size(conf.hist_max_size)
        .history_ignore_dups(conf.hist_ign_dups)
        .history_ignore_space(conf.hist_ign_space)
        .completion_type(conf.completion_type)
        .completion_prompt_limit(conf.completion_limit)
        .keyseq_timeout(conf.keyseq_timeout)
        .edit_mode(conf.edit_mode)
        .auto_add_history(conf.auto_add_history)
        .bell_style(conf.bell_style)
        .color_mode(conf.color_mode)
        .tab_stop(conf.tab_stop)
        .check_cursor_position(conf.check_cur_pos)
        .indent_size(conf.indent_size)
        .bracketed_paste(conf.bracketed_paste)
        .build()
}
