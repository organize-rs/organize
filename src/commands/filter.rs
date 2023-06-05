//! Filters that organize can apply

use std::path::PathBuf;

use abscissa_core::{Command, Runnable};
use clap::Args;

use organize_rs_core::{
    filters::{
        FilterApplicationKind, FilterGroup, FilterGroupCollection, FilterGroupOperationKind,
        FilterKind, RecursiveFilterArgs,
    },
    locations::{TargetKind},
};

/// `filter` subcommand
#[derive(Command, Debug, Args, Clone)]
pub struct FilterCmd {
    #[clap(subcommand)]
    filter: FilterKind,

    /// Words in file names to be ignored
    #[arg(long, global = true)]
    ignore_name: Option<Vec<String>>,

    /// Words in Paths to be ignored
    #[arg(long, global = true)]
    ignore_path: Option<Vec<String>>,

    /// Mode, how the filters should apply
    #[arg(short, long, global = true, default_value_t = FilterApplicationKind::Any, value_enum)]
    filter_mode: FilterApplicationKind,

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
            FilterGroupOperationKind::Include,
            self.filter_mode,
            vec![self.filter.clone()],
        );

        let mut filters = vec![];

        if let Some(ignore_names) = self.ignore_name.clone() {
            if !ignore_names.is_empty() {
                filters.push(FilterKind::IgnoreName {
                    in_name: ignore_names,
                });
            }
        };

        if let Some(ignore_paths) = self.ignore_path.clone() {
            if !ignore_paths.is_empty() {
                filters.push(FilterKind::IgnorePath {
                    in_path: ignore_paths,
                });
            }
        };

        let mut filter_group = vec![filter_group0];

        if !filters.is_empty() {
            filter_group.push(FilterGroup::new(
                FilterGroupOperationKind::Include,
                FilterApplicationKind::None,
                filters,
            ));
        };

        let filter_group_collection = FilterGroupCollection::from_vec(filter_group);

        Self::inner_run(&filter_group_collection);
    }
}

impl FilterCmd {
    fn inner_run(filters: &FilterGroupCollection) {
        // TODO: Support different config file formats
        // ? implement `FilterKind::to_config_string(format: ConfigFileFormat)`
        let yaml_string = serde_yaml::to_string(&filters).unwrap();
        println!("This is a 'filter' snippet for a yaml config:");
        println!("'''");
        println!("{yaml_string}");
        println!("'''");
    }
}
