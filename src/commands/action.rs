//! Actions that organize can apply

use abscissa_core::{Command, Runnable};
use clap::Args;
use organize_rs_core::rules::actions::OrganizeAction;

/// `action` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(Command, Debug, Args)]
pub struct ActionCmd {
    #[clap(subcommand)]
    actions: OrganizeAction,

    /// Run an action destructively
    #[arg(short, long, global = true, default_value_t = false)]
    destructive_run: bool,
}

impl Runnable for ActionCmd {
    fn run(&self) {
        self.actions.run();
    }
}
