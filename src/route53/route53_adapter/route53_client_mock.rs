use async_trait::async_trait;
use rusoto_core::RusotoError;
use rusoto_route53::{
    AssociateVPCWithHostedZoneError, AssociateVPCWithHostedZoneRequest,
    AssociateVPCWithHostedZoneResponse, ChangeResourceRecordSetsError,
    ChangeResourceRecordSetsRequest, ChangeResourceRecordSetsResponse, ChangeTagsForResourceError,
    ChangeTagsForResourceRequest, ChangeTagsForResourceResponse, CreateHealthCheckError,
    CreateHealthCheckRequest, CreateHealthCheckResponse, CreateHostedZoneError,
    CreateHostedZoneRequest, CreateHostedZoneResponse, CreateQueryLoggingConfigError,
    CreateQueryLoggingConfigRequest, CreateQueryLoggingConfigResponse,
    CreateReusableDelegationSetError, CreateReusableDelegationSetRequest,
    CreateReusableDelegationSetResponse, CreateTrafficPolicyError,
    CreateTrafficPolicyInstanceError, CreateTrafficPolicyInstanceRequest,
    CreateTrafficPolicyInstanceResponse, CreateTrafficPolicyRequest, CreateTrafficPolicyResponse,
    CreateTrafficPolicyVersionError, CreateTrafficPolicyVersionRequest,
    CreateTrafficPolicyVersionResponse, CreateVPCAssociationAuthorizationError,
    CreateVPCAssociationAuthorizationRequest, CreateVPCAssociationAuthorizationResponse,
    DeleteHealthCheckError, DeleteHealthCheckRequest, DeleteHealthCheckResponse,
    DeleteHostedZoneError, DeleteHostedZoneRequest, DeleteHostedZoneResponse,
    DeleteQueryLoggingConfigError, DeleteQueryLoggingConfigRequest,
    DeleteQueryLoggingConfigResponse, DeleteReusableDelegationSetError,
    DeleteReusableDelegationSetRequest, DeleteReusableDelegationSetResponse,
    DeleteTrafficPolicyError, DeleteTrafficPolicyInstanceError, DeleteTrafficPolicyInstanceRequest,
    DeleteTrafficPolicyInstanceResponse, DeleteTrafficPolicyRequest, DeleteTrafficPolicyResponse,
    DeleteVPCAssociationAuthorizationError, DeleteVPCAssociationAuthorizationRequest,
    DeleteVPCAssociationAuthorizationResponse, DisassociateVPCFromHostedZoneError,
    DisassociateVPCFromHostedZoneRequest, DisassociateVPCFromHostedZoneResponse,
    GetAccountLimitError, GetAccountLimitRequest, GetAccountLimitResponse, GetChangeError,
    GetChangeRequest, GetChangeResponse, GetCheckerIpRangesError, GetCheckerIpRangesRequest,
    GetCheckerIpRangesResponse, GetGeoLocationError, GetGeoLocationRequest, GetGeoLocationResponse,
    GetHealthCheckCountError, GetHealthCheckCountRequest, GetHealthCheckCountResponse,
    GetHealthCheckError, GetHealthCheckLastFailureReasonError,
    GetHealthCheckLastFailureReasonRequest, GetHealthCheckLastFailureReasonResponse,
    GetHealthCheckRequest, GetHealthCheckResponse, GetHealthCheckStatusError,
    GetHealthCheckStatusRequest, GetHealthCheckStatusResponse, GetHostedZoneCountError,
    GetHostedZoneCountRequest, GetHostedZoneCountResponse, GetHostedZoneError,
    GetHostedZoneLimitError, GetHostedZoneLimitRequest, GetHostedZoneLimitResponse,
    GetHostedZoneRequest, GetHostedZoneResponse, GetQueryLoggingConfigError,
    GetQueryLoggingConfigRequest, GetQueryLoggingConfigResponse, GetReusableDelegationSetError,
    GetReusableDelegationSetLimitError, GetReusableDelegationSetLimitRequest,
    GetReusableDelegationSetLimitResponse, GetReusableDelegationSetRequest,
    GetReusableDelegationSetResponse, GetTrafficPolicyError, GetTrafficPolicyInstanceCountError,
    GetTrafficPolicyInstanceCountRequest, GetTrafficPolicyInstanceCountResponse,
    GetTrafficPolicyInstanceError, GetTrafficPolicyInstanceRequest,
    GetTrafficPolicyInstanceResponse, GetTrafficPolicyRequest, GetTrafficPolicyResponse,
    HostedZone, ListGeoLocationsError, ListGeoLocationsRequest, ListGeoLocationsResponse,
    ListHealthChecksError, ListHealthChecksRequest, ListHealthChecksResponse,
    ListHostedZonesByNameError, ListHostedZonesByNameRequest, ListHostedZonesByNameResponse,
    ListHostedZonesError, ListHostedZonesRequest, ListHostedZonesResponse,
    ListQueryLoggingConfigsError, ListQueryLoggingConfigsRequest, ListQueryLoggingConfigsResponse,
    ListResourceRecordSetsError, ListResourceRecordSetsRequest, ListResourceRecordSetsResponse,
    ListReusableDelegationSetsError, ListReusableDelegationSetsRequest,
    ListReusableDelegationSetsResponse, ListTagsForResourceError, ListTagsForResourceRequest,
    ListTagsForResourceResponse, ListTagsForResourcesError, ListTagsForResourcesRequest,
    ListTagsForResourcesResponse, ListTrafficPoliciesError, ListTrafficPoliciesRequest,
    ListTrafficPoliciesResponse, ListTrafficPolicyInstancesByHostedZoneError,
    ListTrafficPolicyInstancesByHostedZoneRequest, ListTrafficPolicyInstancesByHostedZoneResponse,
    ListTrafficPolicyInstancesByPolicyError, ListTrafficPolicyInstancesByPolicyRequest,
    ListTrafficPolicyInstancesByPolicyResponse, ListTrafficPolicyInstancesError,
    ListTrafficPolicyInstancesRequest, ListTrafficPolicyInstancesResponse,
    ListTrafficPolicyVersionsError, ListTrafficPolicyVersionsRequest,
    ListTrafficPolicyVersionsResponse, ListVPCAssociationAuthorizationsError,
    ListVPCAssociationAuthorizationsRequest, ListVPCAssociationAuthorizationsResponse,
    ResourceRecordSet, Route53, TestDNSAnswerError, TestDNSAnswerRequest, TestDNSAnswerResponse,
    UpdateHealthCheckError, UpdateHealthCheckRequest, UpdateHealthCheckResponse,
    UpdateHostedZoneCommentError, UpdateHostedZoneCommentRequest, UpdateHostedZoneCommentResponse,
    UpdateTrafficPolicyCommentError, UpdateTrafficPolicyCommentRequest,
    UpdateTrafficPolicyCommentResponse, UpdateTrafficPolicyInstanceError,
    UpdateTrafficPolicyInstanceRequest, UpdateTrafficPolicyInstanceResponse,
};

pub struct MockRoute53Client {}

#[async_trait]
impl Route53 for MockRoute53Client {
    async fn associate_vpc_with_hosted_zone(
        &self,
        __input: AssociateVPCWithHostedZoneRequest,
    ) -> Result<AssociateVPCWithHostedZoneResponse, RusotoError<AssociateVPCWithHostedZoneError>>
    {
        unimplemented!()
    }

    async fn change_resource_record_sets(
        &self,
        __input: ChangeResourceRecordSetsRequest,
    ) -> Result<ChangeResourceRecordSetsResponse, RusotoError<ChangeResourceRecordSetsError>> {
        unimplemented!()
    }

    async fn change_tags_for_resource(
        &self,
        __input: ChangeTagsForResourceRequest,
    ) -> Result<ChangeTagsForResourceResponse, RusotoError<ChangeTagsForResourceError>> {
        unimplemented!()
    }

    async fn create_health_check(
        &self,
        __input: CreateHealthCheckRequest,
    ) -> Result<CreateHealthCheckResponse, RusotoError<CreateHealthCheckError>> {
        unimplemented!()
    }

    async fn create_hosted_zone(
        &self,
        __input: CreateHostedZoneRequest,
    ) -> Result<CreateHostedZoneResponse, RusotoError<CreateHostedZoneError>> {
        unimplemented!()
    }

    async fn create_query_logging_config(
        &self,
        __input: CreateQueryLoggingConfigRequest,
    ) -> Result<CreateQueryLoggingConfigResponse, RusotoError<CreateQueryLoggingConfigError>> {
        unimplemented!()
    }

    async fn create_reusable_delegation_set(
        &self,
        __input: CreateReusableDelegationSetRequest,
    ) -> Result<CreateReusableDelegationSetResponse, RusotoError<CreateReusableDelegationSetError>>
    {
        unimplemented!()
    }

    async fn create_traffic_policy(
        &self,
        __input: CreateTrafficPolicyRequest,
    ) -> Result<CreateTrafficPolicyResponse, RusotoError<CreateTrafficPolicyError>> {
        unimplemented!()
    }

    async fn create_traffic_policy_instance(
        &self,
        __input: CreateTrafficPolicyInstanceRequest,
    ) -> Result<CreateTrafficPolicyInstanceResponse, RusotoError<CreateTrafficPolicyInstanceError>>
    {
        unimplemented!()
    }

    async fn create_traffic_policy_version(
        &self,
        __input: CreateTrafficPolicyVersionRequest,
    ) -> Result<CreateTrafficPolicyVersionResponse, RusotoError<CreateTrafficPolicyVersionError>>
    {
        unimplemented!()
    }

    async fn create_vpc_association_authorization(
        &self,
        _input: CreateVPCAssociationAuthorizationRequest,
    ) -> Result<
        CreateVPCAssociationAuthorizationResponse,
        RusotoError<CreateVPCAssociationAuthorizationError>,
    > {
        unimplemented!()
    }

    async fn delete_health_check(
        &self,
        _input: DeleteHealthCheckRequest,
    ) -> Result<DeleteHealthCheckResponse, RusotoError<DeleteHealthCheckError>> {
        unimplemented!()
    }

    async fn delete_hosted_zone(
        &self,
        _input: DeleteHostedZoneRequest,
    ) -> Result<DeleteHostedZoneResponse, RusotoError<DeleteHostedZoneError>> {
        unimplemented!()
    }

    async fn delete_query_logging_config(
        &self,
        _input: DeleteQueryLoggingConfigRequest,
    ) -> Result<DeleteQueryLoggingConfigResponse, RusotoError<DeleteQueryLoggingConfigError>> {
        unimplemented!()
    }

    async fn delete_reusable_delegation_set(
        &self,
        _input: DeleteReusableDelegationSetRequest,
    ) -> Result<DeleteReusableDelegationSetResponse, RusotoError<DeleteReusableDelegationSetError>>
    {
        unimplemented!()
    }

    async fn delete_traffic_policy(
        &self,
        _input: DeleteTrafficPolicyRequest,
    ) -> Result<DeleteTrafficPolicyResponse, RusotoError<DeleteTrafficPolicyError>> {
        unimplemented!()
    }

    async fn delete_traffic_policy_instance(
        &self,
        _input: DeleteTrafficPolicyInstanceRequest,
    ) -> Result<DeleteTrafficPolicyInstanceResponse, RusotoError<DeleteTrafficPolicyInstanceError>>
    {
        unimplemented!()
    }

    async fn delete_vpc_association_authorization(
        &self,
        _input: DeleteVPCAssociationAuthorizationRequest,
    ) -> Result<
        DeleteVPCAssociationAuthorizationResponse,
        RusotoError<DeleteVPCAssociationAuthorizationError>,
    > {
        unimplemented!()
    }

    async fn disassociate_vpc_from_hosted_zone(
        &self,
        _input: DisassociateVPCFromHostedZoneRequest,
    ) -> Result<
        DisassociateVPCFromHostedZoneResponse,
        RusotoError<DisassociateVPCFromHostedZoneError>,
    > {
        unimplemented!()
    }

    async fn get_account_limit(
        &self,
        _input: GetAccountLimitRequest,
    ) -> Result<GetAccountLimitResponse, RusotoError<GetAccountLimitError>> {
        unimplemented!()
    }

    async fn get_change(
        &self,
        _input: GetChangeRequest,
    ) -> Result<GetChangeResponse, RusotoError<GetChangeError>> {
        unimplemented!()
    }

    async fn get_checker_ip_ranges(
        &self,
        _input: GetCheckerIpRangesRequest,
    ) -> Result<GetCheckerIpRangesResponse, RusotoError<GetCheckerIpRangesError>> {
        unimplemented!()
    }

    async fn get_geo_location(
        &self,
        _input: GetGeoLocationRequest,
    ) -> Result<GetGeoLocationResponse, RusotoError<GetGeoLocationError>> {
        unimplemented!()
    }

    async fn get_health_check(
        &self,
        _input: GetHealthCheckRequest,
    ) -> Result<GetHealthCheckResponse, RusotoError<GetHealthCheckError>> {
        unimplemented!()
    }

    async fn get_health_check_count(
        &self,
        _input: GetHealthCheckCountRequest,
    ) -> Result<GetHealthCheckCountResponse, RusotoError<GetHealthCheckCountError>> {
        unimplemented!()
    }

    async fn get_health_check_last_failure_reason(
        &self,
        _input: GetHealthCheckLastFailureReasonRequest,
    ) -> Result<
        GetHealthCheckLastFailureReasonResponse,
        RusotoError<GetHealthCheckLastFailureReasonError>,
    > {
        unimplemented!()
    }

    async fn get_health_check_status(
        &self,
        _input: GetHealthCheckStatusRequest,
    ) -> Result<GetHealthCheckStatusResponse, RusotoError<GetHealthCheckStatusError>> {
        unimplemented!()
    }

    async fn get_hosted_zone(
        &self,
        _input: GetHostedZoneRequest,
    ) -> Result<GetHostedZoneResponse, RusotoError<GetHostedZoneError>> {
        unimplemented!()
    }

    async fn get_hosted_zone_count(
        &self,
        _input: GetHostedZoneCountRequest,
    ) -> Result<GetHostedZoneCountResponse, RusotoError<GetHostedZoneCountError>> {
        unimplemented!()
    }

    async fn get_hosted_zone_limit(
        &self,
        _input: GetHostedZoneLimitRequest,
    ) -> Result<GetHostedZoneLimitResponse, RusotoError<GetHostedZoneLimitError>> {
        unimplemented!()
    }

    async fn get_query_logging_config(
        &self,
        _input: GetQueryLoggingConfigRequest,
    ) -> Result<GetQueryLoggingConfigResponse, RusotoError<GetQueryLoggingConfigError>> {
        unimplemented!()
    }

    async fn get_reusable_delegation_set(
        &self,
        _input: GetReusableDelegationSetRequest,
    ) -> Result<GetReusableDelegationSetResponse, RusotoError<GetReusableDelegationSetError>> {
        unimplemented!()
    }

    async fn get_reusable_delegation_set_limit(
        &self,
        _input: GetReusableDelegationSetLimitRequest,
    ) -> Result<
        GetReusableDelegationSetLimitResponse,
        RusotoError<GetReusableDelegationSetLimitError>,
    > {
        unimplemented!()
    }

    async fn get_traffic_policy(
        &self,
        _input: GetTrafficPolicyRequest,
    ) -> Result<GetTrafficPolicyResponse, RusotoError<GetTrafficPolicyError>> {
        unimplemented!()
    }

    async fn get_traffic_policy_instance(
        &self,
        _input: GetTrafficPolicyInstanceRequest,
    ) -> Result<GetTrafficPolicyInstanceResponse, RusotoError<GetTrafficPolicyInstanceError>> {
        unimplemented!()
    }

    async fn get_traffic_policy_instance_count(
        &self,
        _input: GetTrafficPolicyInstanceCountRequest,
    ) -> Result<
        GetTrafficPolicyInstanceCountResponse,
        RusotoError<GetTrafficPolicyInstanceCountError>,
    > {
        unimplemented!()
    }

    async fn list_geo_locations(
        &self,
        _input: ListGeoLocationsRequest,
    ) -> Result<ListGeoLocationsResponse, RusotoError<ListGeoLocationsError>> {
        unimplemented!()
    }

    async fn list_health_checks(
        &self,
        _input: ListHealthChecksRequest,
    ) -> Result<ListHealthChecksResponse, RusotoError<ListHealthChecksError>> {
        unimplemented!()
    }

    async fn list_hosted_zones(
        &self,
        _input: ListHostedZonesRequest,
    ) -> Result<ListHostedZonesResponse, RusotoError<ListHostedZonesError>> {
        let mut response = ListHostedZonesResponse::default();
        response.hosted_zones = vec![HostedZone {
            caller_reference: "".to_string(),
            config: None,
            id: "foo_id".to_string(),
            linked_service: None,
            name: "foo".to_string(),
            resource_record_set_count: None,
        }];
        Ok(response)
    }

    async fn list_hosted_zones_by_name(
        &self,
        _input: ListHostedZonesByNameRequest,
    ) -> Result<ListHostedZonesByNameResponse, RusotoError<ListHostedZonesByNameError>> {
        unimplemented!()
    }

    async fn list_query_logging_configs(
        &self,
        _input: ListQueryLoggingConfigsRequest,
    ) -> Result<ListQueryLoggingConfigsResponse, RusotoError<ListQueryLoggingConfigsError>> {
        unimplemented!()
    }

    async fn list_resource_record_sets(
        &self,
        input: ListResourceRecordSetsRequest,
    ) -> Result<ListResourceRecordSetsResponse, RusotoError<ListResourceRecordSetsError>> {
        let mut response = ListResourceRecordSetsResponse::default();

        if input.hosted_zone_id == "valid" {
            response.resource_record_sets = vec![ResourceRecordSet {
                alias_target: None,
                failover: None,
                geo_location: None,
                health_check_id: None,
                multi_value_answer: None,
                name: "foo.bar.io".to_string(),
                region: None,
                resource_records: None,
                set_identifier: None,
                ttl: None,
                traffic_policy_instance_id: None,
                type_: "A".to_string(),
                weight: None,
            }];
        }

        Ok(response)
    }

    async fn list_reusable_delegation_sets(
        &self,
        _input: ListReusableDelegationSetsRequest,
    ) -> Result<ListReusableDelegationSetsResponse, RusotoError<ListReusableDelegationSetsError>>
    {
        unimplemented!()
    }

    async fn list_tags_for_resource(
        &self,
        _input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        unimplemented!()
    }

    async fn list_tags_for_resources(
        &self,
        _input: ListTagsForResourcesRequest,
    ) -> Result<ListTagsForResourcesResponse, RusotoError<ListTagsForResourcesError>> {
        unimplemented!()
    }

    async fn list_traffic_policies(
        &self,
        _input: ListTrafficPoliciesRequest,
    ) -> Result<ListTrafficPoliciesResponse, RusotoError<ListTrafficPoliciesError>> {
        unimplemented!()
    }

    async fn list_traffic_policy_instances(
        &self,
        _input: ListTrafficPolicyInstancesRequest,
    ) -> Result<ListTrafficPolicyInstancesResponse, RusotoError<ListTrafficPolicyInstancesError>>
    {
        unimplemented!()
    }

    async fn list_traffic_policy_instances_by_hosted_zone(
        &self,
        _input: ListTrafficPolicyInstancesByHostedZoneRequest,
    ) -> Result<
        ListTrafficPolicyInstancesByHostedZoneResponse,
        RusotoError<ListTrafficPolicyInstancesByHostedZoneError>,
    > {
        unimplemented!()
    }

    async fn list_traffic_policy_instances_by_policy(
        &self,
        _input: ListTrafficPolicyInstancesByPolicyRequest,
    ) -> Result<
        ListTrafficPolicyInstancesByPolicyResponse,
        RusotoError<ListTrafficPolicyInstancesByPolicyError>,
    > {
        unimplemented!()
    }

    async fn list_traffic_policy_versions(
        &self,
        _input: ListTrafficPolicyVersionsRequest,
    ) -> Result<ListTrafficPolicyVersionsResponse, RusotoError<ListTrafficPolicyVersionsError>>
    {
        unimplemented!()
    }

    async fn list_vpc_association_authorizations(
        &self,
        _input: ListVPCAssociationAuthorizationsRequest,
    ) -> Result<
        ListVPCAssociationAuthorizationsResponse,
        RusotoError<ListVPCAssociationAuthorizationsError>,
    > {
        unimplemented!()
    }

    async fn test_dns_answer(
        &self,
        _input: TestDNSAnswerRequest,
    ) -> Result<TestDNSAnswerResponse, RusotoError<TestDNSAnswerError>> {
        unimplemented!()
    }

    async fn update_health_check(
        &self,
        _input: UpdateHealthCheckRequest,
    ) -> Result<UpdateHealthCheckResponse, RusotoError<UpdateHealthCheckError>> {
        unimplemented!()
    }

    async fn update_hosted_zone_comment(
        &self,
        _input: UpdateHostedZoneCommentRequest,
    ) -> Result<UpdateHostedZoneCommentResponse, RusotoError<UpdateHostedZoneCommentError>> {
        unimplemented!()
    }

    async fn update_traffic_policy_comment(
        &self,
        _input: UpdateTrafficPolicyCommentRequest,
    ) -> Result<UpdateTrafficPolicyCommentResponse, RusotoError<UpdateTrafficPolicyCommentError>>
    {
        unimplemented!()
    }

    async fn update_traffic_policy_instance(
        &self,
        _input: UpdateTrafficPolicyInstanceRequest,
    ) -> Result<UpdateTrafficPolicyInstanceResponse, RusotoError<UpdateTrafficPolicyInstanceError>>
    {
        unimplemented!()
    }
}

pub struct MockFailingRoute53Client {}

#[async_trait]
impl Route53 for MockFailingRoute53Client {
    async fn associate_vpc_with_hosted_zone(
        &self,
        __input: AssociateVPCWithHostedZoneRequest,
    ) -> Result<AssociateVPCWithHostedZoneResponse, RusotoError<AssociateVPCWithHostedZoneError>>
    {
        unimplemented!()
    }

    async fn change_resource_record_sets(
        &self,
        __input: ChangeResourceRecordSetsRequest,
    ) -> Result<ChangeResourceRecordSetsResponse, RusotoError<ChangeResourceRecordSetsError>> {
        unimplemented!()
    }

    async fn change_tags_for_resource(
        &self,
        __input: ChangeTagsForResourceRequest,
    ) -> Result<ChangeTagsForResourceResponse, RusotoError<ChangeTagsForResourceError>> {
        unimplemented!()
    }

    async fn create_health_check(
        &self,
        __input: CreateHealthCheckRequest,
    ) -> Result<CreateHealthCheckResponse, RusotoError<CreateHealthCheckError>> {
        unimplemented!()
    }

    async fn create_hosted_zone(
        &self,
        __input: CreateHostedZoneRequest,
    ) -> Result<CreateHostedZoneResponse, RusotoError<CreateHostedZoneError>> {
        unimplemented!()
    }

    async fn create_query_logging_config(
        &self,
        __input: CreateQueryLoggingConfigRequest,
    ) -> Result<CreateQueryLoggingConfigResponse, RusotoError<CreateQueryLoggingConfigError>> {
        unimplemented!()
    }

    async fn create_reusable_delegation_set(
        &self,
        __input: CreateReusableDelegationSetRequest,
    ) -> Result<CreateReusableDelegationSetResponse, RusotoError<CreateReusableDelegationSetError>>
    {
        unimplemented!()
    }

    async fn create_traffic_policy(
        &self,
        __input: CreateTrafficPolicyRequest,
    ) -> Result<CreateTrafficPolicyResponse, RusotoError<CreateTrafficPolicyError>> {
        unimplemented!()
    }

    async fn create_traffic_policy_instance(
        &self,
        __input: CreateTrafficPolicyInstanceRequest,
    ) -> Result<CreateTrafficPolicyInstanceResponse, RusotoError<CreateTrafficPolicyInstanceError>>
    {
        unimplemented!()
    }

    async fn create_traffic_policy_version(
        &self,
        __input: CreateTrafficPolicyVersionRequest,
    ) -> Result<CreateTrafficPolicyVersionResponse, RusotoError<CreateTrafficPolicyVersionError>>
    {
        unimplemented!()
    }

    async fn create_vpc_association_authorization(
        &self,
        _input: CreateVPCAssociationAuthorizationRequest,
    ) -> Result<
        CreateVPCAssociationAuthorizationResponse,
        RusotoError<CreateVPCAssociationAuthorizationError>,
    > {
        unimplemented!()
    }

    async fn delete_health_check(
        &self,
        _input: DeleteHealthCheckRequest,
    ) -> Result<DeleteHealthCheckResponse, RusotoError<DeleteHealthCheckError>> {
        unimplemented!()
    }

    async fn delete_hosted_zone(
        &self,
        _input: DeleteHostedZoneRequest,
    ) -> Result<DeleteHostedZoneResponse, RusotoError<DeleteHostedZoneError>> {
        unimplemented!()
    }

    async fn delete_query_logging_config(
        &self,
        _input: DeleteQueryLoggingConfigRequest,
    ) -> Result<DeleteQueryLoggingConfigResponse, RusotoError<DeleteQueryLoggingConfigError>> {
        unimplemented!()
    }

    async fn delete_reusable_delegation_set(
        &self,
        _input: DeleteReusableDelegationSetRequest,
    ) -> Result<DeleteReusableDelegationSetResponse, RusotoError<DeleteReusableDelegationSetError>>
    {
        unimplemented!()
    }

    async fn delete_traffic_policy(
        &self,
        _input: DeleteTrafficPolicyRequest,
    ) -> Result<DeleteTrafficPolicyResponse, RusotoError<DeleteTrafficPolicyError>> {
        unimplemented!()
    }

    async fn delete_traffic_policy_instance(
        &self,
        _input: DeleteTrafficPolicyInstanceRequest,
    ) -> Result<DeleteTrafficPolicyInstanceResponse, RusotoError<DeleteTrafficPolicyInstanceError>>
    {
        unimplemented!()
    }

    async fn delete_vpc_association_authorization(
        &self,
        _input: DeleteVPCAssociationAuthorizationRequest,
    ) -> Result<
        DeleteVPCAssociationAuthorizationResponse,
        RusotoError<DeleteVPCAssociationAuthorizationError>,
    > {
        unimplemented!()
    }

    async fn disassociate_vpc_from_hosted_zone(
        &self,
        _input: DisassociateVPCFromHostedZoneRequest,
    ) -> Result<
        DisassociateVPCFromHostedZoneResponse,
        RusotoError<DisassociateVPCFromHostedZoneError>,
    > {
        unimplemented!()
    }

    async fn get_account_limit(
        &self,
        _input: GetAccountLimitRequest,
    ) -> Result<GetAccountLimitResponse, RusotoError<GetAccountLimitError>> {
        unimplemented!()
    }

    async fn get_change(
        &self,
        _input: GetChangeRequest,
    ) -> Result<GetChangeResponse, RusotoError<GetChangeError>> {
        unimplemented!()
    }

    async fn get_checker_ip_ranges(
        &self,
        _input: GetCheckerIpRangesRequest,
    ) -> Result<GetCheckerIpRangesResponse, RusotoError<GetCheckerIpRangesError>> {
        unimplemented!()
    }

    async fn get_geo_location(
        &self,
        _input: GetGeoLocationRequest,
    ) -> Result<GetGeoLocationResponse, RusotoError<GetGeoLocationError>> {
        unimplemented!()
    }

    async fn get_health_check(
        &self,
        _input: GetHealthCheckRequest,
    ) -> Result<GetHealthCheckResponse, RusotoError<GetHealthCheckError>> {
        unimplemented!()
    }

    async fn get_health_check_count(
        &self,
        _input: GetHealthCheckCountRequest,
    ) -> Result<GetHealthCheckCountResponse, RusotoError<GetHealthCheckCountError>> {
        unimplemented!()
    }

    async fn get_health_check_last_failure_reason(
        &self,
        _input: GetHealthCheckLastFailureReasonRequest,
    ) -> Result<
        GetHealthCheckLastFailureReasonResponse,
        RusotoError<GetHealthCheckLastFailureReasonError>,
    > {
        unimplemented!()
    }

    async fn get_health_check_status(
        &self,
        _input: GetHealthCheckStatusRequest,
    ) -> Result<GetHealthCheckStatusResponse, RusotoError<GetHealthCheckStatusError>> {
        unimplemented!()
    }

    async fn get_hosted_zone(
        &self,
        _input: GetHostedZoneRequest,
    ) -> Result<GetHostedZoneResponse, RusotoError<GetHostedZoneError>> {
        unimplemented!()
    }

    async fn get_hosted_zone_count(
        &self,
        _input: GetHostedZoneCountRequest,
    ) -> Result<GetHostedZoneCountResponse, RusotoError<GetHostedZoneCountError>> {
        unimplemented!()
    }

    async fn get_hosted_zone_limit(
        &self,
        _input: GetHostedZoneLimitRequest,
    ) -> Result<GetHostedZoneLimitResponse, RusotoError<GetHostedZoneLimitError>> {
        unimplemented!()
    }

    async fn get_query_logging_config(
        &self,
        _input: GetQueryLoggingConfigRequest,
    ) -> Result<GetQueryLoggingConfigResponse, RusotoError<GetQueryLoggingConfigError>> {
        unimplemented!()
    }

    async fn get_reusable_delegation_set(
        &self,
        _input: GetReusableDelegationSetRequest,
    ) -> Result<GetReusableDelegationSetResponse, RusotoError<GetReusableDelegationSetError>> {
        unimplemented!()
    }

    async fn get_reusable_delegation_set_limit(
        &self,
        _input: GetReusableDelegationSetLimitRequest,
    ) -> Result<
        GetReusableDelegationSetLimitResponse,
        RusotoError<GetReusableDelegationSetLimitError>,
    > {
        unimplemented!()
    }

    async fn get_traffic_policy(
        &self,
        _input: GetTrafficPolicyRequest,
    ) -> Result<GetTrafficPolicyResponse, RusotoError<GetTrafficPolicyError>> {
        unimplemented!()
    }

    async fn get_traffic_policy_instance(
        &self,
        _input: GetTrafficPolicyInstanceRequest,
    ) -> Result<GetTrafficPolicyInstanceResponse, RusotoError<GetTrafficPolicyInstanceError>> {
        unimplemented!()
    }

    async fn get_traffic_policy_instance_count(
        &self,
        _input: GetTrafficPolicyInstanceCountRequest,
    ) -> Result<
        GetTrafficPolicyInstanceCountResponse,
        RusotoError<GetTrafficPolicyInstanceCountError>,
    > {
        unimplemented!()
    }

    async fn list_geo_locations(
        &self,
        _input: ListGeoLocationsRequest,
    ) -> Result<ListGeoLocationsResponse, RusotoError<ListGeoLocationsError>> {
        unimplemented!()
    }

    async fn list_health_checks(
        &self,
        _input: ListHealthChecksRequest,
    ) -> Result<ListHealthChecksResponse, RusotoError<ListHealthChecksError>> {
        unimplemented!()
    }

    async fn list_hosted_zones(
        &self,
        _input: ListHostedZonesRequest,
    ) -> Result<ListHostedZonesResponse, RusotoError<ListHostedZonesError>> {
        Err(RusotoError::<ListHostedZonesError>::Blocking)
    }

    async fn list_hosted_zones_by_name(
        &self,
        _input: ListHostedZonesByNameRequest,
    ) -> Result<ListHostedZonesByNameResponse, RusotoError<ListHostedZonesByNameError>> {
        unimplemented!()
    }

    async fn list_query_logging_configs(
        &self,
        _input: ListQueryLoggingConfigsRequest,
    ) -> Result<ListQueryLoggingConfigsResponse, RusotoError<ListQueryLoggingConfigsError>> {
        unimplemented!()
    }

    async fn list_resource_record_sets(
        &self,
        _input: ListResourceRecordSetsRequest,
    ) -> Result<ListResourceRecordSetsResponse, RusotoError<ListResourceRecordSetsError>> {
        Err(RusotoError::<ListResourceRecordSetsError>::Blocking)
    }

    async fn list_reusable_delegation_sets(
        &self,
        _input: ListReusableDelegationSetsRequest,
    ) -> Result<ListReusableDelegationSetsResponse, RusotoError<ListReusableDelegationSetsError>>
    {
        unimplemented!()
    }

    async fn list_tags_for_resource(
        &self,
        _input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        unimplemented!()
    }

    async fn list_tags_for_resources(
        &self,
        _input: ListTagsForResourcesRequest,
    ) -> Result<ListTagsForResourcesResponse, RusotoError<ListTagsForResourcesError>> {
        unimplemented!()
    }

    async fn list_traffic_policies(
        &self,
        _input: ListTrafficPoliciesRequest,
    ) -> Result<ListTrafficPoliciesResponse, RusotoError<ListTrafficPoliciesError>> {
        unimplemented!()
    }

    async fn list_traffic_policy_instances(
        &self,
        _input: ListTrafficPolicyInstancesRequest,
    ) -> Result<ListTrafficPolicyInstancesResponse, RusotoError<ListTrafficPolicyInstancesError>>
    {
        unimplemented!()
    }

    async fn list_traffic_policy_instances_by_hosted_zone(
        &self,
        _input: ListTrafficPolicyInstancesByHostedZoneRequest,
    ) -> Result<
        ListTrafficPolicyInstancesByHostedZoneResponse,
        RusotoError<ListTrafficPolicyInstancesByHostedZoneError>,
    > {
        unimplemented!()
    }

    async fn list_traffic_policy_instances_by_policy(
        &self,
        _input: ListTrafficPolicyInstancesByPolicyRequest,
    ) -> Result<
        ListTrafficPolicyInstancesByPolicyResponse,
        RusotoError<ListTrafficPolicyInstancesByPolicyError>,
    > {
        unimplemented!()
    }

    async fn list_traffic_policy_versions(
        &self,
        _input: ListTrafficPolicyVersionsRequest,
    ) -> Result<ListTrafficPolicyVersionsResponse, RusotoError<ListTrafficPolicyVersionsError>>
    {
        unimplemented!()
    }

    async fn list_vpc_association_authorizations(
        &self,
        _input: ListVPCAssociationAuthorizationsRequest,
    ) -> Result<
        ListVPCAssociationAuthorizationsResponse,
        RusotoError<ListVPCAssociationAuthorizationsError>,
    > {
        unimplemented!()
    }

    async fn test_dns_answer(
        &self,
        _input: TestDNSAnswerRequest,
    ) -> Result<TestDNSAnswerResponse, RusotoError<TestDNSAnswerError>> {
        unimplemented!()
    }

    async fn update_health_check(
        &self,
        _input: UpdateHealthCheckRequest,
    ) -> Result<UpdateHealthCheckResponse, RusotoError<UpdateHealthCheckError>> {
        unimplemented!()
    }

    async fn update_hosted_zone_comment(
        &self,
        _input: UpdateHostedZoneCommentRequest,
    ) -> Result<UpdateHostedZoneCommentResponse, RusotoError<UpdateHostedZoneCommentError>> {
        unimplemented!()
    }

    async fn update_traffic_policy_comment(
        &self,
        _input: UpdateTrafficPolicyCommentRequest,
    ) -> Result<UpdateTrafficPolicyCommentResponse, RusotoError<UpdateTrafficPolicyCommentError>>
    {
        unimplemented!()
    }

    async fn update_traffic_policy_instance(
        &self,
        _input: UpdateTrafficPolicyInstanceRequest,
    ) -> Result<UpdateTrafficPolicyInstanceResponse, RusotoError<UpdateTrafficPolicyInstanceError>>
    {
        unimplemented!()
    }
}
