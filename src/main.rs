mod clap_config;
mod config;
mod handler;
mod logger;
mod route53;
mod ip;

use crate::config::{load_config, DomainProvider};
use crate::handler::{handle_route53, HandlerError};
use clap_config::generate_clap_config;
use logger::initialize_logger;
use std::error::Error;
use std::path::Path;
use crate::ip::{IPError, fetch_ip};

#[macro_use]
extern crate log;

fn log_generic_error(err: Box<dyn Error>) -> Box<dyn Error> {
    error!("{:?}", err);
    err
}

fn log_handler_error(err: HandlerError) {
    error!("{:?}", err);
}

fn log_ip_error(err: IPError) -> IPError {
    warn!("{:?}", err);
    err
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = generate_clap_config().get_matches();
    initialize_logger(&matches).map_err(log_generic_error)?;

    let config_path = Path::new(matches.value_of("config-path").unwrap());
    let configuration = load_config(config_path).map_err(log_generic_error)?;

    debug!("Configuration: {:?}", configuration);

    let ip = fetch_ip().map_err(log_ip_error)?;
    debug!("Got IP: {}", &ip);

    for domain in configuration.domains {
        let _ = match domain.provider {
            DomainProvider::Aws => handle_route53(&ip, &domain).map_err(log_handler_error),
        };
    }

    Ok(())
}
