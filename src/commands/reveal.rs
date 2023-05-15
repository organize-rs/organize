//! `start` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::commands::CONFIG_FILE;
use crate::config::OrganizeConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};
use duct::cmd;

/// Reveals the default config file.
#[derive(clap::Parser, Command, Debug)]
pub struct RevealCmd {
    /// show the full path to the default config
    #[clap(long)]
    pub path: bool,
}

impl Runnable for RevealCmd {
    fn run(&self) {
        if self.path {
            println!("{}", CONFIG_FILE.display());
        } else {
            let path = CONFIG_FILE
                .parent()
                .expect("can't get parent for config file path!");

            #[cfg(windows)]
            match cmd!("explorer", path).run() {
                Ok(_) => {}
                Err(_) => {
                    // TODO: handle exit(1) on Windows
                    // println!("{err}")
                }
            };

            #[cfg(osx)]
            match cmd!("open", path).run() {
                Ok(_) => {}
                Err(_) => {
                    // TODO: osx support
                    // println!("{err}")
                }
            };

            #[cfg(unix)]
            match cmd!("xdg-open", path).run() {
                Ok(_) => {}
                Err(_) => {
                    // TODO: unix support
                    // println!("{err}")
                }
            };
        }
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
