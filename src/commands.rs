//! Organize Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `start`: launches the application
//! - `--version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

mod action;
mod docs;
mod filter;

// Old approach
// mod edit;
// mod reveal;
// mod run;
// mod schema;
// mod sim;

use crate::{
    commands::{action::ActionCmd, docs::DocsCmd, filter::FilterCmd},
    config::OrganizeConfig,
};
use abscissa_core::{config::Override, Command, Configurable, FrameworkError, Runnable};
use clap;
use directories::BaseDirs;
use once_cell::sync::Lazy;
use organize_rs_core::rules::{OrganizeLocation, OrganizeTargets};
use std::{fs::create_dir_all, path::PathBuf};

/// Organize Configuration Filename
// pub static CONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| {
//     let dir = if let Some(base_dir) = BaseDirs::new() {
//         let organize_config_dir = base_dir.config_local_dir().join("organize");
//         _ = create_dir_all(&organize_config_dir);
//         organize_config_dir.join("organize.yaml")
//     } else {
//         todo!("alternative default config file locations")
//     };
//     dir
// });

/// Organize Configuration Filename
pub const CONFIG_FILE: &str = "organize.toml";

/// Organize Subcommands
/// Subcommands need to be listed in an enum.
#[derive(clap::Parser, Command, Debug, Runnable)]
pub enum OrganizeCmd {
    /// Actions that organize can apply
    Action(ActionCmd),
    /// Show the documentation
    Docs(DocsCmd),
    /// Filters that organize can apply
    Filter(FilterCmd),
}

#[allow(clippy::struct_excessive_bools)]
/// Entry point for the application. It needs to be a struct to allow using subcommands!
#[derive(clap::Parser, Command, Debug)]
#[command(author, about, version)]
pub struct EntryPoint {
    #[command(subcommand)]
    cmd: OrganizeCmd,

    /// Targets to operate on
    #[clap(short, long, global = true)]
    targets: Option<OrganizeTargets>,

    /// Locations to operate on
    #[clap(short, long, global = true)]
    locations: Vec<String>,
    // /// Enable verbose logging
    // #[clap(short, long)]
    // pub verbose: bool,

    // /// Enable debug mode
    // #[clap(short, long)]
    // pub debug: bool,

    // /// Use the specified organize-rs config file
    // #[clap(short, long)]
    // pub config: Option<String>,
    // /// path to the py-organize config file
    // #[clap(long, env = "ORGANIZE_CONFIG")]
    // pub py_organize_config: Option<String>,

    // /// Applicable tags
    // ///
    // /// Rules tagged with the special tag always will always run (except if --skip-tags=always is specified)
    // ///
    // /// Rules tagged with the special tag never will never run (except if ' --tags=never is specified)
    // #[clap(long)]
    // pub tags: Option<Vec<String>>,

    // /// Skip-Tags
    // #[clap(long)]
    // pub skip_tags: Option<Vec<String>>,

    // /// if this is set, the output is not colored
    // #[clap(long, env = "NO_COLOR")]
    // pub no_color: bool,
}

impl Runnable for EntryPoint {
    fn run(&self) {
        self.cmd.run();
    }
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<OrganizeConfig> for EntryPoint {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.
        // let filename = self
        //     .config
        //     .as_ref()
        //     .map(PathBuf::from)
        //     .unwrap_or_else(|| CONFIG_FILE.into());

        // if filename.exists() {
        //     Some(filename)
        // } else {
        //     None
        // }
        None
    }

    /// Apply changes to the config after it's been loaded, e.g. overriding
    /// values in a config file using command-line options.
    ///
    /// This can be safely deleted if you don't want to override config
    /// settings from command-line options.
    fn process_config(&self, config: OrganizeConfig) -> Result<OrganizeConfig, FrameworkError> {
        // match &self.cmd {
        // OrganizeCmd::Reveal(cmd) => cmd.override_config(config),
        //
        // If you don't need special overrides for some
        // subcommands, you can just use a catch all
        // _ => Ok(config),
        // }
        Ok(config)
    }
}
