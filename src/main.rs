mod clap_config;
mod config;
mod handler;
mod logger;
mod route53;

use crate::config::{load_config, DomainProvider};
use crate::handler::handle_route53;
use clap_config::generate_clap_config;
use logger::initialize_logger;
use std::error::Error;
use std::path::Path;

#[macro_use]
extern crate log;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = generate_clap_config().get_matches();
    initialize_logger(&matches)?;

    let config_path = Path::new(matches.value_of("config-path").unwrap());
    let configuration = load_config(config_path)?;

    debug!("Configuration: {:?}", configuration);

    for domain in configuration.domains {
        match domain.provider {
            DomainProvider::Aws => handle_route53(&domain)?,
        }
    }

    Ok(())
}
