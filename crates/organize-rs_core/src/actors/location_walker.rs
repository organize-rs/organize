use itertools::Itertools;
use jwalk::{ClientState, DirEntry};
use std::{fmt::Display, fs::FileType, path::Path, slice::Iter, vec::IntoIter};

use crate::{
    error::{OrganizeResult, WalkerErrorKind},
    locations::{LocationCollection, LocationKind, MaxDepth, TargetKind},
};

pub struct IterCarry<'it, C: ClientState> {
    pub iter: &'it mut dyn Iterator<Item = jwalk::DirEntry<C>>,
}

#[derive(Debug, Default)]
pub struct DirEntryData(Vec<jwalk::DirEntry<((), ())>>);

impl IntoIterator for DirEntryData {
    type Item = jwalk::DirEntry<((), ())>;

    type IntoIter = IntoIter<DirEntry<((), ())>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl DirEntryData {
    pub fn print_entries(&self) {
        let count = self.0.len();
        self.0.iter().for_each(|f| {
            println!("{f:?}");
        });
        println!("Total entry count: {count}");
    }

    pub(crate) fn iter(&self) -> Iter<'_, jwalk::DirEntry<((), ())>> {
        self.0.iter()
    }
}

impl Display for DirEntryData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let last_five = self.0.iter().rev().take(5).collect_vec();
        write!(
            f,
            "
        FilterApplicator
        
        Last 5 entries:
        {:?}
        ",
            last_five
        )
    }
}

impl From<Vec<jwalk::DirEntry<((), ())>>> for DirEntryData {
    fn from(value: Vec<jwalk::DirEntry<((), ())>>) -> Self {
        Self(value)
    }
}

// impl std::ops::DerefMut for DirEntryData {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl std::ops::Deref for DirEntryData {
//     type Target = Vec<jwalk::DirEntry<((), ())>>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

#[derive(Debug, Default)]
pub struct LocationWalker {
    locations: LocationCollection,
}

impl LocationWalker {
    pub const DEFAULT_MAX_DEPTH: usize = 0;

    pub fn new(locations: LocationCollection) -> Self {
        Self {
            locations,
            ..Default::default()
        }
    }

    pub fn collect_dir_entry_data(&mut self) -> DirEntryData {
        let entries = self
            .locations
            .iter()
            .unique()
            .map(|location| match location {
                LocationKind::RecursiveWithMaxDepth {
                    path,
                    max_depth,
                    target,
                } => Self::populate_entries(path, *max_depth, *target),
                LocationKind::NonRecursive { path, target } => {
                    Self::populate_entries(path, None, *target)
                }
                LocationKind::BarePath(path) => {
                    Self::populate_entries(path, None, TargetKind::default())
                }
            })
            .flatten_ok()
            .flat_map(std::result::Result::ok)
            .collect_vec();

        DirEntryData::from(entries)
    }

    fn populate_entries<A>(
        path: A,
        max_depth: impl Into<Option<MaxDepth>>,
        targets: TargetKind,
    ) -> OrganizeResult<Vec<jwalk::DirEntry<((), ())>>>
    where
        A: AsRef<Path>,
    {
        let depth = if let Some(max_depth) = max_depth.into() {
            usize::try_from(*max_depth).map_err(WalkerErrorKind::FailedToConvertNumbers)?
        } else {
            Self::DEFAULT_MAX_DEPTH
        };

        // TODO: Initialize indicatif progress bar

        let files = jwalk::WalkDir::new(path)
            .max_depth(depth)
            .into_iter()
            .flat_map(|f| f.ok())
            .filter(|f| match targets {
                TargetKind::Directories => FileType::is_dir(&f.file_type()),
                TargetKind::Files => FileType::is_file(&f.file_type()),
                TargetKind::Both => true,
            })
            .collect_vec();

        Ok(files)
    }
}
