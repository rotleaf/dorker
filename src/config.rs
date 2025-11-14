use crate::utilities::Ret;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_keys: Vec<String>,
    pub location: String,
}

pub fn read_config(config_path: &str) -> Ret<()> {
    let content = fs::read_to_string(config_path)?;
    let conf = serde_yaml::from_str::<Config>(&content)?;
    dbg!(conf);
    Ok(())
}
