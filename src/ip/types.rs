use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum IPError {
    LibError(String),
}

impl Error for IPError {}

impl std::fmt::Display for IPError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message: &str;

        match self {
            IPError::LibError(orig_err) => message = orig_err,
        }

        write!(f, "{}", message)
    }
}
