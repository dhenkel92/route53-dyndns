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

#[cfg(test)]
mod tests {
    mod load_config {
        use crate::config::load_config;
        use std::path::Path;

        #[test]
        fn should_load_config_and_include_at_least_one_domain() {
            let path = Path::new(file!())
                .parent()
                .unwrap()
                .join("../example-config.yaml");
            let config = load_config(path.as_path()).unwrap();
            assert!(config.domains.len() > 0)
        }

        #[test]
        fn should_fail_while_loading_config() {
            let path = Path::new("invalid-path");
            let err = load_config(path).err();
            assert_ne!(err.unwrap().to_string(), "")
        }
    }
}
