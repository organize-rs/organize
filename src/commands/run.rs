//! `run` subcommand

mod config;
mod script;



use abscissa_core::{status_err, Application, Command, Runnable, Shutdown};
use anyhow::Result;
use clap::{Parser, Subcommand};





use crate::{
    application::ORGANIZE_APP,
    commands::run::{config::RunConfigCmd, script::RunScriptCmd},
};

#[derive(Debug, Subcommand)]
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
        match self.inner_run() {
            Ok(_) => {}
            Err(err) => {
                status_err!("{}", err);
                ORGANIZE_APP.shutdown(Shutdown::Crash)
            }
        };
    }
}

impl RunCmd {
    fn inner_run(&self) -> Result<()> {
        todo!()
    }
}
