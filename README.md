# Route53 DynDNS

## Montivation

Like many of us, I also have a Raspberry PI at home, hosting some home automation stuff, and sometimes you need to get access from the outside.
You'll probably want to have a DNS name pointing to your router at home as you don't want to remember daily changing IP addresses.

Since I was looking for a project for learning Rust, I've decided to build the tool myself instead of using a cronjob executing a small Terraform script.

## Configuration

The application takes a YAML configuration file of the following format

```yaml
domains:
  - provider: aws
    tld: henkel.tech
    domain: dyndns.henkel.tech
```

As you can see, it's also possible to update multiple entries from multiple providers.
Right now, the only supported provider is `aws`, but there might be more in the future.

## Permissions

Here you can find the required permissions for the different providers.

### AWS Route53

We basically need permissions to list hosted zones and resource record sets in order to discover the records we need to update.
Furthermore, we also need permissions to update record itself.

The following JSON contains a simple IAM policy to grant access to the discussed actions.
You could also narrow it down further by defining which hosted zones it's allowed to list / update the records.

```json
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "",
            "Effect": "Allow",
            "Action": [
                "route53:ListHostedZones",
                "route53:ChangeResourceRecordSets",
                "route53:ListResourceRecordSets"
            ],
            "Resource": "*"
        }
    ]
}
```

