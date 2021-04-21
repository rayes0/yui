use std::{
    process::Command,
    fs::File,
    io::{ prelude::*, BufReader },
    path::Path,
};

use crate::builtins;
use crate::paths;

// read files line by line, putting each line into a vector

pub fn split_lines(path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn split_to_args(line: String) -> Vec<String> {
    let mut args = Vec::new();
    let mut cur_sep = String::new(); // for tracking the current seperator
    let mut cur_arg = String::new(); // for tracking the current arg
    let mut new_cycle = false;

    // TODO: Is just looping over all the characters really the best way to do this?
    for (_, c) in line.chars().enumerate() {
        // Order matters here!

        // Single and double quotes
        if c == '"' || c == '\'' {
            if cur_sep.is_empty() {
                cur_sep.push(c);
                continue;
            } else if cur_sep == c.to_string() {
                cur_sep = String::new();
                continue;
            } /*else {
                cur_arg.push(c);
            }*/
        }

        // Spaces
        if c == ' ' {
            if cur_sep.is_empty() {
              args.push(cur_arg.trim().to_string());
              new_cycle = true;
            } else {
                cur_arg.push(c);
                continue;
            }
        }

        // Expand home
        if c == '~' {
            if cur_sep.is_empty() {
                cur_arg.push_str(&paths::get_user_home());
                continue;
            } else {
                cur_arg.push('~');
                continue;
            }
        }

        if new_cycle == true {
            cur_arg = String::new();
            new_cycle = false;
            continue;
        }

        // Regular character if it matches none of the above
        cur_arg.push(c);
    }

    args.push(cur_arg.trim().to_string());
    return args;
}

pub fn spawn_cmd(c: &Vec<String>) {
    //let mut cmd_split = c.trim().split_whitespace();
    let mut cmd_split = c.iter();
    let cmd = cmd_split.next().unwrap(); // first one will be the command
    let args = cmd_split;
    // Need to check for builtins
    match cmd.as_str() {
        "cd" => builtins::cd(args.collect()),
        "echo" => builtins::echo(args.collect()),
        "export" => builtins::export(args.collect()),
        //"history" => builtins::history(),
        "version" => println!("yui, version 0.0\nA bash-like shell focused on speed and simplicity.\n"),
        "builtins" => println!("Builtin commands:\ncd\necho\nversion\nexit"),
        // Run commands, echo any errors
        cmd => {
            let child_cur = Command::new(cmd).args(args).spawn();
            match child_cur {
                Ok(mut child_cur) => { 
                    if let Err(m) = child_cur.wait() {
                        eprintln!("{}", m);
                    }
                },
                Err(e) => eprintln!("{}", e),
            }
        }
    };
}

