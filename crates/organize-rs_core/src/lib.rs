pub mod error;
pub mod locations;
pub mod parsers;
pub mod rules;

use crate::{
    error::OrganizeResult,
    locations::{MaxDepth, OrganizeLocation, OrganizeTarget},
};

use std::{fs::FileType, path::Path, vec};

use itertools::{enumerate, Itertools};
use walkdir::{DirEntry, WalkDir};

pub mod constants {
    pub const MAX_DEPTH: usize = 0;
}

pub struct IterCarry<'it> {
    pub iter: &'it mut dyn Iterator<Item = DirEntry>,
}

#[derive(Debug)]
pub struct FilterWalker {}

impl FilterWalker {
    pub fn get_applicable_items<'carry, 'iter>(
        locations: &'iter mut impl Iterator<Item = OrganizeLocation>,
    ) -> Vec<DirEntry> {
        locations
            .map(|location| match location {
                OrganizeLocation::RecursiveWithMaxDepth {
                    path,
                    max_depth,
                    target,
                } => Self::entries(path, max_depth, target),
                OrganizeLocation::NonRecursive { path, target } => {
                    Self::entries(path, None, target)
                }
            })
            .flatten_ok()
            .filter_map(std::result::Result::ok)
            .collect_vec()
    }

    pub fn entries<A>(
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

    pub fn apply_single_filter(
        entries: &mut impl Iterator<Item = DirEntry>,
        filter: Box<dyn FnMut(&DirEntry) -> bool>,
    ) -> Vec<DirEntry> {
        entries.into_iter().filter(filter).collect_vec()
    }

    pub fn apply_filters(
        entries: Vec<DirEntry>,
        filters: Vec<Box<dyn FnMut(&DirEntry) -> bool>>,
    ) -> Vec<DirEntry> {
        let mut vec = vec![];
        filters.into_iter().enumerate().for_each(|(idx, filter)| {
            vec = if idx == 1 {
                Self::apply_single_filter(&mut entries.clone().into_iter(), filter)
            } else {
                Self::apply_single_filter(&mut vec.clone().into_iter(), filter)
            };
        });

        vec
    }
}
