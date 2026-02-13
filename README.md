# ncbi_nuccore_rs
CLI tool for downloading sequence files from NCBI nuccore in FASTA, GenBank, or GFF3 format.

## Installation
The easiest way to install is downloading a binary from [releases](`https://github.com/OscarAspelin95/ncbi_nuccore_rs/releases`).

## Usage
Run with:<br>
`ncbi_nuccore_rs --accession <accession> --outdir <outdir> [--format <format>...]`<br>

Required arguments:
<pre>
<b>--accession, -a</b> NCBI nuccore accession(s), space-delimited.
<b>--outdir, -o</b>    Output directory.
</pre>

Optional arguments:
<pre>
<b>--format, -f</b>    Output format(s): fasta, genbank, gff3. Default: fasta. Space-delimited for multiple.
</pre>

## Examples

Download a single accession in FASTA format (default):
```
ncbi_nuccore_rs -a AP022815.1 -o output
```

Download in multiple formats:
```
ncbi_nuccore_rs -a AP022815.1 -o output -f fasta genbank gff3
```

Download multiple accessions:
```
ncbi_nuccore_rs -a AP022815.1 CP000001.1 -o output -f fasta gff3
```

Output is organized as `<outdir>/<accession>/<accession>.<ext>`, e.g.:
```
output/
  AP022815.1/
    AP022815.1.fasta
    AP022815.1.gb
    AP022815.1.gff3
```
