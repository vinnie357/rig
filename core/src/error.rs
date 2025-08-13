use thiserror::Error;

pub type Result<T> = std::result::Result<T, RigError>;

#[derive(Error, Debug)]
pub enum RigError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Credential storage error: {0}")]
    Keyring(#[from] keyring::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Generic error: {0}")]
    Generic(String),
}

impl RigError {
    pub fn auth<S: Into<String>>(msg: S) -> Self {
        RigError::Auth(msg.into())
    }

    pub fn generic<S: Into<String>>(msg: S) -> Self {
        RigError::Generic(msg.into())
    }
}
