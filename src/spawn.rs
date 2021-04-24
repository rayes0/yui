use std::process::Command;

use crate::builtins;
use crate::parser;

pub fn choose_and_run(raw: parser::ArgTypes) {
    match raw {
        parser::ArgTypes::Norm(data) => { // normal command
            spawn_cmd(&data);
            return;
        },
        parser::ArgTypes::Piped(data) => { // contains a pipe
            spawn_cmd(&data);
            return;
        },
    };
}

pub fn spawn_cmd(c: &Vec<String>) {
//pub fn spawn_cmd(c: &parser::ArgTypes) {
    let mut cmd_split = c.iter();
    let cmd = cmd_split.next().unwrap(); // first one will be the command
    let args = cmd_split;
    // check for builtins
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
                Ok(mut child) => { 
                    if let Err(m) = child.wait() {
                        eprintln!("{}", m);
                    }
                },
                Err(e) => eprintln!("{}", e),
            }
        }
    };
}

/*fn spawn_piped(c: &Vec<String>, total: usize) {
    let mut first_cmd = Command::new("ls").arg("/").stdout(Stdio::piped()).spawn().unwrap();

    let mut second_cmd = Command::new("grep").arg("etc").stdin(Stdio::piped()).stdout(Stdio::inherit()).spawn().unwrap();

    if let Some(ref mut stdout) = first_cmd.stdout {
        if let Some(ref mut stdin) = second_cmd.stdin {
            let mut buf: Vec<u8> = Vec::new();
            stdout.read_to_end(&mut buf).unwrap();
            stdin.write_all(&buf).unwrap();
        }
    };
}*/
