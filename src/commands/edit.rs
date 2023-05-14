//! `edit` subcommand

use abscissa_core::{Command, Runnable};
use clap::Parser;

/// Edit the rules.
///
/// If called without arguments it will open the default config file in $EDITOR.
#[derive(Command, Debug, Parser)]
pub struct EditCmd {
    /// editor used to edit the config file
    #[clap(long, env = "EDITOR")]
    pub editor: Option<String>,
}

impl Runnable for EditCmd {
    /// Start the application.
    fn run(&self) {
        todo!("opening config file in editor.")
    }
}
