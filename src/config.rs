use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub domains: Vec<Domain>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DomainProvider {
    #[serde(rename = "aws")]
    Aws,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    pub provider: DomainProvider,
    pub tld: String,
    pub domain: String,
}

pub fn load_config(path: &Path) -> Result<Configuration, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let config: Configuration = serde_yaml::from_str(&contents)?;
    Ok(config)
}
