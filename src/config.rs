use std::default::Default;
use std::convert::Into;
use std::process::exit;

use rustyline::config::*;
use rustyline::config::{
    CompletionType::*,
    EditMode::*,
    //BellStyle::*,
    ColorMode::*,
};
use crate::CONFIG;

use colored::Color;

pub struct YuiConfig {
    pub hist_ign_space: bool,
    pub hist_ign_dups: bool,
    pub hist_max_size: usize,
    pub completion_type: CompletionType,
    pub completion_limit: usize,
    pub hinting_color: Color,
    pub keyseq_timeout: i32,
    pub edit_mode: EditMode,
    pub auto_add_history: bool,
    pub bell_style: BellStyle,
    pub color_mode: ColorMode,
    pub tab_stop: usize,
    pub check_cur_pos: bool,
    pub indent_size: usize,
    pub bracketed_paste: bool,
}

// Define defaults here
impl Default for YuiConfig {
    fn default() -> Self {
        YuiConfig {
            hist_ign_space: true,
            hist_ign_dups: false,
            hist_max_size: 1000,
            completion_type: CompletionType::List,
            completion_limit: 50,
            hinting_color: Color::BrightBlack,
            keyseq_timeout: 10,
            edit_mode: EditMode::Emacs,
            auto_add_history: true,
            bell_style: BellStyle::None,
            color_mode: ColorMode::Enabled,
            tab_stop: 4,
            check_cur_pos: false,
            indent_size: 2,
            bracketed_paste: true,
        }
    }
}

pub fn setblock_parse_and_exec(setline: &String) -> bool {
    let mut split = setline.split("=");
    let key = split.next().unwrap();
    let raw = split.next().unwrap();
    convert_and_set_key(&key, &raw)
}

pub fn convert_and_set_key(key: &str, raw: &str) -> bool {
    // TODO: find cleaner way to do this?
    match key {
        "hist_ign_space" => CONFIG.lock().unwrap().hist_ign_space = string_to_type(raw, &"boolean").into(),
        "hist_ign_dups" => CONFIG.lock().unwrap().hist_ign_dups = string_to_type(raw, &"boolean").into(),
        "hist_max_size" => CONFIG.lock().unwrap().hist_max_size = string_to_type(raw, &"size").into(),
        "completion_type" => CONFIG.lock().unwrap().completion_type = string_to_type(raw, &"complete").into(),
        "completion_limit" => CONFIG.lock().unwrap().completion_limit = string_to_type(raw, &"size").into(),
        "hinting_color" => CONFIG.lock().unwrap().hinting_color = string_to_type(raw, &"colorname").into(),
        "keyseq_timeout" => CONFIG.lock().unwrap().keyseq_timeout = string_to_type(raw, &"int32").into(),
        "edit_mode" => CONFIG.lock().unwrap().edit_mode = string_to_type(raw, &"edit").into(),
        "auto_add_history" => CONFIG.lock().unwrap().auto_add_history = string_to_type(raw, &"boolean").into(),
        "bell_style" => CONFIG.lock().unwrap().bell_style = string_to_type(raw, &"bell").into(),
        "color_mode" => CONFIG.lock().unwrap().color_mode = string_to_type(raw, &"color").into(),
        "tab_stop" => CONFIG.lock().unwrap().tab_stop = string_to_type(raw, &"size").into(),
        "check_cur_pos" => CONFIG.lock().unwrap().check_cur_pos = string_to_type(raw, &"boolean").into(),
        "indent_size" => CONFIG.lock().unwrap().indent_size = string_to_type(raw, &"size").into(),
        "bracketed_paste" => CONFIG.lock().unwrap().bracketed_paste = string_to_type(raw, &"boolean").into(),
        _ => return false,
    }
    true
}

// We have to convert the strings in the config to appropriate types
enum ConfigTypes {
    Boolean(bool),
    Size(usize),
    Num(i32),
    Completion(CompletionType),
    ColorNames(Color),
    EditMode(EditMode),
    BellStyle(BellStyle),
    Color(ColorMode),
    Error(), // For if the value in the config is invalid
}

// TODO: write a proc macro for this, will be helpful if we add more settings options
impl Into<bool> for ConfigTypes {
    fn into(self) -> bool {
        if let ConfigTypes::Boolean(b) = self { b } else { eprintln!("set: unsupported value"); exit(1) }
    }
}
impl Into<usize> for ConfigTypes {
    fn into(self) -> usize {
        if let ConfigTypes::Size(s) = self { s } else { eprintln!("set: unsupported value"); exit(1) }
    }
}
impl Into<i32> for ConfigTypes {
    fn into(self) -> i32 {
        if let ConfigTypes::Num(s) = self { s } else { eprintln!("set: unsupported value"); exit(1) }
    }
}
impl Into<CompletionType> for ConfigTypes {
    fn into(self) -> CompletionType {
        if let ConfigTypes::Completion(c) = self { c } else { eprintln!("set: unsupported value"); exit(1) }
    }
}
impl Into<Color> for ConfigTypes {
    fn into(self) -> Color {
        if let ConfigTypes::ColorNames(c) = self { c } else { eprintln!("set: unsupported value"); exit(1) }
    }
}
impl Into<EditMode> for ConfigTypes {
    fn into(self) -> EditMode {
        if let ConfigTypes::EditMode(m) = self { m } else { eprintln!("set: unsupported value"); exit(1) }
    }
}
impl Into<BellStyle> for ConfigTypes {
    fn into(self) -> BellStyle {
        if let ConfigTypes::BellStyle(b) = self { b } else { eprintln!("set: unsupported value"); exit(1) }
    }
}
impl Into<ColorMode> for ConfigTypes {
    fn into(self) -> ColorMode {
        if let ConfigTypes::Color(c) = self { c } else { eprintln!("set: unsupported value"); exit(1) }
    }
}

// We have to convert strings from the config to proper formats.. Pain..
fn string_to_type(string: &str, target: &str) -> ConfigTypes {
    let matcher = string.to_lowercase(); // case insensitive matching
    match target {
        "boolean" => {
            if matcher == "true" {
                ConfigTypes::Boolean(true)
            } else {
                ConfigTypes::Boolean(false)
            }
        },
        "size" => {
            ConfigTypes::Size(string.parse::<usize>().unwrap())
        },
        "int32" => {
            ConfigTypes::Num(string.parse::<i32>().unwrap())
        },
        "complete" => {
            if matcher == "circular" {
                ConfigTypes::Completion(Circular)
            } else if matcher == "list" {
                ConfigTypes::Completion(List)
            } else {
                ConfigTypes::Error()
            }
        },
        "colorname" => {
            match matcher.as_str() {
                "black" => ConfigTypes::ColorNames(Color::Black),
                "red" => ConfigTypes::ColorNames(Color::Red),
                "green" => ConfigTypes::ColorNames(Color::Green),
                "yellow" => ConfigTypes::ColorNames(Color::Yellow),
                "blue" => ConfigTypes::ColorNames(Color::Blue),
                "magenta" => ConfigTypes::ColorNames(Color::Magenta),
                "cyan" => ConfigTypes::ColorNames(Color::Cyan),
                "white" => ConfigTypes::ColorNames(Color::White),
                "brightblack" => ConfigTypes::ColorNames(Color::BrightBlack),
                "brightred" => ConfigTypes::ColorNames(Color::BrightRed),
                "brightgreen" => ConfigTypes::ColorNames(Color::BrightGreen),
                "brightyellow" => ConfigTypes::ColorNames(Color::BrightYellow),
                "brightblue" => ConfigTypes::ColorNames(Color::BrightBlue),
                "brightmagenta" => ConfigTypes::ColorNames(Color::BrightMagenta),
                "brightcyan" => ConfigTypes::ColorNames(Color::BrightCyan),
                "brightwhite" => ConfigTypes::ColorNames(Color::BrightWhite),
                _ => ConfigTypes::Error(),
            }
        },
        "edit" => {
            if matcher == "emacs" {
                ConfigTypes::EditMode(Emacs)
            } else if matcher == "vi" {
                ConfigTypes::EditMode(Vi)
            } else {
                ConfigTypes::Error()
            }
        },
        "bell" => {
            // need to state manually because "None" conflicts with the "None" used above in the
            // match statements
            if matcher == "audible" {
                ConfigTypes::BellStyle(BellStyle::Audible)
            } else if matcher == "visible" {
                ConfigTypes::BellStyle(BellStyle::Visible)
            } else if matcher == "none" {
                ConfigTypes::BellStyle(BellStyle::None)
            } else {
                ConfigTypes::Error()
            }
        },
        "color" => {
            if matcher == "enabled" {
                ConfigTypes::Color(Enabled)
            } else if matcher == "forced" {
                ConfigTypes::Color(Forced)
            } else if matcher == "disabled" {
                ConfigTypes::Color(Disabled)
            } else {
                ConfigTypes::Error()
            }
        },
        _ => ConfigTypes::Error(),
    }
}
