//! `schema` subcommand

use abscissa_core::{Command, Runnable};
use clap::Parser;

/// Prints the json schema for config files.
#[derive(Command, Debug, Parser)]
pub struct SchemaCmd {}

impl Runnable for SchemaCmd {
    /// Start the application.
    fn run(&self) {
        // Your code goes here
    }
}
