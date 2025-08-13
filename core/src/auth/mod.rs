use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{HttpClient, Result, RigError};

const KEYRING_SERVICE: &str = "rig-cli";
const KEYRING_USER: &str = "default";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub expires_at: Option<u64>,
}

#[derive(Debug)]
pub struct AuthClient {
    http_client: HttpClient,
    keyring: Entry,
}

impl AuthClient {
    pub fn new(http_client: HttpClient) -> Result<Self> {
        let keyring = Entry::new(KEYRING_SERVICE, KEYRING_USER)?;

        Ok(Self {
            http_client,
            keyring,
        })
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<()> {
        let auth_response = self.http_client.authenticate(username, password).await?;

        let expires_at = auth_response
            .expires_at
            .and_then(|exp| exp.parse::<u64>().ok());

        let token = AuthToken {
            token: auth_response.token,
            expires_at,
        };

        self.store_token(&token)?;
        Ok(())
    }

    pub fn get_token(&self) -> Result<Option<AuthToken>> {
        match self.keyring.get_password() {
            Ok(token_json) => {
                let token: AuthToken = serde_json::from_str(&token_json)?;

                // Check if token is expired
                if let Some(expires_at) = token.expires_at {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    if now >= expires_at {
                        // Token expired, remove it
                        self.logout()?;
                        return Ok(None);
                    }
                }

                Ok(Some(token))
            }
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(RigError::from(e)),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.get_token().unwrap_or(None).is_some()
    }

    pub fn logout(&self) -> Result<()> {
        match self.keyring.delete_password() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // Already logged out
            Err(e) => Err(RigError::from(e)),
        }
    }

    fn store_token(&self, token: &AuthToken) -> Result<()> {
        let token_json = serde_json::to_string(token)?;
        self.keyring.set_password(&token_json)?;
        Ok(())
    }
}
