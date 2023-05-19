pub mod error;
pub mod rules;

use crate::error::OrganizeResult;

use std::path::Path;

use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub struct FilterWalker {}

impl FilterWalker {
    pub fn entries<A>(path: A, max_depth: u64) -> OrganizeResult<Vec<DirEntry>>
    where
        A: AsRef<Path>,
    {
        let depth = usize::try_from(max_depth)?;

        let files: Vec<DirEntry> = WalkDir::new(path)
            .max_depth(depth)
            .contents_first(true)
            .into_iter()
            .filter_map(|f| f.ok())
            .collect();

        Ok(files)
    }
}
