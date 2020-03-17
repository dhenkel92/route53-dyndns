#![cfg_attr(feature = "strict", deny(warnings))]

mod clap_config;
mod config;
mod handler;
mod ip;
mod logger;
mod route53;

use crate::config::{load_config, DomainProvider, Domain};
use crate::handler::{handle_route53, HandlerResult};
use crate::ip::{fetch_ip, IPError};
use clap_config::generate_clap_config;
use logger::initialize_logger;
use futures::future::join_all;
use std::error::Error;
use std::path::Path;

#[macro_use]
extern crate log;

fn log_generic_error(err: Box<dyn Error>) -> Box<dyn Error> {
    error!("{:?}", err);
    err
}

fn log_ip_error(err: IPError) -> IPError {
    warn!("{:?}", err);
    err
}

async fn handle_domain(ip: &str, domain: &Domain) -> HandlerResult {
    let handler = match domain.provider {
        DomainProvider::Aws => handle_route53(ip, domain),
    };
    handler.await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = generate_clap_config().get_matches();
    initialize_logger(&matches).map_err(log_generic_error)?;

    let config_path = Path::new(matches.value_of("config-path").unwrap());
    let configuration = load_config(config_path).map_err(log_generic_error)?;

    debug!("Configuration: {:?}", configuration);

    let ip = fetch_ip().await.map_err(log_ip_error)?;
    debug!("Got IP: {}", &ip);

    let futures = configuration.domains.iter().map(|domain| handle_domain(&ip, domain)).collect::<Vec<_>>();
    let results = join_all(futures).await;
    for result in results {
        match result {
            Err(err) => error!("{:?}", err),
            _ => (),
        }
    }

    Ok(())
}
