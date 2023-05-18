//! Filters that organize can apply

use std::{borrow::BorrowMut, fs::FileType};

use abscissa_core::{Application, Command, Runnable};
use clap::{Args, Parser};
use itertools::Itertools;
use organize_rs_core::{
    error::OrganizeResult,
    rules::{
        filters::{FilterRecursive, OrganizeFilter},
        OrganizeTargets,
    },
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
#[derive(Command, Debug, Args)]
pub struct FilterCmd {
    #[clap(subcommand)]
    filters: OrganizeFilter,

    /// Locations to operate on
    #[arg(short, long, global = true)]
    locations: Vec<String>,

    /// Targets to operate on
    #[arg(short, long, global = true, default_value_t = OrganizeTargets::Files, value_enum)]
    targets: OrganizeTargets,

    #[command(flatten)]
    recursive: FilterRecursive,
}

impl Runnable for FilterCmd {
    fn run(&self) {
        println!("Filter chosen: {:?}", self.filters);
        let filter = self.filters.get_filter();

        let viable_locations = self
            .locations
            .iter()
            .map(|path| FilterWalker::entries(path, self.recursive.max_depth()))
            .flatten_ok()
            .filter_map(std::result::Result::ok)
            .filter(|f| {
                let file_type = &f.file_type();
                match self.targets {
                    OrganizeTargets::Dirs => FileType::is_dir(file_type),
                    OrganizeTargets::Files => FileType::is_file(file_type),
                    OrganizeTargets::Both => true,
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