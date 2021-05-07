use std::collections::HashMap;

use crate::config::YuiConfig;
use crate::paths;

#[derive(Clone)]
pub struct Context {
    pub config: YuiConfig,
    pub histfile: String,
    pub aliases: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            config: YuiConfig::default(),
            histfile: [paths::get_user_home(), ".yui_history".to_string()].join("/"),
            aliases: HashMap::new(),
        }
    }

    pub fn new_alias(&mut self, alias: String, value: String) {
        self.aliases.insert(alias, value);
    }
}
