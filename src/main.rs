#![cfg_attr(feature = "strict", deny(warnings))]

mod config;
mod handler;
mod ip;
mod logger;
mod route53;

use crate::config::{load_config, Domain, DomainProvider};
use crate::handler::{HandlerResult, Route53Handler};
use crate::ip::{fetch_ip, IPError};
use clap::{load_yaml, App};
use futures::future::join_all;
use logger::initialize_logger;
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
    match domain.provider {
        DomainProvider::Aws => Route53Handler::new().handle(ip, domain).await,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let clap_config = load_yaml!("clap-config.yml");
    let matches = App::from(clap_config)
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(&*("\n".to_owned() + clap::crate_description!()))
        .get_matches();

    initialize_logger(&matches).map_err(log_generic_error)?;

    let config_path = Path::new(matches.value_of("config-path").unwrap());
    let configuration = load_config(config_path).map_err(log_generic_error)?;

    debug!("Configuration: {:?}", configuration);

    let ip = fetch_ip().await.map_err(log_ip_error)?;
    debug!("Got IP: {}", &ip);

    let futures = configuration
        .domains
        .iter()
        .map(|domain| handle_domain(&ip, domain))
        .collect::<Vec<_>>();
    let results = join_all(futures).await;
    for result in results {
        if let Err(err) = result {
            error!("{:?}", err);
        }
    }

    Ok(())
}
