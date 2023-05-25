//! `run` subcommand

mod config;
mod script;

use abscissa_core::{Command, Runnable};

use clap::{Parser, Subcommand};

use crate::commands::run::{config::RunConfigCmd, script::RunScriptCmd};

#[derive(Subcommand, Command, Debug, Runnable)]
pub enum RunSubCmd {
    /// Run a *.ron config with organize
    Config(RunConfigCmd),
    /// Run a *.rhai script with organize
    Script(RunScriptCmd),
}

/// `run` subcommand
#[derive(Command, Debug, Parser)]
pub struct RunCmd {
    #[clap(subcommand)]
    commands: RunSubCmd,

    /// flag to apply actions destructively
    #[clap(long)]
    execute: bool,
}

impl Runnable for RunCmd {
    fn run(&self) {
        self.commands.run();
    }
}
