use clap::{Arg, ArgAction, Command};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let matches = Command::new("purge")
        .version("0.1.0")
        .author("Ryan Horner <ryan1horner@gmail.com>")
        .about("Deletes files based on specified criteria")
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .action(ArgAction::SetTrue)
                .help("Recursively delete files in subdirectories"),
        )
        .arg(
            Arg::new("ftype")
                .long("ftype")
                .value_name("FILETYPE")
                .num_args(1)
                .help("Specify the file type to delete (e.g., mp3, mp4)"),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .value_name("DIRECTORY")
                .num_args(1)
                .required(true)
                .help("Specify the target directory (e.g., ./)"),
        )
        .arg(
            Arg::new("dry_run")
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("Show what would be deleted without deleting files"),
        )
        .get_matches();

    let recursive = matches.get_flag("recursive");
    let ftype = matches.get_one::<String>("ftype");
    let dir = matches.get_one::<String>("dir").unwrap(); // Safe unwrap because it's required
    let dry_run = matches.get_flag("dry_run");

    let dir_path = Path::new(dir);
    if !dir_path.is_dir() {
        eprintln!("Error: The specified path is not a directory.");
        std::process::exit(1);
    }

    let walker = if recursive {
        WalkDir::new(dir_path).into_iter()
    } else {
        WalkDir::new(dir_path).max_depth(1).into_iter()
    };

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if path == dir_path {
                    continue;
                }

                if path.is_file() {
                    if let Some(ftype) = ftype {
                        if let Some(ext) = path.extension() {
                            if ext.to_string_lossy().eq_ignore_ascii_case(ftype) {
                                if dry_run {
                                    println!("Would delete: {}", path.display());
                                } else {
                                    delete_file(path);
                                }
                            }
                        }
                    } else {
                        if dry_run {
                            println!("Would delete: {}", path.display());
                        } else {
                            delete_file(path);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error accessing entry: {}", e),
        }
    }
}

fn delete_file(path: &Path) {
    match fs::remove_file(path) {
        Ok(_) => println!("Deleted: {}", path.display()),
        Err(e) => eprintln!("Failed to delete {}: {}", path.display(), e),
    }
}
