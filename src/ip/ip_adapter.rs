use crate::ip::IPError;

pub async fn fetch_ip() -> Result<String, IPError> {
    reqwest::get("http://ifconfig.so")
        .await
        .map_err(|err| IPError::LibError(err.to_string()))?
        .text()
        .await
        .map_err(|err| IPError::LibError(err.to_string()))
}
