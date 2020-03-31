use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq)]
pub enum HandlerError {
    RecordNotFound,
    ProviderError(String),
}

impl Error for HandlerError {}

impl std::fmt::Display for HandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message: &str;

        match self {
            HandlerError::RecordNotFound => message = "Record was not found!",
            HandlerError::ProviderError(orig_err) => message = orig_err,
        }

        write!(f, "{}", message)
    }
}

pub type HandlerResult = Result<(), HandlerError>;
