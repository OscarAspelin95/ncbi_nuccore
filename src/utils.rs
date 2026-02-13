use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use std::collections::HashSet;
use std::path::Path;

use crate::args::Format;
use crate::errors::AppError;

pub fn ensure_dir(path: &Path) -> Result<(), std::io::Error> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn get_url(accession: &str, format: &Format) -> String {
    format!(
        "https://www.ncbi.nlm.nih.gov/sviewer/viewer.fcgi?id={}&db=nuccore&report={}&retmode=text",
        accession,
        format.as_report_param()
    )
}

pub fn accession_norm_filt(accessions: Vec<String>) -> Result<HashSet<String>, AppError> {
    let normfilt: HashSet<String> = accessions
        .iter()
        .map(|accession| accession.trim().to_ascii_uppercase())
        .collect();

    match normfilt.is_empty() {
        true => Err(AppError::EmptyAccessionList),
        false => Ok(normfilt),
    }
}

pub fn get_client() -> Result<ClientWithMiddleware, AppError> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    Ok(client)
}
