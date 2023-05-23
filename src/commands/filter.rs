//! Filters that organize can apply

use std::path::PathBuf;

use abscissa_core::{Command, Runnable};
use clap::Args;
use itertools::Itertools;
use organize_rs_core::{
    locations::{MaxDepth, OrganizeLocation, OrganizeTarget},
    rules::filters::{FilterRecursive, OrganizeFilter},
    FilterWalker,
};
use walkdir::DirEntry;

#[derive(Debug, Args, Clone)]
#[group(id = "location")]
pub struct LocationOpts {
    /// Locations to operate on
    #[arg(short, long, global = true)]
    locations: Vec<PathBuf>,

    #[command(flatten)]
    recursive: FilterRecursive,

    /// Targets to operate on
    #[arg(short, long, global = true, default_value_t = OrganizeTarget::Files, value_enum)]
    targets: OrganizeTarget,
}

/// `filter` subcommand
#[derive(Command, Debug, Args, Clone)]
pub struct FilterCmd {
    #[clap(subcommand)]
    filters: OrganizeFilter,

    /// Words in file names to be ignored
    #[arg(long, global = true)]
    ignore_name: Option<Vec<String>>,

    /// Words in Paths to be ignored
    #[arg(long, global = true)]
    ignore_path: Option<Vec<String>>,

    #[command(flatten)]
    location_opts: LocationOpts,
}

impl Runnable for FilterCmd {
    fn run(&self) {
        self.inner_run();
    }
}

impl FilterCmd {
    fn inner_run(&self) {
        println!("Filter chosen: {:?}", self.filters);

        // Convert to OrganizeLocation
        let mut locations = self.location_opts.locations.iter().map(|f| {
            if self.location_opts.recursive.recursive() {
                OrganizeLocation::from((
                    f.clone(),
                    MaxDepth::new(self.location_opts.recursive.max_depth()),
                    self.location_opts.targets,
                ))
            } else {
                OrganizeLocation::from((f.clone(), self.location_opts.targets))
            }
        });

        let base_entries = FilterWalker::get_applicable_items(&mut locations);

        let mut filters = vec![];

        if let Some(ignore_name) = self.ignore_name.clone() {
            println!("Ignore-Filter chosen: {ignore_name:?}");
            filters.push(
                OrganizeFilter::IgnoreName {
                    in_name: ignore_name,
                }
                .get_filter(),
            );
        };

        if let Some(ignore_path) = self.ignore_path.clone() {
            println!("Ignore-Filter chosen: {ignore_path:?}");
            filters.push(
                OrganizeFilter::IgnorePath {
                    in_path: ignore_path,
                }
                .get_filter(),
            );
        };

        filters.push(self.get_filter());

        let filtered_entries = FilterWalker::apply_filters(base_entries, filters);

        _ = filtered_entries
            .into_iter()
            .inspect(|dir_entry| {
                println!(
                    "{}\t\"{}\"",
                    if dir_entry.path().is_dir() {
                        "D"
                    } else if dir_entry.path().is_file() {
                        "F"
                    } else {
                        "L"
                    },
                    dir_entry.path().display()
                );
            })
            .collect_vec();
    }

    fn get_filter(&self) -> Box<dyn FnMut(&DirEntry) -> bool> {
        self.filters.get_filter()
    }
}
