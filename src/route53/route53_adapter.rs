use crate::route53::types::Route53Result;
use crate::route53::Route53Error;
use rusoto_core::Region;
use rusoto_route53::{
    Change, ChangeBatch, ChangeResourceRecordSetsRequest, HostedZone,
    ListResourceRecordSetsRequest, ResourceRecord, ResourceRecordSet, Route53, Route53Client,
};

pub struct Route53Adapter<T>
where
    T: Route53,
{
    route53_client: T,
}

impl Route53Adapter<Route53Client> {
    pub fn new(route53_client: Option<Route53Client>) -> Route53Adapter<Route53Client> {
        // as Route53 is global and not bound to a region we have to use the "default"
        // region, which is us-east-1 in this case.
        let client = route53_client.unwrap_or_else(|| Route53Client::new(Region::UsEast1));
        Route53Adapter {
            route53_client: client,
        }
    }

    pub async fn find_hosted_zone_by_name(
        &self,
        hosted_zone_name: &str,
    ) -> Route53Result<HostedZone> {
        let hosted_zones = self
            .list_hosted_zones()
            .await
            .map_err(|err| Route53Error::LibError(err.to_string()))?;
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

    async fn list_hosted_zones(&self) -> Route53Result<Vec<HostedZone>> {
        let hosted_zones_res = self
            .route53_client
            .list_hosted_zones(Default::default())
            .await
            .map_err(|err| Route53Error::LibError(err.to_string()))?;

        Ok(hosted_zones_res.hosted_zones)
    }

    async fn find_record_set_by_name(
        &self,
        hosted_zone_id: &str,
        domain: &str,
    ) -> Route53Result<ResourceRecordSet> {
        let records = self.list_record_sets(hosted_zone_id).await?;

        for record in records {
            if record.name.starts_with(domain) {
                return Ok(record);
            }
        }

        Err(Route53Error::RecordSetNotFound)
    }

    fn default_record_set(&self, domain: &str) -> ResourceRecordSet {
        ResourceRecordSet {
            alias_target: None,
            failover: None,
            geo_location: None,
            health_check_id: None,
            multi_value_answer: None,
            name: domain.to_string(),
            region: None,
            resource_records: None,
            set_identifier: None,
            ttl: Some(300),
            traffic_policy_instance_id: None,
            type_: "A".to_string(),
            weight: None,
        }
    }

    pub async fn upsert_record(
        &self,
        hosted_zone_id: &str,
        domain: &str,
        ip: &str,
    ) -> Route53Result<ResourceRecordSet> {
        let mut record_set = self
            .find_record_set_by_name(hosted_zone_id, domain)
            .await
            .unwrap_or_else(|_| self.default_record_set(domain));
        record_set.resource_records = Some(vec![ResourceRecord {
            value: ip.to_string(),
        }]);
        let request = ChangeResourceRecordSetsRequest {
            hosted_zone_id: hosted_zone_id.to_string(),
            change_batch: ChangeBatch {
                changes: vec![Change {
                    action: "UPSERT".to_string(),
                    resource_record_set: record_set.clone(),
                }],
                comment: None,
            },
        };

        self.route53_client
            .change_resource_record_sets(request)
            .await
            .map_err(|err| Route53Error::LibError(err.to_string()))?;
        Ok(record_set)
    }

    async fn list_record_sets(
        &self,
        hosted_zone_id: &str,
    ) -> Route53Result<Vec<ResourceRecordSet>> {
        let request = ListResourceRecordSetsRequest {
            hosted_zone_id: hosted_zone_id.to_string(),
            max_items: None,
            start_record_identifier: None,
            start_record_name: None,
            start_record_type: None,
        };
        let record_sets_res = self
            .route53_client
            .list_resource_record_sets(request)
            .await
            .map_err(|err| Route53Error::LibError(err.to_string()))?;

        Ok(record_sets_res.resource_record_sets)
    }
}
