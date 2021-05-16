use std::collections::HashMap;

use crate::config::Config;
use crate::paths;

#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub histfile: String,
    pub aliases: HashMap<String, String>,
    pub laststatus: i32, // exit status of last command
}

impl Context {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            histfile: [paths::get_user_home(), ".yui_history".to_string()].join("/"),
            aliases: HashMap::new(),
            laststatus: 0,
        }
    }

    pub fn new_alias(&mut self, alias: String, value: String) {
        self.aliases.insert(alias, value);
    }
}
