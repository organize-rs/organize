//! `docs` subcommand

use abscissa_core::{Command, Runnable};
use clap::Parser;

/// Opens the documentation.
#[derive(Command, Debug, Parser)]
pub struct DocsCmd {}

impl Runnable for DocsCmd {
    /// Start the application.
    fn run(&self) {
        // Your code goes here
    }
}
