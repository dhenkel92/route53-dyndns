use crate::config::Domain;
use crate::handler::types::{HandlerError, HandlerResult};
use crate::route53::{Route53Adapter, Route53Error};
use rusoto_route53::HostedZone;

fn map_route53_error(err: Route53Error) -> HandlerError {
    // todo: make it proper :)
    HandlerError::ProviderError(format!("{:?}", err))
}

fn extract_id_from_hz(hosted_zone: &HostedZone) -> String {
    let split: Vec<&str> = hosted_zone.id.split('/').collect();
    split[2].to_string()
}

pub fn handle_route53(domain: &Domain) -> HandlerResult {
    let route53_adapter = Route53Adapter::new();
    let hosted_zone = route53_adapter
        .find_hosted_zone_by_name(&domain.tld)
        .map_err(map_route53_error)?;

    let hosted_zone_id = extract_id_from_hz(&hosted_zone);

    let record_set = route53_adapter
        .upsert_record(&hosted_zone_id, &domain.domain, "127.0.0.1")
        .map_err(map_route53_error)?;

    info!("{:?}", record_set);
    Ok(())
}
