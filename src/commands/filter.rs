//! Filters that organize can apply

use std::path::PathBuf;

use abscissa_core::{Command, Runnable};
use clap::Args;
use itertools::Itertools;
use organize_rs_core::{
    filters::{
        FilterApplicationKind, FilterCollection, FilterGroup, FilterGroupCollection, FilterKind,
        FilterModeKind, RawFilterApplicationKind, RecursiveFilterArgs,
    },
    locations::{LocationCollection, LocationKind, MaxDepth, TargetKind},
    FilteredFileWalker,
};

/// `filter` subcommand
#[derive(Command, Debug, Args, Clone)]
pub struct FilterCmd {
    #[clap(subcommand)]
    filters: FilterKind,

    /// Words in file names to be ignored
    #[arg(long, global = true)]
    ignore_name: Option<Vec<String>>,

    /// Words in Paths to be ignored
    #[arg(long, global = true)]
    ignore_path: Option<Vec<String>>,

    /// Mode, how the filters should apply
    #[arg(short, long, global = true, default_value_t = FilterModeKind::Any, value_enum)]
    filter_mode: FilterModeKind,

    #[command(flatten)]
    location_opts: LocationOpts,
}

#[derive(Debug, Args, Clone)]
#[group(id = "location")]
pub struct LocationOpts {
    /// Locations to operate on
    #[arg(short, long, global = true)]
    locations: Vec<PathBuf>,

    #[command(flatten)]
    recursive: RecursiveFilterArgs,

    /// Targets to operate on
    #[arg(short, long, global = true, default_value_t = TargetKind::Files, value_enum)]
    targets: TargetKind,
}

impl Runnable for FilterCmd {
    fn run(&self) {
        let filter_group0 = FilterGroup::new(
            RawFilterApplicationKind::Apply,
            self.filter_mode,
            vec![self.filters.clone()],
        );

        let mut filters = vec![];

        if let Some(ignore_names) = self.ignore_name.clone() {
            filters.push(FilterKind::IgnoreName {
                in_name: ignore_names,
            });
        };

        if let Some(ignore_paths) = self.ignore_path.clone() {
            filters.push(FilterKind::IgnorePath {
                in_path: ignore_paths,
            });
        };

        let filter_group1 = FilterGroup::new(
            RawFilterApplicationKind::Apply,
            FilterModeKind::None,
            filters,
        );

        let filter_group_collection =
            FilterGroupCollection::from_vec(vec![filter_group0, filter_group1]);

        self.inner_run(filter_group_collection);
    }
}

impl FilterCmd {
    fn inner_run(&self, filters: FilterGroupCollection) {
        let mut filter_walker = FilteredFileWalker::new();

        // Convert to OrganizeLocation
        let locations = self
            .location_opts
            .locations
            .clone()
            .into_iter()
            .map(|f| {
                if self.location_opts.recursive.recursive() {
                    LocationKind::from((
                        f,
                        MaxDepth::new(self.location_opts.recursive.max_depth()),
                        self.location_opts.targets,
                    ))
                } else {
                    LocationKind::from((f, self.location_opts.targets))
                }
            })
            // .inspect(|f| println!("Got the following locations: {f}"))
            .collect_vec();

        let location_collection = LocationCollection::from_vec(locations);

        filter_walker.get_applicable_items(location_collection, filters);

        filter_walker.print_entries();

        // let _test = filter_walker
        //     .entries()
        //     .iter()
        //     .inspect(|dir_entry| {
        //         println!(
        //             "{}\t\"{}\"",
        //             if dir_entry.path().is_dir() {
        //                 "D"
        //             } else if dir_entry.path().is_file() {
        //                 "F"
        //             } else {
        //                 "L"
        //             },
        //             dir_entry.path().display()
        //         );
        //     })
        //     .collect_vec();
    }
}
