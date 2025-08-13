use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;

use crate::{Result, RigError};

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
    base_url: Url,
    #[allow(dead_code)]
    timeout: Duration,
    #[allow(dead_code)]
    retry_attempts: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_at: Option<String>,
}

impl HttpClient {
    pub fn new(base_url: &str, timeout: Duration, retry_attempts: u32) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent(format!("rig-cli/{}", env!("CARGO_PKG_VERSION")))
            .build()?;

        let base_url = Url::parse(base_url)?;

        Ok(Self {
            client,
            base_url,
            timeout,
            retry_attempts,
        })
    }

    pub async fn authenticate(&self, username: &str, password: &str) -> Result<AuthResponse> {
        let auth_request = AuthRequest {
            username: username.to_string(),
            password: password.to_string(),
        };

        let url = self.base_url.join("/auth/login")?;

        let response = self.client.post(url).json(&auth_request).send().await?;

        if !response.status().is_success() {
            return Err(RigError::auth(format!(
                "Authentication failed with status: {}",
                response.status()
            )));
        }

        let auth_response: AuthResponse = response.json().await?;
        Ok(auth_response)
    }

    pub async fn get(&self, path: &str) -> Result<Response> {
        let url = self.base_url.join(path)?;
        let response = self.client.get(url).send().await?;
        Ok(response)
    }

    pub async fn post<T: Serialize>(&self, path: &str, body: &T) -> Result<Response> {
        let url = self.base_url.join(path)?;
        let response = self.client.post(url).json(body).send().await?;
        Ok(response)
    }
}