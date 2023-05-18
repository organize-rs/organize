//! Filters that organize can apply

use std::{borrow::BorrowMut, fs::FileType};

use abscissa_core::{Application, Command, Runnable};
use clap::Parser;
use itertools::Itertools;
use organize_rs_core::{
    error::OrganizeResult,
    rules::{filters::OrganizeFilter, OrganizeTargets},
    FilterWalker,
};
use walkdir::DirEntry;

use crate::application::ORGANIZE_APP;

/// `filter` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(Command, Debug, Parser)]
pub struct FilterCmd {
    #[clap(subcommand)]
    filters: OrganizeFilter,

    /// Locations to operate on
    #[clap(short, long, global = true)]
    locations: Vec<String>,

    /// Targets to operate on
    #[clap(short, long, global = true)]
    targets: Option<OrganizeTargets>,

    /// Recurse into subfolders
    #[clap(short, long, global = true)]
    recursive: bool,

    /// Maximal depth when operating recursively
    #[clap(short, long, global = true)]
    max_depth: Option<u64>,
}

impl Runnable for FilterCmd {
    fn run(&self) {
        println!("Filter chosen: {:?}", self.filters);
        let filter = self.filters.get_filter();

        let viable_locations = self
            .locations
            .iter()
            .map(|path| FilterWalker::entries(path, self.max_depth))
            .flatten_ok()
            .filter_map(std::result::Result::ok)
            .filter(|f| {
                let file_type = &f.file_type();
                match self.targets {
                    Some(OrganizeTargets::Dirs) => FileType::is_dir(file_type),
                    Some(OrganizeTargets::Files) => FileType::is_file(file_type),
                    Some(OrganizeTargets::Both) | None => true,
                }
            })
            .collect_vec();

        let filtered = viable_locations
            .into_iter()
            .filter(filter)
            .inspect(|dir_entry| println!("{}", dir_entry.path().display()))
            .collect_vec();
    }
}
