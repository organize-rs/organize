//! Filters that organize can apply

use std::ops::Deref;

use abscissa_core::{Application, Command, Runnable};
use clap::Parser;
use organize_rs_core::{
    filter_walker,
    rules::{filters::OrganizeFilter, OrganizeTargets},
};

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

        let mut filtered = vec![];

        self.locations.into_iter().for_each(|path| {
            filtered.push(filter_walker(path, filter.deref(), self.max_depth));
        });

        filtered
            .into_iter()
            .flat_map(|f| f)
            .for_each(|dir_entry| println!("{dir_entry:?}"))
    }
}
