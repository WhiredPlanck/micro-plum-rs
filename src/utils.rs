use anyhow::{Ok, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
    static ref LUA_MODULE_EXPAND_PATTERN: Regex = Regex::new(r"^(lua/.+)\.lua$").unwrap();
}

pub fn expand_lua(file: &str) -> Result<Vec<String>> {
    match LUA_MODULE_EXPAND_PATTERN.captures(file) {
        Some(caps) => Ok(vec![file.to_string(), format!("{}/init.lua", &caps[1])]),
        None => Ok(vec![file.to_string()]),
    }
}
