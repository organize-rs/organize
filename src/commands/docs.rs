//! `docs` subcommand

use abscissa_core::{Command, Runnable};
use clap::Args;

/// Opens the documentation.
#[derive(Command, Debug, Args, Clone)]
pub struct DocsCmd {}

impl Runnable for DocsCmd {
    fn run(&self) {
        println!("Please open https://docs.rs/organize-rs in your browser.");
    }
}
