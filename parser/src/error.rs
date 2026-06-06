use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("input was empty")]
    EmptyInput,
    #[error("failed to parse json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("expected the cv json document root to be an object")]
    UnsupportedTopLevel,
}