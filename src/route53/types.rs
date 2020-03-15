use serde::export::Formatter;
use std::error::Error;

#[derive(Debug)]
pub enum Route53Error {
    HostedZoneNotFound,
    Unknown(String),
}

impl Error for Route53Error {}

impl std::fmt::Display for Route53Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}
