use std::process::{Command, Stdio};
use std::io::{Read, Write};

use crate::builtins;
use crate::parser;

pub fn choose_and_run(raw: parser::ArgTypes) {
    match raw {
        parser::ArgTypes::Norm(data, false) => { // normal command
            spawn_cmd(&data);
        },
        parser::ArgTypes::Piped(data, false) => { // contains one or more pipes
            let parts = parser::split_pipes(&data);
            spawn_piped(&parts);
        },
        parser::ArgTypes::Norm(data, true) => { // normal command with op
            // TODO: make this proper once we get exit code handling
            let parts = parser::split_ops(&data);
            spawn_chained(&parts);
        },
        parser::ArgTypes::Piped(_data, true) => { // contains one or more pipes with op in one of them
            eprintln!("yui: Multiple operators with ambiguous precedence");
        },
    };
}

pub fn spawn_cmd(c: &[String]) {
    let mut cmd_split = c.iter();
    let cmd = cmd_split.next().unwrap(); // first one will be the command
    let args = cmd_split.clone();
    // check for builtins
    if check_builtins(&mut cmd.as_str(), &cmd_split.collect::<Vec<&String>>()) {
        return;
    } else {
        // Run commands, echo any errors
        let child_cur = Command::new(cmd).args(args).spawn();
        match child_cur {
            Ok(mut child) => { 
                if let Err(m) = child.wait() {
                    eprintln!("{}", m);
                }
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn spawn_piped(all: &[&[String]]) {
    let mut cmds = all.iter().peekable(); // peekable so we know when we are on the last cmd

    // split off and spawn first cmd
    let mut first_cmd = cmds.next().unwrap().iter();
    let mut first_cmd_spawn = Command::new(first_cmd.next().unwrap())
        .args(first_cmd)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // buffer for writing stdout into
    let mut buf: Vec<u8> = Vec::new();
    let mut store_stdout = first_cmd_spawn.stdout.take();

    if let Err(e) = first_cmd_spawn.wait() {
        eprintln!("yui: pipe error: {}", e);
    }

    while let Some(c) = cmds.next() {
        let mut iter = c.iter();
        if cmds.peek().is_some() {
            let mut middle_cmd_spawn = Command::new(iter.next().unwrap())
                .args(iter)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            // Read previous stdout into current stdin
            if let Some(ref mut prev_stdout) = store_stdout {
                if let Some(ref mut cur_stdin) = middle_cmd_spawn.stdin {
                    prev_stdout.read_to_end(&mut buf).unwrap();
                    cur_stdin.write_all(&buf).unwrap();
                }
            }

            store_stdout = middle_cmd_spawn.stdout.take(); // overwrite the current stdout, which
            // will become the previous stdout in the next round

            if let Err(e) = middle_cmd_spawn.wait() {
                eprintln!("yui: pipe error: {}", e);
            }
        } else { // this means we are on the last command
            let mut last_cmd_spawn = Command::new(iter.next().unwrap())
                .args(iter)
                .stdin(Stdio::piped())
                .stdout(Stdio::inherit())
                .spawn()
                .unwrap();

                store_stdout.unwrap().read_to_end(&mut buf).unwrap();
                last_cmd_spawn.stdin.take().unwrap().write_all(&buf).unwrap();

                if let Err(e) = last_cmd_spawn.wait() {
                    eprintln!("yui: pipe error: {}", e);
                }

            break;
        }
        buf = Vec::new(); // clear buffer
    }
}

fn spawn_chained(all: &[&[String]]) {
    for c in all.iter() {
        let mut iter = c.iter();
        let mut spawn = Command::new(iter.next().unwrap()).args(iter).spawn().unwrap();
        if let Err(m) = spawn.wait() {
            eprintln!("{}", m);
            break;
        }
    }
}

fn check_builtins(c: &str, a: &Vec<&String>) -> bool {
    let args = a.to_vec();
    match c {
        "cd" => builtins::cd(args),
        "echo" => builtins::echo(args),
        "export" => builtins::export(args),
        "set" => builtins::set(args),
        //"history" => builtins::history(),
        "version" => println!("yui, version 0.0\nA bash-like shell focused on speed and simplicity.\n"),
        "builtins" => println!("Builtin commands:\ncd\necho\nversion\nexport\nset\nexit"),
        _ => return false,
    }
    return true;
}
