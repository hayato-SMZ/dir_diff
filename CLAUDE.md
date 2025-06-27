# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust CLI tool called `dir_diff` that compares files between directories using SHA-256 hash-based comparison. The tool recursively scans source and target directories, calculates file hashes, and identifies differences, missing files, and unchanged files.

## Development Commands

### Build and Test
```bash
# Build the project (release mode for production)
cargo build --release

# Build for development/debugging
cargo build

# Run tests
cargo test --verbose

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy for linting
cargo clippy
```

### Running the Application
```bash
# Basic usage
./target/release/dir_diff --source /path/to/source --target /path/to/target

# With custom output file
./target/release/dir_diff --source /path/to/source --target /path/to/target --out custom_output.txt

# Development run
cargo run -- --source /path/to/source --target /path/to/target
```

## Architecture

### Core Components

**Main Entry Point (`src/main.rs`)**
- Uses `clap` for command-line argument parsing
- Coordinates the comparison process through `ComparsionSource`

**Comparison Engine (`src/diff_lib/comparsion_source.rs`)**
- `ComparsionSource` struct manages the entire comparison workflow
- Async/parallel processing using `tokio` and `futures` for performance
- File discovery, hash calculation, and comparison logic
- Tracks comparison results: errors, not found files, and unchanged files

**File Information (`src/diff_lib/file_infomation.rs`)**
- `FileInfomation` struct represents individual file metadata
- Handles path normalization and hash generation
- Uses SHA-256 for path hashing and DefaultHasher for file content hashing

### Key Workflow
1. **Source Analysis**: Recursively scans source directory, builds file inventory with path hashes
2. **Hash Calculation**: Parallel computation of file content hashes for source files
3. **Target Discovery**: Scans target directory and builds comparison file list
4. **Comparison**: Parallel hash comparison between source and target files
5. **Report Generation**: Outputs detailed comparison results to text file

### Dependencies
- `sha2`: SHA-256 hashing for path normalization
- `clap`: Command-line interface with derive macros
- `tokio`: Async runtime with full feature set
- `futures`: Future utilities for parallel processing

## Testing

The project includes unit tests and integration tests:
- Tests are embedded in source files using `#[cfg(test)]`
- Test data is located in `test/source/` and `test/target/` directories
- Run tests with `cargo test --verbose`

## CI/CD

GitHub Actions workflow (`.github/workflows/rust.yml`):
- Triggers on tags matching `rel-**` pattern
- Runs tests and builds for `x86_64-pc-windows-gnu` target
- Creates GitHub releases with compiled binaries

## Output Format

Results are written to a text file containing:
- Processing time and file counts
- Lists of error files (hash mismatches)
- Not found files (missing in source)
- Not compared files (missing in target)