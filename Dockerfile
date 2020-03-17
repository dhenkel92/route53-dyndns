FROM debian:buster-slim

WORKDIR /root

COPY ./target/release/route53-dyndns ./route53-dyndns

ENTRYPOINT ["/root/route53-dyndns"]
