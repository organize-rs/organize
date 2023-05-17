pub mod rules;

use crate::rules::filters::OrganizeFilter;
use itertools::Itertools;
use std::path::Path;
use std::{error::Error, fs::File};
use walkdir::{DirEntry, WalkDir};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn filter_walker<F, A, I>(path: A, filter: F, max_depth: I) -> Vec<DirEntry>
where
    F: FnMut(&DirEntry) -> bool,
    A: AsRef<Path>,
    I: Into<Option<usize>>,
{
    let depth = max_depth.into().unwrap_or(0);

    let filtered_files: Vec<DirEntry> = WalkDir::new(path)
        .max_depth(depth)
        .into_iter()
        .filter_ok(filter)
        .filter_map(|f| f.ok())
        .collect();

    filtered_files
}
