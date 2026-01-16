# ncbi_nuccore_rs
CLI tool for downloading FASTA files from NCBI nuccore.

## Installation
The easiest way to install is downloading a binary from [releases](`https://github.com/OscarAspelin95/ncbi_nuccore_rs/releases`).

## Usage
Run with:<br>
`ncbi_nuccore_rs --accession <accession> --outdir <outdir>`<br>

Required arguments:
<pre>
<b>--accession</b> NCBI nuccore accession(s).
<b>--outdir</b> Output directory.
</pre>

## Roadmap
- [] Add NCBI accession regex validation.
- [] Add support for other formats such as GenBank and GFF3
