//! `rule` subcommand

use std::{fs::File, path::PathBuf, str::FromStr};

use abscissa_core::{status_err, Application, Command, Runnable, Shutdown};
use anyhow::Result;
use clap::{Args, Parser};
use organize_rs_core::{
    locations::{LocationKind, MaxDepth, TargetKind},
    parsers::SizeRange,
    rules::{
        actions::{ActionApplicationKind, ActionKind},
        filters::{FilterApplicationKind, FilterKind, FilterModeGroupKind},
        Rule, Tag,
    },
};
use ron::ser::PrettyConfig;

use crate::application::ORGANIZE_APP;

#[derive(Debug, Args, Clone)]
#[group(id = "location", required = true, multiple = false)]
pub struct RunLocationArgs {
    /// path to a compatible config file containing organize rules
    config_path: Option<PathBuf>,
    /// path to a *.rhai script file containing organize rules
    script_path: Option<PathBuf>,
}

/// `run` subcommand
#[derive(Command, Debug, Parser)]
pub struct RunCmd {
    /// flag to apply actions destructively
    #[clap(long)]
    execute: bool,

    #[command(flatten)]
    location_opts: RunLocationArgs,
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
