mod args;
mod download;
mod errors;
mod utils;

use args::App;
use clap::Parser;
use download::download_files;
use errors::AppError;
use std::collections::HashSet;
use utils::ensure_dir;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args = App::parse();
    ensure_dir(&args.outdir)?;

    let App {
        accession,
        outdir,
        format,
    } = args;

    let mut seen = HashSet::new();
    let formats: Vec<_> = format
        .into_iter()
        .filter(|f| seen.insert(f.clone()))
        .collect();
    let format_refs: Vec<_> = formats.iter().collect();
    download_files(accession, &outdir, &format_refs).await?;

    Ok(())
}
