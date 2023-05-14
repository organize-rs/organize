//! `start` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::OrganizeConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};

/// Reveals the default config file.
#[derive(clap::Parser, Command, Debug)]
pub struct RevealCmd {
    /// show the full path to the default config
    #[clap(long)]
    pub path: bool,
}

impl Runnable for RevealCmd {
    /// Start the application.
    fn run(&self) {
        let config = ORGANIZE_APP.config();
    }
}

impl config::Override<OrganizeConfig> for RevealCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        mut config: OrganizeConfig,
    ) -> Result<OrganizeConfig, FrameworkError> {
        Ok(config)
    }
}
