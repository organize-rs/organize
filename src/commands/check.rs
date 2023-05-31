//! `config` subcommand

use std::path::PathBuf;

use abscissa_core::{Command, Runnable};

use clap::{Args, Parser, Subcommand};
use organize_rs_core::config::OrganizeConfig;

#[derive(Command, Debug, Args, Clone)]
pub struct CheckConfigCmd {
    /// Path the *.ron config to be checked
    #[clap(short, long)]
    path: PathBuf,
}

#[derive(Debug, Args, Clone)]
pub struct CheckScriptCmd {
    /// Path the *.rhai script to be checked
    #[clap(short, long)]
    path: PathBuf,
}

#[derive(Subcommand, Command, Debug, Runnable)]
pub enum CheckSubCmd {
    /// Check given *.ron config file for errors
    Config(CheckConfigCmd),
    /// Check given *.rhai script file for errors
    Script(CheckScriptCmd),
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
        self.commands.run();
    }
}

impl Runnable for CheckConfigCmd {
    fn run(&self) {
        let config = OrganizeConfig::load_from_file(&self.path);
        println!("{config}");
    }
}

impl Runnable for CheckScriptCmd {
    fn run(&self) {
        todo!()
    }
}
