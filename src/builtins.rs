use lazy_static::lazy_static;
use regex::Regex;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, ErrorKind},
    path::Path,
};

use crate::paths;
use crate::Context;
//use crate::ALIASES;

// Initialize this now to avoid recompilation every time its used
lazy_static! {
    pub static ref CHECK_EQ: Regex = Regex::new(r"^([a-zA-Z0-9_]+)=(.*)$").unwrap();
}

pub fn cd(d: &[&String]) {
    let new_dir;
    if d.is_empty() {
        new_dir = paths::get_user_home();
    } else if d.iter().count() > 1 {
        eprintln!("yui: cd: Too many arguments");
        return;
    } else {
        new_dir = d.into_iter().peekable().peek().map_or("/".to_string(), |x| (*x).to_string());
    };
    let final_path = new_dir;
    let root = Path::new(&final_path);
    if let Err(e) = env::set_current_dir(&root) {
        match e.kind() {
            ErrorKind::NotFound => eprintln!("yui: cd: No such file or directory"),
            ErrorKind::PermissionDenied => eprintln!("yui: cd: Permission denied"),
            _ => eprintln!("yui: cd: {}", e),
        }
    }
}

pub fn echo(s: &[&String]) {
    let mut to_print = String::new();
    for word in s.iter() {
        to_print.push_str(word);
        to_print.push(' ');
    }
    println!("{}", to_print.trim());
}

pub fn export(s: &[&String]) {
    for input in s.iter() {
        if !CHECK_EQ.is_match(input) {
            eprintln!("yui: export: invalid usage\n  export OPTION=VALUE");
            break;
        }

        for cap in CHECK_EQ.captures_iter(input) {
            let name = cap[1].to_string();
            let value = paths::expand_home(&cap[2]);
            env::set_var(name, &value);
        }
    }
}

pub fn set(ctx: &mut Context, s: &[&String]) {
    if s.is_empty() {
        println!("{:#?}", ctx.config)
    } else {
        for input in s.iter() {
            if !CHECK_EQ.is_match(input) {
                eprintln!("yui: set: invalid usage\n  set OPTION=VALUE");
                break;
            }

            for cap in CHECK_EQ.captures_iter(input) {
                let name = cap[1].to_string();
                let value = paths::expand_home(&cap[2]);
                if crate::config::convert_and_set_key(ctx, &name, &value) == false {
                    eprintln!("Invalid option: '{}'", name);
                }
            }
        }
    }
}

pub fn alias(ctx: &mut Context, s: &[&String]) {
    if s.is_empty() {
        let map = &mut ctx.aliases;
        if map.is_empty() {
            println!("No aliases set");
        } else {
            println!("Currently set aliases:");
            for (k, v) in map.iter() {
                println!("  {}={}", k, v);
            }
            return;
        }
    }
    let all = &mut ctx.aliases;
    for input in s.iter() {
        if !CHECK_EQ.is_match(input) {
            eprintln!("yui: alias: invalid usage\n  alias OPTION=VALUE OPTION=VALUE ...");
            break;
        }

        for cap in CHECK_EQ.captures_iter(input) {
            let name = cap[1].to_string();
            let value = cap[2].to_string();
            all.insert(name, value);
        }
    }
}

//pub fn history(num: usize, hist: &String, s: &[&String]) {
//let pad = num.to_string().len(); // not the optimal way, but it works
pub fn history(hist: &String, s: &[&String]) {
    if s.is_empty() {
        println!("History from file: {}\n", hist);
        let reader = BufReader::new(File::open(hist).unwrap());
        for (i, l) in reader.lines().enumerate() {
            println!("{: >8}   {}", i, l.unwrap());
        }
    } /* else if s.get(0) == "clear" {
      }*/
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn cd_basic_test() {
        let path = "/tmp".to_string();
        let vec = vec![&path];
        cd(&vec);
        let new = env::current_dir().expect("can't get current dir");
        assert_eq!("/tmp", new.as_os_str().to_str().unwrap());
    }
}
