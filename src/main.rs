use std::{
	//collections::HashMap,
	env,
	fs::File,
	process::exit,
};

//use libc;
use regex::Regex;
use lazy_static::lazy_static;

use rustyline::error::ReadlineError;
use rustyline::{
    Config,
    Editor,
    completion::FilenameCompleter,
    highlight::MatchingBracketHighlighter,
    hint::HistoryHinter,
    validate::MatchingBracketValidator,

};

use colored::*;

mod builtins;
mod config;
mod context;
mod helper;
mod parser;
mod paths;
mod spawn;

use helper::CustomHelper;
use context::Context;

lazy_static! {
    static ref CHANGE_SET: Regex = Regex::new(r"^set\s.*").unwrap();
}

fn main() {
	if let Some(arg) = env::args().nth(1) {
		match arg.as_str() {
			"-h" | "--help" => {
				print_help();
				return;
			}
            "-v" | "--version" => {
                println!("yui 0.1");
                return;
            }
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
				}
                let mut context = context::Context::new();
				spawn::choose_and_run(&mut context, false, parser::split_to_args(to_run));
				return;
			}
			_ => {
				let re = Regex::new(r"-.*").unwrap();
				if re.is_match(&arg) {
					eprintln!("Invalid arg: {}", arg);
					return;
				} else {
					parser::parse_file(&mut context::Context::new(), arg);
				}
			}
		}
	}

    // Initialize config
    let mut context = context::Context::new();
	if let Some(f) = paths::get_user_config() {
		parser::parse_file(&mut context, f);
	}

	loop {
		if repl(&mut context) == true {
			continue;
		} else {
			break;
		}
	}
}

fn repl(ctx: &mut Context) -> bool {
	let helper = CustomHelper {
		completer: FilenameCompleter::new(),
		highlighter: MatchingBracketHighlighter::new(),
		validator: MatchingBracketValidator::new(),
		hinter: HistoryHinter {},
		styled_prompt: "".to_owned(),
	};
	let mut rl = Editor::with_config(editor_config(ctx.clone()));
	rl.set_helper(Some(helper));
	if rl.load_history(&ctx.histfile).is_err() {
		File::create(&ctx.histfile).expect("Could not create history file");
	}

	// REPL
	let ret: bool = loop {
		let prompt = get_prompt();
		rl.helper_mut().expect("No helper!").styled_prompt = prompt.italic().blue().to_string();
		let readline = rl.readline(&prompt);

		match readline {
			Ok(line) => {
				//let mut conf = CONFIG.lock().unwrap();
				//if conf.auto_add_history == false {
				//    rl.add_history_entry(line.as_str());
				//}

				if line.trim() == "exit" {
					println!("Goodbye!");
					rl.save_history(&ctx.histfile).unwrap();
					exit(0);
				} else if line.trim() == "" {
					// line is empty or whitespace only
					continue;
				} else if line.trim().starts_with('#') {
					// line is a comment
					continue;
				//TODO: Make this more reliable by matching later
				} else if CHANGE_SET.is_match(&line.trim()) {
					spawn::choose_and_run(ctx, true, parser::split_to_args(line));
					break true; // need to reload the line editor
				} else {
				    spawn::choose_and_run(ctx, true, parser::split_to_args(line));
                }
			}
			Err(ReadlineError::Interrupted) => {
				println!("^c");
			}
			// exit on ^d
			Err(ReadlineError::Eof) => {
				println!("^d... Goodbye!");
				rl.save_history(&ctx.histfile).unwrap();
				break false;
			}
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

fn get_prompt() -> String {
	let pre = ">> ";
	let cwd = paths::condense_home(&env::current_dir().unwrap().display().to_string());
	let post = " $  ";
	return [pre, &cwd[..], post].join("");
}

fn editor_config(ctx: Context) -> Config {
	//let conf = CONFIG.lock().unwrap();
    let conf = ctx.config;
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

fn print_help() {
	println!("yui: A simple and minimal unix shell\n");
	println!("  USAGE:  yui [OPTIONS] [FILE]\n");
	println!("  Available options:");
	println!("    -h, --help     Show this help message");
	println!("    -c [COMMAND]   Execute the specified command");
}
