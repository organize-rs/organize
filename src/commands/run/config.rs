//! `RunConfig` Subcommand

use std::path::PathBuf;

use abscissa_core::{status_err, Application, Command, Runnable};

use anyhow::Result;
use clap::{Args, Parser};
use organize_rs_core::{config::OrganizeConfig, runner::Runner, state::Init};

use crate::application::ORGANIZE_APP;

/// Run a *.ron config with organize
#[derive(Command, Debug, Args)]
pub struct RunConfigCmd {
    /// path to a compatible config file containing organize rules
    #[arg(long)]
    path: PathBuf,
}

impl RunConfigCmd {
    fn inner_run(&self) -> Result<()> {
        let runner = Runner::<Init>::load_config(&self.path);
        let runner = runner.apply_filters();
        runner.preview_entries();
        // let runner = runner.advance();
        // let runner: Runner<AskConfirmation> = runner.get_confirmation();
        // let runner: Runner<ApplyActions> = runner.apply_actions();
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
