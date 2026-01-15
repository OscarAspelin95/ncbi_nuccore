import argparse
from pathlib import Path

from yaspin import yaspin

from ncbi_download import download_fasta, get_url, valid_accession
from utils import _ensure_dir


def main(accessions: list[str], outdir: Path):
    accessions = set(map(lambda x: x.strip(), accessions))

    num_accessions = len(accessions)
    placeholder = f"Downloading {num_accessions} accessions"

    err = False

    with yaspin(text=placeholder, color="cyan", timer=True) as sp:
        for i, accession in enumerate(accessions, start=1):
            if not valid_accession(accession):
                sp.write(f"{accession} not a valid accession. Skipping.")
                err = True
                continue

            url = get_url(accession)

            _ = download_fasta(url, accession, outdir)
            sp.write(f"{accession} ✓")
            sp.text = f"{placeholder} ({i}/{num_accessions})"

    match err:
        case True:
            sp.fail("Some accessions failed")
        case False:
            sp.ok()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Download NCBI FASTA file(s) from nuccore"
    )
    parser.add_argument(
        "-a", "--accession", nargs="+", help="NCBI accession number", required=True
    )
    parser.add_argument(
        "-o",
        "--outdir",
        help="Output directory",
        type=Path,
        required=False,
        default="./ncbi_nuccore_download",
    )
    args = parser.parse_args()
    outdir = _ensure_dir(args.outdir)

    main(args.accession, outdir)
