pub mod actions;
pub mod aliases;
pub mod concurrency;
pub mod config;
pub mod error;
pub mod filesystem;
pub mod filters;
pub mod locations;
pub mod parsers;
pub mod py_config;
pub mod rules;
pub mod runner;
pub mod ser_de;
pub mod state;
pub mod tags;

use crate::{
    error::{OrganizeResult, WalkerErrorKind},
    filters::{
        FilterApplicationKind, FilterCollection, FilterFilterClosureSliceMut, FilterGroup,
        FilterGroupCollection, FilterKind, FilterModeKind, RawFilterApplicationKind,
    },
    locations::{LocationCollection, LocationKind, MaxDepth, TargetKind},
};

use std::{fmt::Display, fs::FileType, ops::Not, path::Path};

use itertools::{Either, Itertools};
use jwalk::{ClientState, DirEntry, WalkDir};

pub struct IterCarry<'it, C: ClientState> {
    pub iter: &'it mut dyn Iterator<Item = DirEntry<C>>,
}

#[derive(Debug, Default)]
pub struct FilteredFileWalker {
    entries: Vec<DirEntry<((), ())>>,
}

impl FilteredFileWalker {
    pub const MAX_DEPTH: usize = 0;

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
        locations: LocationCollection,
        filters: FilterGroupCollection,
    ) {
        // extract ignore filters
        let (mut ignore_filters, other_filters): (Vec<_>, Vec<_>) =
            filters.iter().partition_map(|filter| match filter.mode() {
                FilterModeKind::None => Either::Left(filter),
                _ => Either::Right(filter),
            });

        // split off any / all filters
        let (mut any_filters, mut all_filters): (Vec<_>, Vec<_>) = other_filters
            .into_iter()
            .partition_map(|filter| match filter.mode() {
                FilterModeKind::Any => Either::Left(filter),
                FilterModeKind::All => Either::Right(filter),
                _ => unreachable!("There should be no items left in `FilterModeGroupKind::None`!"),
            });

        self.entries = Self::get_filtered_entries(
            locations,
            &mut ignore_filters,
            &mut any_filters,
            &mut all_filters,
        );
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
            Self::MAX_DEPTH
        };

        // TODO: Initialize indicatif progress bar

        let files = WalkDir::new(path)
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

    fn filter_group_applies(
        filter_group: &FilterGroup<Vec<FilterKind>>,
        entry: &DirEntry<((), ())>,
    ) -> bool {
        let (matched, not_matched): (Vec<bool>, Vec<bool>) = filter_group
            .filters()
            .iter()
            .map(|single_filter| single_filter.get_filter()(entry))
            .partition(|f| *f);

        match (filter_group.apply(), filter_group.mode()) {
            (RawFilterApplicationKind::Exclude, FilterModeKind::All)
            | (RawFilterApplicationKind::Include, FilterModeKind::All) => not_matched.is_empty(),
            (RawFilterApplicationKind::Exclude, FilterModeKind::Any)
            | (RawFilterApplicationKind::Include, FilterModeKind::Any) => !matched.is_empty(),
            (RawFilterApplicationKind::Exclude, FilterModeKind::None)
            | (RawFilterApplicationKind::Include, FilterModeKind::None) => matched.is_empty(),
        }
    }

    fn get_filtered_entries<'a>(
        locations: LocationCollection,
        ignore_filters: &'a mut [&FilterGroup<Vec<FilterKind>>],
        any_filters: &'a mut [&FilterGroup<Vec<FilterKind>>],
        all_filters: &'a mut [&FilterGroup<Vec<FilterKind>>],
    ) -> Vec<DirEntry<((), ())>> {
        locations
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
            // .inspect(|f| println!("filter matched: {f:?}"))
            .flatten_ok()
            .flat_map(std::result::Result::ok)
            .flat_map(|entry| {
                if !ignore_filters.is_empty() {
                    ignore_filters
                        .iter()
                        // .inspect(|f| println!("Applying ignore filter: {f}"))
                        .map(|filter| Self::filter_group_applies(filter, &entry))
                        .all(|f| matches!(f, false))
                        .then_some(entry)
                } else {
                    Some(entry)
                }
            })
            .flat_map(|entry| {
                if !all_filters.is_empty() {
                    all_filters
                        .iter()
                        // .inspect(|f| println!("Applying filter: {f}"))
                        .map(|filter| Self::filter_group_applies(filter, &entry))
                        .all(|f| matches!(f, true))
                        .then_some(entry)
                } else {
                    Some(entry)
                }
            })
            .flat_map(|entry| {
                if !any_filters.is_empty() {
                    any_filters
                        .iter()
                        // .inspect(|f| println!("Applying filter: {f}"))
                        .map(|filter| Self::filter_group_applies(filter, &entry))
                        .any(|f| matches!(f, true))
                        .then_some(entry)
                } else {
                    Some(entry)
                }
            })
            .collect_vec()
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
