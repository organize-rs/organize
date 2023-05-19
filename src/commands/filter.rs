//! Filters that organize can apply

use std::fs::FileType;

use abscissa_core::{Command, Runnable};
use clap::Args;
use itertools::Itertools;
use organize_rs_core::{
    rules::{
        filters::{FilterRecursive, OrganizeFilter},
        OrganizeTargets,
    },
    FilterWalker,
};

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

    /// Words in file names to be ignored
    #[arg(long, global = true)]
    ignore_name: Option<Vec<String>>,

    /// Words in Paths to be ignored
    #[arg(long, global = true)]
    ignore_path: Option<Vec<String>>,

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
            .filter(|f| {
                self.ignore_name.as_ref().map_or(true, |ignore| {
                    let file_name = &f
                        .file_name()
                        .to_str()
                        .expect("filename should be convertable to a String")
                        .to_string();
                    !ignore.iter().any(|pat| file_name.contains(pat))
                })
            })
            .filter(|f| {
                self.ignore_path.as_ref().map_or(true, |ignore| {
                    let file_name = &f
                        .path()
                        .to_str()
                        .expect("filename should be convertable to a String")
                        .to_string();
                    !ignore.iter().any(|pat| file_name.contains(pat))
                })
            })
            .collect_vec();

        let _filtered = viable_locations
            .into_iter()
            .filter(filter)
            .inspect(|dir_entry| println!("{}", dir_entry.path().display()))
            .collect_vec();
    }
}
