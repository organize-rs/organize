//! `config` subcommand

use std::{fs::read_to_string, path::PathBuf};

use abscissa_core::{Command, Runnable};

use clap::{Args, Parser, Subcommand};
use organize_rs_core::config::OrganizeConfig;
use serde::Deserialize;

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
        let reader =
            read_to_string(self.path.as_path()).expect("reading the file contents should succeed.");
        let data = ron::de::from_str::<OrganizeConfig>(reader.as_str())
            .expect("parsing ron value should succeed.");

        // println!("{config:?}");
        todo!()
    }
}

impl Runnable for CheckScriptCmd {
    fn run(&self) {
        todo!()
    }
}
