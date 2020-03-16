use crate::ip::IPError;

pub fn fetch_ip() -> Result<String, IPError> {
    reqwest::blocking::get("http://ifconfig.so")
        .map_err(|err| IPError::LibError(err.to_string()))?
        .text()
        .map_err(|err| IPError::LibError(err.to_string()))
}
