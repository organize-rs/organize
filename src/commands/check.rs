//! `check` subcommand

use abscissa_core::{Command, Runnable};
use clap::Parser;

/// Checks whether a given config file is valid.
///
/// If called without arguments it will check the default config file.
#[derive(Command, Debug, Parser)]
pub struct CheckCmd {}

impl Runnable for CheckCmd {
    /// Start the application.
    fn run(&self) {
        todo!("check validity of config file.")
    }
}
