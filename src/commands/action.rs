//! Actions that organize can apply

use abscissa_core::{Command, Runnable};
use clap::Args;
use organize_rs_core::actions::ActionKind;

/// `action` subcommand
#[derive(Command, Debug, Args, Clone)]
pub struct ActionCmd {
    #[clap(subcommand)]
    actions: ActionKind,

    /// Run an action destructively
    #[arg(short, long, global = true, default_value_t = false)]
    execute: bool,
}

impl Runnable for ActionCmd {
    fn run(&self) {
        println!("Action chosen: {}", self.actions);
        // let _action = self.actions.get_action();
    }
}
