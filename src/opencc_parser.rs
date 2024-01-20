use anyhow::Result;

struct SegDict {
    r#type: String,
    file: String,
}

struct ChainDict {
    r#type: String,
    file: Option<String>,
    dicts: Vec<SegDict>,
}

struct Segmentation {
    r#type: String,
    dict: SegDict,
}

struct OpenCCConfig {
    name: String,
    segmentation: Segmentation,
    conversion_chain: Vec<ChainDict>,
}

pub fn parse_opencc(config: &OpenCCConfig) -> Result<Vec<Vec<String>>> {
    let mut result: Vec<String> = Vec::new();
    let file = &config.segmentation.dict.file;
    result.push(file.to_string());
    for dict in &config.conversion_chain {
        if let Some(file) = &dict.file {
            if !result.contains(file) { result.push(file.to_string()) }
        }
        if !dict.dicts.is_empty() {
            for item in &dict.dicts {
                if !result.contains(&item.file) { result.push(file.to_string()) }
            }
        }
    }

    let result = result.iter()
        .map(|f| vec![f.to_string()] )
        .collect::<Vec<Vec<_>>>();
    Ok(result)
}