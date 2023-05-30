//! `RunConfig` Subcommand

use std::path::PathBuf;

use abscissa_core::{status_err, Application, Command, Runnable};

use anyhow::Result;
use clap::Parser;
use organize_rs_core::{config::OrganizeConfig, runner::Runner, state::Init};

use crate::application::ORGANIZE_APP;

/// Run a *.ron config with organize
#[derive(Command, Debug, Parser)]
pub struct RunConfigCmd {
    /// path to a compatible config file containing organize rules
    path: PathBuf,
}

impl RunConfigCmd {
    fn inner_run(&self) -> Result<()> {
        let config = OrganizeConfig::load_from_file(&self.path);

        let runner = Runner::<Init>::load(config);

        Ok(())
    }
}

impl Runnable for RunConfigCmd {
    fn run(&self) {
        match self.inner_run() {
            Ok(_) => {}
            Err(err) => {
                status_err!("{}", err);
                ORGANIZE_APP.shutdown(abscissa_core::Shutdown::Crash);
            }
        }
    }
}
