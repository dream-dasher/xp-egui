//! Very basic polars use.

use std::fs;
use std::path::Path;
use walkdir::WalkDir;

const DATA_DIR: &str = "data/example/";
fn main() {
        let path = Path::new(DATA_DIR);

        println!("----------------------------------------");
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
        println!("----------------------------------------");
        for entry in WalkDir::new(path) {
                let entry = entry.unwrap();
                println!("{}", entry.path().display());
        }
        println!("----------------------------------------");
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                println!("{}", entry.path().display());
        }
}
