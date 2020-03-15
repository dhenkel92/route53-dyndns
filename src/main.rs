mod clap_config;
mod logger;
mod route53;

use crate::route53::Route53Adapter;
use clap_config::generate_clap_config;
use logger::initialize_logger;
use std::error::Error;

#[macro_use]
extern crate log;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = generate_clap_config().get_matches();
    initialize_logger(&matches)?;

    let route53_adapter = Route53Adapter::new();
    let hosted_zones = route53_adapter.list_hosted_zones()?;
    for zone in hosted_zones {
        info!("Found Hosted Zone: {:?} ({})", zone.name, zone.id);
        let record_sets = route53_adapter.list_record_sets(&zone)?;
        for set in record_sets {
            info!("Record set: [{}] {}", set.type_, set.name);
        }
    }

    Ok(())
}
