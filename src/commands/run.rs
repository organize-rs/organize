//! `run` subcommand

use abscissa_core::{Command, Runnable};
use clap::Parser;

/// Organizes your files according to your rules.
#[derive(Command, Debug, Parser)]
pub struct RunCmd {
    /// working directory
    #[clap(long)]
    pub working_dir: String,
}

impl Runnable for RunCmd {
    /// Start the application.
    fn run(&self) {
        todo!("run organization.")
    }
}
