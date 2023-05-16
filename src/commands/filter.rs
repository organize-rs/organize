//! Filters that organize can apply

use abscissa_core::{Command, Runnable};
use clap::Parser;
use organize_rs_core::rules::filters::OrganizeFilter;

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

    /// Recurse into subfolders
    #[clap(short, long, global = true)]
    recursive: bool,

    /// Maximal depth when operating recursively
    #[clap(short, long, global = true)]
    max_depth: Option<u64>,
}

impl Runnable for FilterCmd {
    /// Start the application.
    fn run(&self) {
        // Your code goes here
    }
}
