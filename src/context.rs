use crate::config::YuiConfig;
use std::collections::HashMap;

pub struct Context {
    config: YuiConfig,
    aliases: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            config: YuiConfig::default(),
            aliases: HashMap::new(),
        }
    }

    pub fn new_alias(&mut self, alias: String, value: String) {
        self.aliases.insert(alias, value);
    }
}
