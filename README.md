# dir_diff
This project is a CLI tool to compare files between directories

# build
```
>cargo build --release
```

# use
```
# Basic usage (single-thread mode - default)
dir_diff --source {Full path of the comparison source} --target {Full path of comparison destination} --out {out file name (optional)}

# Multi-thread mode for faster processing
dir_diff --source {Full path of the comparison source} --target {Full path of comparison destination} --out {out file name (optional)} --multi-thread

# Examples:
# Single-thread mode (recommended for CPU-intensive environments)
dir_diff --source /path/to/source --target /path/to/target --out comparison_result.txt

# Multi-thread mode (faster, but uses more CPU resources)
dir_diff --source /path/to/source --target /path/to/target --out comparison_result.txt --multi-thread
```