use std::io::{Read, Write};
use std::process::{Command, Stdio};

use crate::builtins;
use crate::parser;
use crate::context::Context;
//use crate::ALIASES;

pub fn choose_and_run(ctx: &mut Context, int: bool, raw: parser::ArgTypes) {
	match raw {
		parser::ArgTypes::Norm(data, false) => {
			// normal command
			spawn_cmd(ctx, int, &data);
		}
		parser::ArgTypes::Piped(data, false) => {
			// contains one or more pipes
			let parts = parser::split_pipes(&data);
			spawn_piped(&parts);
		}
		parser::ArgTypes::Norm(data, true) => {
			// normal command with op
			// TODO: make this proper once we get exit code handling
			let parts = parser::split_ops(&data);
			spawn_chained(&parts);
		}
		parser::ArgTypes::Piped(_data, true) => {
			// contains one or more pipes with op in one of them
			eprintln!("yui: Multiple operators with ambiguous precedence");
		}
	};
}

pub fn spawn_cmd(ctx: &mut Context, int: bool, raw: &[String]) {
	let mut cmd: Vec<String>;
	if let Some(d) = check_aliases(ctx, &raw[0]) {
		if int == true {
			// only expand aliases in interactive mode
			cmd = raw.to_vec();
			let _ = cmd.splice(..1, d.iter().cloned());
		} else {
			cmd = raw.to_vec();
		}
	} else {
		cmd = raw.to_vec();
	}
	let mut cmd_split = cmd.iter();
	let cmd = cmd_split.next().unwrap(); // first one will be the command
	let args = cmd_split.clone();
	// check for builtins
	if check_builtins(ctx, &mut cmd.as_str(), &cmd_split.collect::<Vec<&String>>()) {
		return;
	} else {
		// Run commands, echo any errors
		let child_cur = Command::new(cmd).args(args).spawn();
		match child_cur {
			Ok(mut child) => {
				if let Err(m) = child.wait() {
					eprintln!("{}", m);
				}
			}
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
		} else {
			// this means we are on the last command
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
		let spawn = Command::new(iter.next().unwrap()).args(iter).spawn();
		if let Err(m) = spawn.unwrap().wait() {
			eprintln!("{}", m);
			break;
		}
	}
}

fn check_builtins(ctx: &mut Context, c: &str, a: &[&String], ) -> bool {
	let args = a.to_vec();
	match c {
		"cd" => builtins::cd(&args),
		"echo" => builtins::echo(&args),
		"export" => builtins::export(&args),
		"set" => builtins::set(ctx, &args),
		"alias" => builtins::alias(ctx, &args),
        "history" => builtins::history(&ctx.histfile, &args),
		"version" => {
			println!("yui, version 0.0\nA bash-like shell focused on speed and simplicity.\n")
		}
		"builtins" => println!("Builtin commands:\ncd\necho\nversion\nexport\nset\nexit"),
		_ => return false,
	}
	true
}

fn check_aliases(ctx: &mut Context, c: &String) -> Option<Vec<String>> {
	//let text = &c.to_string();
	let aliases = &mut ctx.aliases;
	if aliases.contains_key(c) {
		let key = &aliases[c];
		if let parser::ArgTypes::Norm(c, _) = parser::split_to_args(key.to_string()) {
			Some(c)
		} else {
			None // should never happen
		}
	} else {
		//c.to_string()
		None
	}
}
