# purge-rust
Using Rust for the first time :D

# Purge
Purge is a command-line tool for deleting files based on specified criteria.

## Features
- Delete files in a specified directory
- Optional recursive deletion in subdirectories
- Filter files by file type
- Dry-run option to preview deletions without actually deleting files

## Usage
purge [OPTIONS] --dir <DIRECTORY>


### Options:
- `-r, --recursive`: Recursively delete files in subdirectories
- `--ftype <FILETYPE>`: Specify the file type to delete (e.g., mp3, mp4)
- `--dir <DIRECTORY>`: Specify the target directory (e.g., ./) (required)
- `--dry-run`: Show what would be deleted without actually deleting files
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Examples
Delete all files in the current directory:
purge --dir ./

Recursively delete all MP3 files in the Music directory:
purge --dir ./Music --recursive --ftype mp3


Preview deletion of all PNG files in the Images directory:
purge --dir ./Images --ftype png --dry-run


## Building
To build the project, make sure you have Rust installed, then run:
cargo build --release

## Install to path (optional)
cargo install --path .
export PATH="$HOME/.cargo/bin:$PATH"

source ~/.bashrc
# or
source ~/.zshrc

# Verify Installation:
purge --help
