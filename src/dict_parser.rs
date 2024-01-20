use anyhow::Result;
use serde_yaml::Value;

pub fn parse_dict(schema: &serde_yaml::Value) -> Result<Vec<Vec<String>>> {
    let mut result: Vec<String> = Vec::new();
    let mapping = schema.as_mapping().unwrap();
    for (key, value) in mapping.iter() {
        let key = key.as_str().unwrap();
        match key {
            "import_tables" => {
                if let Value::Sequence(tables) = value {
                    for file in tables {
                        let file = file.as_str().unwrap();
                        result.push(format!("{}.dict.yaml", file));
                    }
                }
            },
            "vocabulary" => {
                let file = value.as_str().unwrap();
                result.push(format!("{}.txt", file));
            }
            _ => (),
        }
    }

    let result = result.iter()
        .map(|f| vec![f.to_string()] )
        .collect::<Vec<Vec<_>>>();
    Ok(result)
}
