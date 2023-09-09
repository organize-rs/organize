//! `RunConfig` Subcommand

use std::path::PathBuf;

use abscissa_core::{status_err, Application, Command, Runnable};

use anyhow::Result;
use clap::Args;
use dialoguer::Confirm;
use organize_rs_core::{runner::Runner, state::Initialize, tags::Tag};

use crate::application::ORGANIZE_APP;

/// Run rules from a config file with organize
#[derive(Command, Debug, Args)]
pub struct RunConfigCmd {
    /// paths to compatible config files containing organize rules
    #[arg(long)]
    paths: Vec<PathBuf>,

    /// run rules from the config file that contain this tag(s)
    #[arg(long)]
    tags: Vec<Tag>,
}

impl RunConfigCmd {
    fn inner_run(&self) -> Result<()> {
        let runner = Runner::<Initialize>::load_configs(&self.paths)
            .apply_filters(self.tags.clone())
            .inspect_entries()
            .finish_inspection()
            .preview_actions()?;

        if Confirm::new()
        .with_prompt("Are you sure, that you want to execute the previewed actions? This is irreversible.")
        .default(false)
        .interact()? {

            runner.apply_actions()?;
            // ? Conflict handling
            // * Probably done in a loop until all the conflicts are handled
            // * loop can be interrupted
            // * should jump to report of actions
            // loop {
            //      runner
            //      .check_conflicts()
            //      .view_conflicts()
            //      .preview_actions()
            //      .ask_confirmation()?
            //      .apply_actions()?
            // }
        }

        // runner.print_report();
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
