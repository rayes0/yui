use regex::Regex;
use std::str;
use std::env;

// get homedir
pub fn get_user_home() -> String {
    match env::var("HOME") {
        Ok(x) => x,
        Err(e) => {
            println!("yui: env HOME error: {:?}", e);
            String::new()
        }
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
