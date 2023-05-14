//! `sim` subcommand

use abscissa_core::{Command, Runnable};
use clap::Parser;

/// Simulates a run (does not touch your files).
#[derive(Command, Debug, Parser)]
pub struct SimCmd {
    /// working directory
    #[clap(long)]
    pub working_dir: String,
}

impl Runnable for SimCmd {
    /// Start the application.
    fn run(&self) {
        // Your code goes here
    }
}
