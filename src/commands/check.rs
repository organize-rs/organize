//! `check` subcommand

use abscissa_core::{Application, Command, Runnable};
use clap::Parser;

use crate::application::ORGANIZE_APP;

/// Checks whether a given config file is valid.
///
/// If called without arguments it will check the default config file.
#[derive(Command, Debug, Parser)]
pub struct CheckCmd {
    /// Print the parsed config to the terminal
    #[clap(long)]
    show_config: bool,
}

impl Runnable for CheckCmd {
    fn run(&self) {
        if self.show_config {
            println!("{:#?}", ORGANIZE_APP.config());
        } else {
            todo!("check validity of config file.")
        }
    }
}
