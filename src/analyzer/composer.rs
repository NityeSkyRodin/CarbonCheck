use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct ComposerJson {
    pub require: Option<std::collections::HashMap<String, String>>,
    pub name: Option<String>,
}

///  Reads a composer.json file and returns a ComposerJson struct.
/// 
/// # Arguments 
/// 
/// * `path`:  The path to the composer.json file.
/// 
/// returns: Result<ComposerJson, Box<dyn Error, Global>> 
/// 
/// # Examples 
/// 
/// ```
/// 
/// ```
pub fn read_composer_json(path: &str) -> Result<ComposerJson, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let parsed: ComposerJson = serde_json::from_str(&content)?;
    Ok(parsed)
}
