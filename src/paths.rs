use regex::Regex;
use std::env;
use std::path::Path;
use std::str;
use lazy_static::lazy_static;

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

lazy_static! { 
    static ref TILDE_SWAP: Vec<Regex> = vec![
        Regex::new(r"(?P<head> +)~(?P<tail> +)").unwrap(),
        Regex::new(r"(?P<head> +)~(?P<tail>/)").unwrap(),
        Regex::new(r"^(?P<head> *)~(?P<tail>/)",).unwrap(),
        Regex::new(r"(?P<head> +)~(?P<tail> *$)",).unwrap(),
    ];
}

// expands ~ to homedir
pub fn expand_home(text: &str) -> String {
    let mut s = text.to_string();
	for reg in TILDE_SWAP.iter() {
	    let to = format!("$head{}$tail", get_user_home());
		s = reg.replace_all(s.as_str(), to.as_str()).to_string();
	}
    s
}

// substitues ~ for homedir
pub fn condense_home(text: &str) -> String {
	text.replace(&get_user_home(), "~")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn expand_home_test() {
        assert_eq!(expand_home("~/Notes"), [get_user_home(), "Notes".to_string()].join("/"));
        assert_eq!(expand_home("Some~String"), "Some~String".to_string());
        assert_eq!(expand_home("~"), "~".to_string());
        assert_eq!(expand_home("echo ~"), ["echo".to_string(), get_user_home()].join(" "));
	}
}
