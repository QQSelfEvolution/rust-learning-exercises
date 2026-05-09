# File Hash Checker

A Rust security tool for computing and verifying file hashes.

## Features

- SHA256 and MD5 hash computation
- Batch file processing
- Hash verification (integrity check)
- Progress indication for large files
- Multiple output formats (hex, binary)

## Installation

```bash
cargo build --release
```

## Usage

### Compute hash of a single file

```bash
hash_tool hash --file document.pdf
```

### Compute hash using specific algorithm

```bash
hash_tool hash --file document.pdf --algo sha256
```

### Compute hashes for multiple files

```bash
hash_tool hash --files file1.txt file2.txt file3.txt
```

### Verify file against known hash

```bash
hash_tool verify --file document.pdf --hash abc123...
```

### Batch verify multiple files with a manifest

```bash
hash_tool batch --manifest hashes.txt --directory ./files
```

### Compare two files by hash

```bash
hash_tool compare --file1 original.pdf --file2 copy.pdf
```

## Options

- `--file, -f`: Single file to process
- `--files`: Multiple files to process
- `--algo, -a`: Hash algorithm (sha256, md5) [default: sha256]
- `--hash, -h`: Known hash for verification
- `--manifest, -m`: File containing known hashes (format: "hash  filename")
- `--directory, -d`: Directory to scan (for batch mode)
- `--recursive, -r`: Recursively scan directories
- `--output, -o`: Output format (text, json, csv)
- `--compare, -c`: Compare two files

## Hash Manifest Format

Create a file with known hashes:

```
abc123def456...  document1.pdf
789abc123def...  document2.pdf
```

## Examples

### Verify software download

```bash
# Download some software
curl -O https://example.com/software.zip

# Get expected hash from website
EXPECTED_HASH="5d41402abc4b2a76b9719d911017c592"

# Verify
hash_tool verify --file software.zip --hash $EXPECTED_HASH --algo md5
```

### Batch scan directory

```bash
# Create manifest
echo "abc123...  important.docx" > known_hashes.txt
echo "def456...  report.pdf" >> known_hashes.txt

# Verify all files
hash_tool batch --manifest known_hashes.txt --directory ./documents
```

### Generate and save hashes

```bash
hash_tool hash --files *.pdf --output json > hashes.json
```
