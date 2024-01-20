use anyhow::{Ok, Result};
use regex::Regex;
use serde::Deserialize;
use serde_yaml::Value;
use crate::utils::expand_lua;

#[derive(Deserialize)]
struct Translator {
    dictionary: String,
    prism: Option<String>,
}

#[derive(Deserialize)]
struct Schema {
    translator: Option<Translator>,
}

fn parse_include(obj: &serde_yaml::Value, result: &mut Vec<String>) -> Result<()> {
    let mapping = obj.as_mapping().unwrap();
    for (key, value) in mapping.iter() {
        let key = key.as_str().unwrap();
        if key == "__include" || key == "__patch" {
            let values: Vec<Value>;
            match value {
                Value::String(_) => values = vec![value.to_owned()],
                Value::Sequence(value) => values = value.to_owned(),
                _ => return parse_include(value, result),
            }
            for v in values {
                match v {
                    Value::String(sv) => {
                        if let Some(sv) = sv.split_once(':') {
                            let file = sv.0;
                            if file.ends_with(".yaml") {
                                result.push(file.to_string());
                            } else {
                                result.push(format!("{}.yaml", file));
                            }
                        }
                    },
                    _ => parse_include(&v, result)?,
                }
            }
        } else {
            parse_include(value, result)?
        }
    }

    Ok(())
}

pub fn parse_schema(schema: &serde_yaml::Value) -> Result<Vec<Vec<String>>> {
    let mut result: Vec<String> = Vec::new();
    parse_include(schema, &mut result)?;

    let mapping = schema.as_mapping().unwrap();
    for (key, value) in mapping.iter() {
        let key = key.as_str().unwrap();
        let value = value.as_mapping().unwrap();
        match key {
            "engine" => {
                for component in vec!["processor", "segmentor", "translator", "filter"] {
                    let name = &format!("{}s", component);
                    if value.contains_key(name) {
                        let pattern = Regex::new(format!("^lua_{}@(\\*)?([_a-zA-Z0-9]+(/[_a-zA-Z0-9]+)*)(@[_a-zA-Z0-9]+)?$", component).as_str()).unwrap();
                        let sequence = value[name].as_sequence().unwrap();
                        for item in sequence {
                            let item = item.as_str().unwrap();
                            if pattern.is_match(item) {
                                if let Some(caps) = pattern.captures(item) {
                                    if caps.get(1).is_some() {
                                        result.push(format!("lua/{}.lua", &caps[2]));
                                    } else {
                                        let rime_lua = String::from("rime.lua");
                                        if !result.contains(&rime_lua) {
                                            result.push(rime_lua);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "translator" => {
                if value.contains_key("dictionary") {
                    let dict = value["dictionary"].as_str().unwrap();
                    result.push(format!("{}.yaml", dict));
                }
            },
            "punctuator" => {
                if value.contains_key("import_preset") {
                    let import_preset = value["import_preset"].as_str().unwrap();
                    if import_preset != "default" {
                        result.push(format!("{}.yaml", import_preset));
                    }
                }
            },
            _ => (),
        }
    }

    let result = result.iter()
        .map(|f| expand_lua(f).unwrap() )
        .collect::<Vec<Vec<_>>>();
    Ok(result)
}

pub fn get_binary_names(schema: &Schema) -> Result<(Option<String>, Option<String>)>{
    if let Some(translator) = &schema.translator {
        let dict = &translator.dictionary;
        let prism = match &translator.prism {
            Some(p) => p,
            None => dict,
        };
        return Ok((Some(dict.to_string()), Some(prism.to_string())));
    }

    Ok((None, None))
}
