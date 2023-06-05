//! Actions that organize can apply

use abscissa_core::{Command, Runnable};
use clap::Args;
use organize_rs_core::actions::ActionKind;

/// `action` subcommand
#[derive(Command, Debug, Args, Clone)]
pub struct ActionCmd {
    #[clap(subcommand)]
    action: ActionKind,
}

impl Runnable for ActionCmd {
    fn run(&self) {
        // TODO: Support different config file formats
        // ? implement `ActionKind::to_config_string(format: ConfigFileFormat)`
        let yaml_string = serde_yaml::to_string(&self.action).unwrap();
        println!("This is an 'action' snippet for a yaml config:");
        println!("'''");
        println!("{yaml_string}");
        println!("'''");
    }
}
