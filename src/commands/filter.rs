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

/// `filter` subcommand
#[derive(Command, Debug, Args, Clone)]
pub struct FilterCmd {
    #[clap(subcommand)]
    filters: OrganizeFilter,

    /// Locations to operate on
    #[arg(short, long, global = true)]
    locations: Vec<PathBuf>,

    /// Words in file names to be ignored
    #[arg(long, global = true)]
    ignore_name: Option<Vec<String>>,

    /// Words in Paths to be ignored
    #[arg(long, global = true)]
    ignore_path: Option<Vec<String>>,

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
    recursive: FilterRecursive,

    /// Targets to operate on
    #[arg(short, long, global = true, default_value_t = OrganizeTarget::Files, value_enum)]
    targets: OrganizeTarget,
}

impl Runnable for FilterCmd {
    fn run(&self) {
        println!("Filter chosen: {:?}", self.filters);

        let mut filters = vec![self.filters.clone()];

        if let Some(ignore_names) = self.ignore_name.clone() {
            println!("Ignore-Filter chosen: {ignore_names:?}");
            filters.push(OrganizeFilter::IgnoreName {
                in_name: ignore_names,
            });
        };

        if let Some(ignore_paths) = self.ignore_path.clone() {
            println!("Ignore-Filter chosen: {ignore_paths:?}");
            filters.push(OrganizeFilter::IgnorePath {
                in_path: ignore_paths,
            });
        };

        self.inner_run(filters);
    }
}

impl FilterCmd {
    fn inner_run(&self, filters: Vec<OrganizeFilter>) {
        let mut filter_walker = FilterWalker::new();

        // Convert to OrganizeLocation
        let locations = self
            .location_opts
            .locations
            .clone()
            .into_iter()
            .map(|f| {
                if self.location_opts.recursive.recursive() {
                    OrganizeLocation::from((
                        f,
                        MaxDepth::new(self.location_opts.recursive.max_depth()),
                        self.location_opts.targets,
                    ))
                } else {
                    OrganizeLocation::from((f, self.location_opts.targets))
                }
            })
            .collect_vec();

        filter_walker.get_applicable_items(locations, filters);

        _ = filter_walker
            .entries()
            .iter()
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
}
