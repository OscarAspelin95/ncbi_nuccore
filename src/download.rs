use crate::args::Format;
use crate::errors::AppError;
use crate::utils::{accession_norm_filt, ensure_dir, get_client, get_url};
use console::style;
use futures_util::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest_middleware::ClientWithMiddleware;
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Validate that the first bytes of the response look like the expected format.
/// NCBI returns HTTP 200 even for invalid accessions, but the body will contain
/// an error message (often HTML) instead of valid sequence data.
fn validate_first_bytes(bytes: &[u8], format: &Format) -> Result<(), AppError> {
    let trimmed = match std::str::from_utf8(bytes) {
        Ok(s) => s.trim_start(),
        Err(_) => {
            return Err(AppError::InvalidResponseError(
                "response is not valid UTF-8".to_string(),
            ));
        }
    };

    let valid = match format {
        Format::Fasta => trimmed.starts_with('>'),
        Format::Genbank => trimmed.starts_with("LOCUS"),
        Format::Gff3 => trimmed.starts_with("##gff-version"),
    };

    if !valid {
        let preview = &trimmed[..trimmed.len().min(200)];
        return Err(AppError::InvalidResponseError(preview.to_string()));
    }

    Ok(())
}

async fn download_file(
    client: &ClientWithMiddleware,
    url: &str,
    file_path: &Path,
    format: &Format,
) -> Result<(), AppError> {
    let response = client.get(url).send().await?;

    let response = match response.error_for_status() {
        Ok(r) => r,
        Err(e) => {
            return Err(AppError::StatusCodeError(format!(
                "Failed to download from {}. [ERROR]: `{}`",
                url, e
            )));
        }
    };

    let mut file = File::create(file_path).await?;
    let mut stream = response.bytes_stream();
    let mut first_chunk = true;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        if first_chunk {
            validate_first_bytes(&chunk, format)?;
            first_chunk = false;
        }

        file.write_all(&chunk).await?;
    }

    if first_chunk {
        // No chunks received at all — empty response
        return Err(AppError::InvalidResponseError(
            "empty response".to_string(),
        ));
    }

    file.flush().await?;
    Ok(())
}

pub async fn download_files(
    accessions: Vec<String>,
    outdir: &Path,
    formats: &[&Format],
) -> Result<(), AppError> {
    let accession_set = accession_norm_filt(accessions)?;
    let num_accessions = accession_set.len();

    let multi = MultiProgress::new();

    let overall_style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let overall = multi.add(ProgressBar::new(num_accessions as u64));
    overall.set_style(overall_style);
    overall.set_message("accessions");
    overall.enable_steady_tick(Duration::from_millis(200));

    let spinner_style =
        ProgressStyle::with_template("  {spinner:.cyan} {msg}")
            .unwrap();

    let client = get_client()?;

    for accession in &accession_set {
        let accession_dir = outdir.join(accession);
        ensure_dir(&accession_dir)?;

        let spinner = multi.insert_before(&overall, ProgressBar::new_spinner());
        spinner.set_style(spinner_style.clone());
        spinner.enable_steady_tick(Duration::from_millis(100));

        let mut all_ok = true;

        for format in formats {
            spinner.set_message(format!("{}: downloading {}...", accession, format));

            let url = get_url(accession, format);
            let file_name = format!("{}{}", accession, format.file_extension());
            let file_path = accession_dir.join(&file_name);

            match download_file(&client, &url, &file_path, format).await {
                Ok(()) => {}
                Err(e) => {
                    // Remove partial/invalid file
                    let _ = tokio::fs::remove_file(&file_path).await;
                    all_ok = false;
                    spinner.set_message(format!(
                        "{} {} ({}): {}",
                        style("✗").red(),
                        accession,
                        format,
                        e
                    ));
                }
            }
        }

        if all_ok {
            spinner.finish_with_message(format!(
                "{} {}",
                style("✓").green(),
                accession
            ));
        } else {
            spinner.finish();
        }

        overall.inc(1);
    }

    overall.finish_with_message(format!(
        "Processed {} accession(s)",
        num_accessions
    ));

    Ok(())
}
