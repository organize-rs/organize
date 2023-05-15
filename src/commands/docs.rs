//! `docs` subcommand

use abscissa_core::{Command, Runnable};
use clap::Parser;

/// Opens the documentation.
#[derive(Command, Debug, Parser)]
pub struct DocsCmd {}

impl Runnable for DocsCmd {
    fn run(&self) {
        println!("Please open https://docs.rs/organize-rs in your browser.");
    }
}
