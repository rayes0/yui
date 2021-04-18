use std::process::Command;
//use std::io::stdin;
//use std::io::stdout;
//use std::env;
//use std::path::Path;
//use std::io::ErrorKind;
//use std::fs;

mod builtins;

// Parse the text entered by user, checking for pipes, redirections, etc.
pub fn parse_entry(u: String) {
    /*
    // check for piped commands
    let mut cmd_parts = u.trim().split("|").peekable();
    // loop over each individual command
    while let Some(command) = cmd_parts.next() {
        let stdout = spawn_cmd(command.trim().to_string());
    }
    */
    spawn_cmd(u.trim().to_string());
}

// This is the function that actually runs the commands
fn spawn_cmd(c: String) {
    // Parse args
    let mut cmd_split = c.trim().split_whitespace();
    let cmd = cmd_split.next().unwrap();
    let args = cmd_split;
    // Need to check for builtins
    match cmd {
        "cd" => builtins::cd(args.collect()),
        "echo" => builtins::echo(args.collect()),
        //"history" => builtins::history(),
        "version" => println!("yui, version 0.0\nA bash-like shell focused on speed and simplicity.\n"),
        "builtins" =>println!("Builtin commands:\ncd\necho\nversion\nexit"),
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
