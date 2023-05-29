pub mod ser_de;
pub mod actions;
pub mod aliases;
pub mod config;
pub mod error;
pub mod filters;
pub mod locations;
pub mod parsers;
pub mod py_config;
pub mod rules;
pub mod state;
pub mod tags;

use crate::{
    error::{OrganizeResult, WalkerErrorKind},
    filters::{
        FilterApplicationKind, FilterCollection, FilterFilterClosureSliceMut, FilterKind,
        FilterModeKind,
    },
    locations::{LocationKind, MaxDepth, TargetKind},
};

use std::{fmt::Display, fs::FileType, ops::Not, path::Path};

use itertools::{Either, Itertools};
use jwalk::{ClientState, DirEntry, WalkDir};

pub mod constants {
    pub const MAX_DEPTH: usize = 0;
}

pub struct IterCarry<'it, C: ClientState> {
    pub iter: &'it mut dyn Iterator<Item = DirEntry<C>>,
}

#[derive(Debug, Default)]
pub struct FilteredFileWalker {
    entries: Vec<DirEntry<((), ())>>,
}

impl FilteredFileWalker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn print_entries(&self) {
        let count = self.entries.len();
        self.entries().iter().for_each(|f| {
            println!("{f:?}");
        });
        println!("Total entry count: {count}");
    }

    pub fn entries(&self) -> &[DirEntry<((), ())>] {
        &self.entries
    }

    pub fn get_applicable_items(
        &mut self,
        locations: Vec<LocationKind>,
        filters: FilterCollection,
    ) {
        // print out filters that will be applied
        println!("{filters}");

        // extract ignore filters
        let (mut ignore_filters, other_filters): (Vec<_>, Vec<_>) = filters
            .decompose()
            .into_iter()
            .partition_map(|filter| match filter {
                (FilterModeKind::None, value) => Either::Left(value),
                other => Either::Right(other),
            });

        // split off any / all filters
        let (mut any_filters, mut all_filters): (Vec<_>, Vec<_>) = other_filters
            .into_iter()
            .partition_map(|filter| match filter {
                (FilterModeKind::Any, value_any) => Either::Left(value_any),
                (FilterModeKind::All, value_all) => Either::Right(value_all),
                _ => unreachable!("There should be no items left in `FilterModeGroupKind::None`!"),
            });

        self.entries = Self::get_filtered_iterator(
            locations,
            &mut ignore_filters,
            &mut any_filters,
            &mut all_filters,
        )
        .collect_vec();
    }

    fn populate_entries<A>(
        path: A,
        max_depth: impl Into<Option<MaxDepth>>,
        targets: TargetKind,
    ) -> OrganizeResult<Vec<DirEntry<((), ())>>>
    where
        A: AsRef<Path>,
    {
        let depth = if let Some(max_depth) = max_depth.into() {
            usize::try_from(*max_depth).map_err(WalkerErrorKind::FailedToConvertNumbers)?
        } else {
            constants::MAX_DEPTH
        };

        println!("We are getting entries ...");

        let files = WalkDir::new(path)
            .max_depth(depth)
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| match targets {
                TargetKind::Directories => FileType::is_dir(&f.file_type()),
                TargetKind::Files => FileType::is_file(&f.file_type()),
                TargetKind::Both => true,
            })
            .collect_vec();

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
        entries: Vec<DirEntry<((), ())>>,
        filters: FilterFilterClosureSliceMut<((), ())>,
    ) -> Vec<DirEntry<((), ())>> {
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

    fn filter_applies(
        filter: &FilterApplicationKind<FilterKind>,
        entry: &DirEntry<((), ())>,
    ) -> bool {
        match filter {
            FilterApplicationKind::Apply(filt) => filt.get_filter()(entry),
            FilterApplicationKind::Invert(inv_filt) => inv_filt.get_filter()(entry).not(),
        }
    }

    fn get_filtered_iterator<'a>(
        locations: Vec<LocationKind>,
        ignore_filters: &'a mut [FilterApplicationKind<FilterKind>],
        any_filters: &'a mut [FilterApplicationKind<FilterKind>],
        all_filters: &'a mut [FilterApplicationKind<FilterKind>],
    ) -> impl Iterator<Item = DirEntry<((), ())>> + 'a {
        locations
            .into_iter()
            .unique()
            .map(|location| match location {
                LocationKind::RecursiveWithMaxDepth {
                    path,
                    max_depth,
                    target,
                } => Self::populate_entries(path, max_depth, target),
                LocationKind::NonRecursive { path, target } => {
                    Self::populate_entries(path, None, target)
                }
                LocationKind::BarePath(path) => {
                    Self::populate_entries(path, None, TargetKind::default())
                }
            })
            // .inspect(|f| println!("filter matched: {f:?}"))
            .flatten_ok()
            .filter_map(std::result::Result::ok)
            .filter_map(|entry| {
                ignore_filters
                    .iter()
                    // .inspect(|f| println!("Applying ignore filter: {f}"))
                    .map(|filter| Self::filter_applies(filter, &entry))
                    .all(|f| matches!(f, false))
                    .then_some(entry)
            })
            .filter_map(|entry| {
                any_filters
                    .iter()
                    // .inspect(|f| println!("Applying filter: {f}"))
                    .map(|filter| Self::filter_applies(filter, &entry))
                    .any(|f| matches!(f, true))
                    .then_some(entry)
            })
            .filter_map(|entry| {
                all_filters
                    .iter()
                    // .inspect(|f| println!("Applying filter: {f}"))
                    .map(|filter| Self::filter_applies(filter, &entry))
                    .all(|f| matches!(f, true))
                    .then_some(entry)
            })
    }
}

impl Display for FilteredFileWalker {
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
