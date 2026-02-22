use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Request error: {0}")]
    ReqwestMiddlewareError(#[from] reqwest_middleware::Error),

    #[error("Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Download error: {0}")]
    StatusCodeError(String),

    #[error("Invalid response (accession likely does not exist): {0}")]
    InvalidResponseError(String),

    #[error("No valid accessions provided")]
    EmptyAccessionList,
}
