use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    domains: Vec<Domain>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ConfigurationType {
    #[serde(rename = "route53")]
    ROUTE53,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    #[serde(rename = "type")]
    config_type: ConfigurationType,
    tld: String,
    domain: String,
}

pub fn load_config(path: &Path) -> Result<Configuration, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let config: Configuration = serde_yaml::from_str(&contents)?;
    Ok(config)
}
