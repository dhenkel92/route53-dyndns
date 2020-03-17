FROM debian:buster-slim

WORKDIR /root

RUN apt-get update \
    && apt-get install -y openssl

COPY ./target/release/route53-dyndns ./route53-dyndns

ENTRYPOINT ["/root/route53-dyndns"]
