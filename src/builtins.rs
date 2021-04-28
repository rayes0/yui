use regex::Regex;
use std::{env, io::ErrorKind, path::Path};

use crate::paths;

pub fn cd(d: Vec<&String>) {
	let new_dir;
	if d.is_empty() {
		new_dir = paths::get_user_home();
	} else if d.iter().count() > 2 {
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

pub fn echo(s: Vec<&String>) {
	let mut to_print = String::new();
	for word in s.iter() {
		to_print.push_str(word);
		to_print.push(' ');
	}
	println!("{}", to_print.trim());
}

pub fn export(s: Vec<&String>) {
	for input in s.iter() {
		if let Ok(re) = Regex::new(r"^([a-zA-Z0-9_]+)=(.*)$") {
			if !re.is_match(input) {
				println!("yui: export: invalid usage");
			}

			for cap in re.captures_iter(input) {
				let name = cap[1].to_string();
				let value = paths::expand_home(&cap[2]);
				env::set_var(name, &value);
			}
		} else {
			eprintln!("yui: export: regex error");
		}
	}
}

pub fn set(s: Vec<&String>) {
	for input in s.iter() {
		if let Ok(re) = Regex::new(r"^([a-zA-Z0-9_]+)=(.*)$") {
			if !re.is_match(input) {
				eprintln!("yui: set: invalid usage\n  set OPTION=VALUE");
			}

			for cap in re.captures_iter(input) {
				let name = cap[1].to_string();
				let value = paths::expand_home(&cap[2]);
				if crate::config::convert_and_set_key(&name, &value) == false {
					eprintln!("Invalid option: {}", name);
				}
			}
		} else {
			eprintln!("yui: set: regex error");
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::env;

	#[test]
	fn cd_basic_test() {
		let path = "/tmp".to_string();
		let vec = vec![&path];
		cd(vec);
		let new = env::current_dir().expect("can't get current dir");
		assert_eq!("/tmp", new.as_os_str().to_str().unwrap());
	}
}
