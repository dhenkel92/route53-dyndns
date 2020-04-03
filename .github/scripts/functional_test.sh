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
until [ "$(dig +short $TEST_DOMAIN)" != "" ]
do
  sleep 5
done

# Delete recently created Resource Record Set
hosted_zone_id=$(aws route53 list-hosted-zones | jq -r --arg TEST_BASE_DOMAIN "$TEST_BASE_DOMAIN" '.HostedZones[] | select(.Name == $TEST_BASE_DOMAIN) | .Id')
record_set=$(aws route53 list-resource-record-sets --hosted-zone-id $hosted_zone_id | jq --arg TEST_DOMAIN "$TEST_DOMAIN" '.ResourceRecordSets[] | select(.Name == $TEST_DOMAIN)')

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

# Test if IP was properly set
aws_ip=$(echo "$record_set" | jq -r '.ResourceRecords[0].Value')
internal_ip=$(curl -s ifconfig.so)

if [ "$aws_ip" != "$internal_ip" ]; then
  exit 1
fi

exit 0
