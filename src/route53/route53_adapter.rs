use crate::route53::Route53Error;
use rusoto_core::Region;
use rusoto_route53::{HostedZone, Route53, Route53Client};
use std::error::Error;

pub struct Route53Adapter<T>
where
    T: Route53,
{
    pub route53_client: T,
}

impl Route53Adapter<Route53Client> {
    pub fn new() -> Route53Adapter<Route53Client> {
        // as Route53 is global and not bound to a region we have to use the "default"
        // region, which is us-east-1 in this case.
        let client = Route53Client::new(Region::UsEast1);
        Route53Adapter {
            route53_client: client,
        }
    }

    pub fn find_hosted_zone_by_name(
        &self,
        hosted_zone_name: &str,
    ) -> Result<HostedZone, Route53Error> {
        let hosted_zones = self
            .list_hosted_zones()
            .map_err(|err| Route53Error::Unknown(err.to_string()))?;
        for zone in hosted_zones {
            debug!(
                "HZ {} starts with {}? {}",
                zone.name,
                hosted_zone_name,
                zone.name.starts_with(hosted_zone_name)
            );
            if zone.name.starts_with(hosted_zone_name) {
                return Ok(zone);
            }
        }

        Err(Route53Error::HostedZoneNotFound)
    }

    pub fn list_hosted_zones(&self) -> Result<Vec<HostedZone>, Box<dyn Error>> {
        let hosted_zones_res = self
            .route53_client
            .list_hosted_zones(Default::default())
            .sync()?;

        Ok(hosted_zones_res.hosted_zones)
    }

    // pub fn list_record_sets(
    //     &self,
    //     hosted_zone: &HostedZone,
    // ) -> Result<Vec<ResourceRecordSet>, Box<dyn Error>> {
    //     let split: Vec<&str> = hosted_zone.id.split('/').collect();
    //     let request = ListResourceRecordSetsRequest {
    //         hosted_zone_id: split[2].to_string(),
    //         max_items: None,
    //         start_record_identifier: None,
    //         start_record_name: None,
    //         start_record_type: None,
    //     };
    //     let record_sets_res = self
    //         .route53_client
    //         .list_resource_record_sets(request)
    //         .sync()?;
    //
    //     Ok(record_sets_res.resource_record_sets)
    // }
}
