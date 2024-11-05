use std::error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to parse {0}")]
    Url(#[from] url::ParseError),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("failed to make the request {0}")]
    Request(#[from] reqwest::Error),

    #[error("{0}")]
    RequestStatus(String),

    #[error("{0}")]
    Parse(#[from] serde_json::error::Error),

    #[error(transparent)]
    Config(#[from] figment::Error),

    #[error("unknown  error {0}")]
    Unknown(String),
}
