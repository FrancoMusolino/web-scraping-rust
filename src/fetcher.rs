use async_trait::async_trait;
use reqwest::Response as ReqwestResponse;
use std::error::Error;

#[async_trait]
pub trait ResponseMethods {
    async fn parse_to_text(self) -> Result<String, Box<dyn Error>>;
}

#[async_trait]
impl ResponseMethods for ReqwestResponse {
    async fn parse_to_text(self) -> Result<String, Box<dyn Error>> {
        let bytes = self.bytes().await?;
        let body = String::from_utf8_lossy(&bytes).into_owned();

        Ok(body)
    }
}

#[async_trait]
pub trait Methods {
    type Response: ResponseMethods;
    async fn get(hostname: &str) -> Result<Self::Response, Box<dyn Error>>;
}

pub struct Fetcher {}

#[async_trait]
impl Methods for Fetcher {
    type Response = ReqwestResponse;

    async fn get(hostname: &str) -> Result<Self::Response, Box<dyn Error>> {
        let response = reqwest::get(hostname).await?;

        Ok(response)
    }
}
