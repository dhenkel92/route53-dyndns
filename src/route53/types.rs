use serde::export::Formatter;
use std::error::Error;

pub type Route53Result<T> = Result<T, Route53Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Route53Error {
    HostedZoneNotFound,
    RecordSetNotFound,
    LibError(String),
}

impl Error for Route53Error {}

impl std::fmt::Display for Route53Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message: &str;

        match self {
            Route53Error::LibError(orig_err) => message = orig_err,
            Route53Error::RecordSetNotFound => message = "Record set was not found!",
            Route53Error::HostedZoneNotFound => message = "Hosted zone was not found!",
        }

        write!(f, "{}", message)
    }
}
