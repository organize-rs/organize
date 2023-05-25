//! `config` subcommand

use std::path::PathBuf;

use abscissa_core::{Command, Runnable};

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Args, Clone)]
pub struct CheckConfigOpts {
    /// Path the *.ron config to be checked
    #[clap(short, long)]
    path: PathBuf,
}

#[derive(Debug, Args, Clone)]
pub struct CheckScriptOpts {
    /// Path the *.rhai script to be checked
    #[clap(short, long)]
    path: PathBuf,
}

#[derive(Debug, Subcommand, Clone)]
pub enum CheckSubCmd {
    /// Check given *.ron config file for errors
    Config(CheckConfigOpts),
    /// Check given *.rhai script file for errors
    Script(CheckScriptOpts),
}

/// `config` subcommand
#[derive(Command, Debug, Parser)]
pub struct CheckCmd {
    #[clap(subcommand)]
    commands: CheckSubCmd,
}

impl Runnable for CheckCmd {
    /// Start the application.
    fn run(&self) {
        todo!()
    }
}

impl CheckCmd {}
