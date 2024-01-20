use anyhow::{Ok, Result};
use lazy_static::lazy_static;
use luaparse::{ast::{FunctionArgs, FunctionCallee, Statement}, token::TokenValue};
use regex::Regex;
use std::str;
use crate::utils::expand_lua;

lazy_static!{
    static ref LUA_MODULE_NAME_PATTERN: Regex = Regex::new("'[_a-zA-Z0-9./]+'|\"[_a-zA-Z0-9./]+\"").unwrap();
}

pub fn parse_lua(content: &str) -> Result<Vec<Vec<String>>> {
    let mut result: Vec<String> = Vec::new();
    let block = luaparse::parse(content).unwrap();
    let statements = block.statements;
    for statement in statements {
        if let Statement::FunctionCall(call) = statement {
            let (callee, args) = (call.callee, call.args);
            if let FunctionCallee::Method { name, .. } = callee {
                if let TokenValue::Ident(ident) = name.0.token.value {
                    if ident == "require" || ident == "dofile" {
                        if let FunctionArgs::StringLit(lit) = args {
                            if let TokenValue::String { value, .. } = lit.0.token.value {
                                let module = str::from_utf8(value.as_ref()).unwrap();
                                if LUA_MODULE_NAME_PATTERN.is_match(module) {
                                    result.push(format!("lua/{}.lua", module.replace(".", "/")));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let result = result.iter()
        .map(|f| expand_lua(f).unwrap() )
        .collect::<Vec<Vec<_>>>();
    Ok(result)
}
