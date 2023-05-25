//! `docs` subcommand

use abscissa_core::{status_err, Application, Command, Runnable, Shutdown};
use clap::Args;

use crate::application::ORGANIZE_APP;

/// Opens the documentation.
#[derive(Command, Debug, Args)]
pub struct DocsCmd {}

impl Runnable for DocsCmd {
    fn run(&self) {
        match open::that("https://docs.rs/organize-rs") {
            Ok(_) => {}
            Err(err) => {
                status_err!("{}", err);
                ORGANIZE_APP.shutdown(Shutdown::Crash);
            }
        };
    }
}
