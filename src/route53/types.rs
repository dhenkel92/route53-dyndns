use serde::export::Formatter;
use std::error::Error;

pub type Route53Result<T> = Result<T, Route53Error>;

#[derive(Debug)]
pub enum Route53Error {
    HostedZoneNotFound,
    RecordSetNotFound,
    LibError(String),
}

impl Error for Route53Error {}

impl std::fmt::Display for Route53Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}
