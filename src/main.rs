use std::env;
//use std::process::exit;
//use std::path;
use std::fs::File;
//use std::io::prelude::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::process::exit;
//use rustyline::highlight::Highlighter;
//use colored::*;

mod spawn;

fn main() {
    /*let homedir: String;
    match home::home_dir() {
        Some(path) => { homedir = path.display().to_string(); },
        None => {
            eprintln!("Can't get your home dir! Exiting...");
            exit(1);
        },
    }*/
    let homedir: String = env::var("HOME").expect("Could not get your home directory");
    let histpath: String = [homedir, ".yui_history".to_string()].join("/");
    let mut rl = Editor::<()>::new();
    loop {
        let pre = ">> ";
        let cwd: String = env::current_dir().unwrap().display().to_string();
        let prompt = " $  ";
        let prompt_full = [pre, &cwd[..], prompt].join("");
        let prompt_full_slice: &str = &prompt_full[..];
        //println("{}", Highlighter::highlight_prompt(prompt_full_slice));
        let readline = rl.readline(prompt_full_slice);
        /*if !path::Path::new(&histpath).exists() {
        }*/
        if rl.load_history(&histpath).is_err() {
            File::create(&histpath).expect("Could not create history file");
        }
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line == "exit" {
                    println!("Goodbye!");
                    rl.save_history(&histpath).unwrap();
                    exit(0);
                }
                spawn::parse_entry(line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("^c");
            },
            Err(ReadlineError::Eof) => {
                println!("^d... Goodbye!");
                rl.save_history(&histpath).unwrap();
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}
