//! `RunConfig` Subcommand

use std::path::PathBuf;

use abscissa_core::{status_err, Application, Command, Runnable};

use anyhow::Result;
use clap::Args;
use organize_rs_core::{runner::Runner, state::Init, tags::Tag};

use crate::application::ORGANIZE_APP;

/// Run rules from a config file with organize
#[derive(Command, Debug, Args)]
pub struct RunConfigCmd {
    /// path to a compatible config file containing organize rules
    #[arg(long)]
    path: PathBuf,

    /// run rules from the config file that contain this tag(s)
    #[arg(long)]
    tags: Vec<Tag>,
}

impl RunConfigCmd {
    fn inner_run(&self) -> Result<()> {
        let runner = Runner::<Init>::load_config(&self.path)
            .apply_filters(self.tags.clone())
            .preview_entries();
        // ? Check how we can make state transitions better
        // ? https://www.novatec-gmbh.de/en/blog/the-case-for-the-typestate-pattern-the-typestate-pattern-itself/
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
