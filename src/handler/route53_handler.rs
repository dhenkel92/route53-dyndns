use crate::config::Domain;
use crate::handler::types::{HandlerError, HandlerResult};
use crate::route53::{Route53Adapter, Route53Error};
use rusoto_core::Region;
use rusoto_route53::{HostedZone, Route53, Route53Client};

fn map_route53_error(err: Route53Error) -> HandlerError {
    match err {
        Route53Error::RecordSetNotFound => HandlerError::RecordNotFound,
        Route53Error::HostedZoneNotFound => HandlerError::RecordNotFound,
        Route53Error::LibError(err) => HandlerError::ProviderError(err),
    }
}

fn extract_id_from_hz(hosted_zone: &HostedZone) -> String {
    let split: Vec<&str> = hosted_zone.id.split('/').collect();
    split[2].to_string()
}

pub struct Route53Handler<T>
where
    T: Route53,
{
    adapter: Route53Adapter<T>,
}

impl Route53Handler<Route53Client> {
    pub fn new() -> Route53Handler<Route53Client> {
        let adapter = Route53Adapter::<Route53Client>::new(Route53Client::new(Region::UsEast1));
        Route53Handler { adapter }
    }
}

impl<T> Route53Handler<T>
where
    T: Route53,
{
    pub async fn handle(&self, ip: &str, domain: &Domain) -> HandlerResult {
        // as Route53 is global and not bound to a region we have to use the "default"
        // region, which is us-east-1 in this case.
        let hosted_zone = self
            .adapter
            .find_hosted_zone_by_name(&domain.tld)
            .await
            .map_err(map_route53_error)?;

        let hosted_zone_id = extract_id_from_hz(&hosted_zone);

        let record_set = self
            .adapter
            .upsert_record(&hosted_zone_id, &domain.domain, ip)
            .await
            .map_err(map_route53_error)?;

        info!("{:?}", record_set);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod map_route_53_error {
        use crate::handler::route53_handler::map_route53_error;
        use crate::handler::HandlerError;
        use crate::route53::Route53Error;

        #[test]
        fn map_lib_error() {
            let route53_err = Route53Error::LibError("Lib error".to_string());
            let handler_err = map_route53_error(route53_err);
            assert_eq!(
                handler_err,
                HandlerError::ProviderError("Lib error".to_string())
            )
        }
    }
}
