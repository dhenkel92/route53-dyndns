use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum IPError {
    LibError(String)
}

impl Error for IPError {}

impl std::fmt::Display for IPError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // todo: properly format error
        write!(f, "invalid first item to double")
    }
}
