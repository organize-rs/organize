//! `run` subcommand

use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use abscissa_core::{status_err, Application, Command, Runnable, Shutdown};
use clap::Parser;
use rhai::{Engine, EvalAltResult};

use crate::{application::ORGANIZE_APP, commands::filter::FilterCmd, scripting::add};

/// Run a *.rhai script with organize
#[derive(Command, Debug, Parser)]
pub struct RunScriptCmd {
    /// Path to the *.rhai script that should be run
    #[clap(long)]
    path: PathBuf,
}

impl Runnable for RunScriptCmd {
    /// Start the application.
    fn run(&self) {
        match start_scripting_engine(&self.path) {
            Ok(_) => {}
            Err(err) => {
                status_err!("failed to execute script: {}", err);
                ORGANIZE_APP.shutdown(Shutdown::Crash)
            }
        };
    }
}

fn start_scripting_engine(path: impl Into<PathBuf>) -> Result<(), Box<EvalAltResult>> {
    // Create an 'Engine'
    let mut engine = Engine::new();

    engine.register_fn("add", add);

    // engine.build_type::<FilterCmd>();

    // Run the script
    engine.run_file(path.into())?;

    // Done!
    Ok(())
}
