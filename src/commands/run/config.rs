//! `RunConfig` Subcommand

use std::{path::PathBuf};

use abscissa_core::{Command, Runnable};

use clap::{Parser};







/// Run a *.ron config with organize
#[derive(Command, Debug, Parser)]
pub struct RunConfigCmd {
    /// path to a compatible config file containing organize rules
    config_path: PathBuf,
}

impl Runnable for RunConfigCmd {
    fn run(&self) {
        todo!()
    }
}
