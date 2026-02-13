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

async fn download_file(
    client: &ClientWithMiddleware,
    url: &str,
    file_path: &Path,
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

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
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

            match download_file(&client, &url, &file_path).await {
                Ok(()) => {}
                Err(e) => {
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
