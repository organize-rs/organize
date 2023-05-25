//! `config` subcommand

use std::{path::PathBuf};

use abscissa_core::{Command, Runnable};

use clap::{Parser, Subcommand};






#[derive(Debug, Subcommand)]
pub enum CheckSubCmd {
    /// Check given *.ron config file for errors
    Config,
    /// Check given *.rhai script file for errors
    Script,
}

/// `config` subcommand
#[derive(Command, Debug, Parser)]
pub struct CheckCmd {
    #[clap(subcommand)]
    commands: CheckSubCmd,

    #[clap(short, long, global = true)]
    path: PathBuf,
}

impl Runnable for CheckCmd {
    /// Start the application.
    fn run(&self) {
        todo!()
    }
}

impl CheckCmd {}
