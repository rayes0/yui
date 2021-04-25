use std::{
    process::exit,
    fs::File,
    io::{ prelude::*, BufReader, ErrorKind },
    path::Path,
};

use rustyline::history::History;

use crate::paths;

// read files line by line, putting each line into a vector

pub fn split_lines(path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(path);
    match file {
        Ok(file) => {
            let buf = BufReader::new(file);
            buf.lines()
                .map(|l| l.expect("Could not parse line"))
                .collect()
        },
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => eprintln!("yui: File not found"),
                ErrorKind::PermissionDenied => eprintln!("yui: Permission denied"),
                _ => eprintln!("yui: Error reading file: {}", err),
            }
            exit(1);
        },
    }
}

pub enum ArgTypes {
    // first field is for the command, second indicates prescence of operator
    Piped(Vec<String>, bool),
    //Redir
    Norm(Vec<String>, bool),
}

pub fn split_to_args(line: String) -> ArgTypes {
    let mut args = Vec::new();
    let mut cur_quot = String::new(); // for tracking the current quoted string
    let mut cur_arg = String::new(); // for tracking the current arg
    let mut has_pipe = false;
    let mut has_op = false;
    let mut prev_space = false;
    let mut prev_spchar: char = '_'; // track the previous special character for two character combos, the '_' is just a placeholder
    let mut new_cycle = false;

    // TODO: Is just looping over all the characters really the best way to do this?
    for c in line.trim_end().chars() {
        // Order matters here!

        // Single and double quotes
        if c == '"' || c == '\'' {
            if cur_quot.is_empty() {
                cur_quot.push(c);
                continue;
            } else if cur_quot == c.to_string() {
                cur_quot = String::new();
                continue;
            }
        }

        // Pipes
        if c == '|' {
            if cur_quot.is_empty() {
                has_pipe = true;
                args.push("|".to_string());
                continue;
            } else {
                cur_arg.push(c);
                continue;
            }
        }

        // Spaces
        if c == ' ' {
            if cur_quot.is_empty() {
                if prev_space == false {
                    args.push(cur_arg.trim().to_string());
                    new_cycle = true;
                    prev_space = true;
                } else {
                    continue;
                }
            } else {
                cur_arg.push(c);
                continue;
            }
        } else {
            prev_space = false;
        }

        // && Operator
        if c == '&' {
            if cur_quot.is_empty() {
                if prev_spchar == '&' {
                    cur_arg.push_str("&&");
                    prev_spchar = ' ';
                    has_op = true;
                    continue;
                } else {
                    prev_spchar = '&';
                    continue;
                }
            } else {
                cur_arg.push(c);
                continue;
            }
        }

        // !! history expansion NOTE: WIP
        /*if c == '!' {
            if cur_quot.is_empty() {
                if prev_char == '!' {
                    cur_arg.push_str(History::last().unwrap());
                    prev_char = ' ';
                } else {
                    prev_char = '!';
                }
            } else {
                cur_arg.push('!');
            }
        }*/

        // Expand home
        if c == '~' {
            if cur_quot.is_empty() {
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

    // Once again, the order is vitally important here
    if has_pipe == true && has_op == true { // both pipes and operators
        ArgTypes::Piped(args, true)
    } else if has_op == true { // one or more operators
        ArgTypes::Norm(args, true)
    } else if has_pipe == true { // one or more pipes
        ArgTypes::Piped(args, false)
    } else { // normal command
        ArgTypes::Norm(args, false)
    }
}

// split vector by '|'
pub fn split_pipes(all: &[String]) -> Vec<&[String]> {
    let mut vec = Vec::new();
    let iter = all.split(|f| f == &"|");
    vec.extend(iter);
    return vec;
}

// split operators, currently only && for now
pub fn split_ops(all: &[String]) -> Vec<&[String]> {
    let mut vec = Vec::new();
    let iter = all.split(|f| f== &"&&");
    vec.extend(iter);
    return vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_ops() {
        assert_eq!(
            split_ops(&vec!["ls".to_string(), "&&".to_string(), "echo".to_string()]),
            vec![vec!["ls".to_string()], vec!["echo".to_string()]]
        );
        assert_eq!(
            split_ops(&vec!["ls".to_string(), "-al".to_string(), "&&".to_string(), "echo".to_string(), "hello space".to_string()]),
            vec![vec!["ls".to_string(), "-al".to_string()], vec!["echo".to_string(), "hello space".to_string()]]
        );
    }

    #[test]
    fn test_split_pipes() {
        assert_eq!(
            split_pipes(&vec!["ls".to_string(), "|".to_string(), "echo".to_string()]),
            vec![vec!["ls".to_string()], vec!["echo".to_string()]]
        );
        assert_eq!(
            split_pipes(&vec!["ls".to_string(), "-al".to_string(), "|".to_string(), "echo".to_string(), "hello space".to_string(), "|".to_string(), "echo".to_string()]),
            vec![vec!["ls".to_string(), "-al".to_string()], vec!["echo".to_string(), "hello space".to_string()], vec!["echo".to_string()]]
        );
    }

    /*#[test]
    fn test_split_line_to_args() {
        assert_eq!(split_to_args("ls".to_string()), vec!["ls"]);
        assert_eq!(split_to_args(" ls".to_string()), vec!["", "ls"]);
        assert_eq!(split_to_args(" ls   ".to_string()), vec!["", "ls"]);
        assert_eq!(split_to_args("   ls   ".to_string()), vec!["", "ls"]);
        assert_eq!(split_to_args("   ls  -a     -l  ".to_string()), vec!["", "ls", "-a", "-l"]);
        assert_eq!(split_to_args("ls -al".to_string()), vec!["ls", "-al"]);
        assert_eq!(split_to_args("ls -a -l".to_string()), vec!["ls", "-a", "-l"]);
        assert_eq!(split_to_args("ls 'single quotes'".to_string()), vec!["ls", "single quotes"]);
        assert_eq!(split_to_args("ls \"double quotes\"".to_string()), vec!["ls", "double quotes"]);
        assert_eq!(split_to_args("ls | wc".to_string()), vec!["ls", "|", "wc"]);
        assert_eq!(split_to_args("echo pipes with args | wc".to_string()), vec!["echo", "pipes", "with", "args", "|", "wc"]);
    }*/
}
