pub mod rules;

use std::path::Path;

use walkdir::WalkDir;

pub fn file_walker(path: impl AsRef<Path>) {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }
}
