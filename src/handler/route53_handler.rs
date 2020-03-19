use crate::config::Domain;
use crate::handler::types::{HandlerError, HandlerResult};
use crate::route53::{Route53Adapter, Route53Error};
use rusoto_core::Region;
use rusoto_route53::{HostedZone, Route53Client};

fn map_route53_error(err: Route53Error) -> HandlerError {
    // todo: make it proper :)
    HandlerError::ProviderError(format!("{:?}", err))
}

fn extract_id_from_hz(hosted_zone: &HostedZone) -> String {
    let split: Vec<&str> = hosted_zone.id.split('/').collect();
    split[2].to_string()
}

pub async fn handle_route53(ip: &str, domain: &Domain) -> HandlerResult {
    // as Route53 is global and not bound to a region we have to use the "default"
    // region, which is us-east-1 in this case.
    let route53_adapter = Route53Adapter::<Route53Client>::new(Route53Client::new(Region::UsEast1));
    let hosted_zone = route53_adapter
        .find_hosted_zone_by_name(&domain.tld)
        .await
        .map_err(map_route53_error)?;

    let hosted_zone_id = extract_id_from_hz(&hosted_zone);

    let record_set = route53_adapter
        .upsert_record(&hosted_zone_id, &domain.domain, ip)
        .await
        .map_err(map_route53_error)?;

    info!("{:?}", record_set);
    Ok(())
}
