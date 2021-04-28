use regex::Regex;
use std::env;
use std::path::Path;
use std::str;

// get homedir
pub fn get_user_home() -> String {
	match env::var("HOME") {
		Ok(x) => x,
		Err(e) => {
			println!("yui: error in getting HOME env: {:?}", e);
			String::new()
		}
	}
}

pub fn get_user_config() -> Option<String> {
	let pri1 = [get_user_home(), ".yuirc".to_string()].join("/");
	let pri2 = [check_xdg(), "yui/yuirc".to_string()].join("/");
	let pri3 = [get_user_home(), ".config/yui/yuirc".to_string()].join("/");
	fn check_xdg() -> String {
		if let Ok(p) = env::var("XDG_CONFIG_HOME") {
			p
		} else {
			"".to_string()
		}
	}
	if Path::new(&pri1).exists() {
		Some(pri1)
	} else if Path::new(&pri2).exists() {
		Some(pri2)
	} else if Path::new(&pri3).exists() {
		Some(pri3)
	} else {
		None
	}
}

// expands ~ to homedir
pub fn expand_home(text: &str) -> String {
	let mut s: String = text.to_string();
	let v = vec![
		r"(?P<head> +)~(?P<tail> +)",
		r"(?P<head> +)~(?P<tail>/)",
		r"^(?P<head> *)~(?P<tail>/)",
		r"(?P<head> +)~(?P<tail> *$)",
	];
	for item in &v {
		let re;
		if let Ok(x) = Regex::new(item) {
			re = x;
		} else {
			return String::new();
		}
		let home = get_user_home();
		let ss = s.clone();
		let to = format!("$head{}$tail", home);
		let result = re.replace_all(ss.as_str(), to.as_str());
		s = result.to_string();
	}
	s
}

// substitues ~ for homedir
pub fn condense_home(text: &str) -> String {
	text.replace(&get_user_home(), "~")
}
