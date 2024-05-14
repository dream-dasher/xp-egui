//! Very basic polars use.

use std::fs;
use std::path::Path;

const DATA_DIR: &str = "data/example/";
fn main() {
    let path = Path::new(DATA_DIR);

    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.is_file() {
                        println!("File: {}", path.display());
                    }
                }
            }
            Err(e) => eprintln!("Failed to read directory: {}", e),
        }
    }
}
