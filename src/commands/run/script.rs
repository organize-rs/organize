//! `RunScript` Subcommand

use std::path::PathBuf;

use abscissa_core::{status_err, Application, Command, Runnable, Shutdown};
use anyhow::Result;
use clap::Parser;

use rhai::{Engine, EvalAltResult};

use crate::application::ORGANIZE_APP;
use crate::scripting::add;

/// Run a *.rhai script with organize
#[derive(Command, Debug, Parser)]
pub struct RunScriptCmd {
    /// path to a *.rhai script file containing organize rules
    script_path: PathBuf,
}

impl Runnable for RunScriptCmd {
    fn run(&self) {
        match start_scripting_engine(&self.script_path) {
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

    engine.run_file(path.into())?;

    // Done!
    Ok(())
}
