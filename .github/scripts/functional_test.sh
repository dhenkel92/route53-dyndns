#! /bin/bash

set -ex

# Create test configuration
cat <<EOF >./test-config.yml
domains:
  - provider: aws
    tld: $TEST_BASE_DOMAIN
    domain: $TEST_DOMAIN
EOF

# Execute route53 dyn dns
chmod +x ./route53-dyndns
./route53-dyndns -c ./test-config.yml

# Wait for the Record to stabilize
sleep 15

# Delete recently created Resource Record Set
hosted_zone_id=$(aws route53 list-hosted-zones | jq '.HostedZones[] | select(.Name == "$TEST_BASE_DOMAIN.") | .Id')
record_set=$(aws route53 list-resource-record-sets --hosted-zone-id $hosted_zone_id | jq '.ResourceRecordSets[] | select(.Name == "$TEST_DOMAIN.")')

cat <<EOF >./delete.json
{
  "Comment": "Delete CI entry",
  "Changes": [
    {
      "Action": "DELETE",
      "ResourceRecordSet": $record_set
    }
  ]
}
EOF

aws route53 change-resource-record-sets --hosted-zone-id $hosted_zone_id --change-batch file://./delete.json
