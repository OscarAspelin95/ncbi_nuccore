use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Format {
    Fasta,
    Genbank,
    Gff3,
}

impl Format {
    pub fn as_report_param(&self) -> &str {
        match self {
            Format::Fasta => "fasta",
            Format::Genbank => "genbank",
            Format::Gff3 => "gff3",
        }
    }

    pub fn file_extension(&self) -> &str {
        match self {
            Format::Fasta => ".fasta",
            Format::Genbank => ".gb",
            Format::Gff3 => ".gff3",
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_report_param())
    }
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Download sequence files from NCBI nuccore in FASTA, GenBank, or GFF3 format."
)]
pub struct App {
    #[arg(short, long, required = true, value_delimiter = ' ', num_args = 1..)]
    pub accession: Vec<String>,

    /// The output directory
    #[arg(short, long, required = true)]
    pub outdir: PathBuf,

    /// Output format(s) to download
    #[arg(short, long, value_delimiter = ' ', num_args = 1.., default_value = "fasta")]
    pub format: Vec<Format>,
}

