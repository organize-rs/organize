pub mod error;
pub mod locations;
pub mod parsers;
pub mod rules;

use crate::{
    error::OrganizeResult,
    locations::{MaxDepth, OrganizeLocation, OrganizeTarget},
    rules::filters::OrganizeFilter,
};

use std::{fmt::Display, fs::FileType, path::Path};

use itertools::Itertools;
use walkdir::{DirEntry, WalkDir};

pub mod constants {
    pub const MAX_DEPTH: usize = 0;
}

pub struct IterCarry<'it> {
    pub iter: &'it mut dyn Iterator<Item = DirEntry>,
}

#[derive(Debug, Clone, Default)]
pub struct FilterWalker {
    entries: Vec<DirEntry>,
}

impl FilterWalker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn print_entries(&self) {
        self.entries().iter().for_each(|f| println!("{f:?}"));
    }

    pub fn entries(&self) -> &[DirEntry] {
        &self.entries
    }

    pub fn get_applicable_items(
        &mut self,
        locations: Vec<OrganizeLocation>,
        filters: Vec<OrganizeFilter>,
    ) {
        // extract ignore filters
        let (ignore_filters, other_filters): (Vec<_>, Vec<_>) = filters
            .into_iter()
            .partition(|filter| filter.is_ignore_name() | filter.is_ignore_path());

        self.entries = locations
            .into_iter()
            .map(|location| match location {
                OrganizeLocation::RecursiveWithMaxDepth {
                    path,
                    max_depth,
                    target,
                } => Self::populate_entries(path, max_depth, target),
                OrganizeLocation::NonRecursive { path, target } => {
                    Self::populate_entries(path, None, target)
                }
            })
            // .inspect(|f| println!("filter matched: {f:?}"))
            .flatten_ok()
            .filter_map(std::result::Result::ok)
            .filter_map(|entry| {
                ignore_filters
                    .iter()
                    // .inspect(|f| println!("Applying ignore filter: {f}"))
                    .map(|filter| filter.get_filter()(&entry))
                    .all(|f| matches!(f, false))
                    .then_some(entry)
            })
            .filter_map(|entry| {
                other_filters
                    .iter()
                    // .inspect(|f| println!("Applying filter: {f}"))
                    .map(|filter| filter.get_filter()(&entry))
                    .any(|f| matches!(f, true))
                    .then_some(entry)
            })
            .collect();
    }

    fn populate_entries<A>(
        path: A,
        max_depth: impl Into<Option<MaxDepth>>,
        targets: OrganizeTarget,
    ) -> OrganizeResult<Vec<DirEntry>>
    where
        A: AsRef<Path>,
    {
        let depth = if let Some(max_depth) = max_depth.into() {
            usize::try_from(*max_depth)?
        } else {
            constants::MAX_DEPTH
        };

        println!("We are getting entries ...");

        let files: Vec<DirEntry> = WalkDir::new(path)
            .max_depth(depth)
            .contents_first(true)
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| match targets {
                OrganizeTarget::Dirs => FileType::is_dir(&f.file_type()),
                OrganizeTarget::Files => FileType::is_file(&f.file_type()),
                OrganizeTarget::Both => true,
            })
            .collect();

        Ok(files)
    }

    // pub fn apply_single_filter(
    //     entries: &DirEntry,
    //     mut filter: Box<dyn FnMut(&DirEntry) -> bool>,
    // ) -> Vec<DirEntry> {
    //     entries
    //         .into_iter()
    //         .filter_map(|f| filter(&f).then_some(f))
    //         .collect_vec()
    // }

    pub fn apply_filters(
        entries: Vec<DirEntry>,
        filters: &mut [Box<dyn FnMut(&DirEntry) -> bool>],
    ) -> Vec<DirEntry> {
        entries
            .into_iter()
            .filter(|entry| {
                let mut results = vec![];
                filters
                    .iter_mut()
                    .for_each(|filter| results.push(filter(entry)));
                results.contains(&true)
            })
            .collect_vec()
    }
}

impl Display for FilterWalker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let last_five = self.entries.iter().rev().take(5).collect_vec();
        write!(
            f,
            "
        FilterWalker
        
        Last 5 entries:
        {:?}
        ",
            last_five
        )
    }
}
