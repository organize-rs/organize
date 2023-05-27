//! Generate completions for your shell, and organize config and script files

mod gen_completions;
mod gen_config;

use abscissa_core::{Command, Runnable};
use clap::{Parser, Subcommand};

use crate::commands::generate::{gen_completions::GenCompletionsCmd, gen_config::GenConfigCmd};

#[derive(Subcommand, Command, Debug, Runnable)]
pub enum GenerateSubCmd {
    /// Generate a *.ron config stub
    Config(GenConfigCmd),
    /// Generate completions for your shell
    Completions(GenCompletionsCmd),
    // TODO: Generate a *.rhai script stub
    // Script,
}

/// Generate completions for your shell, and organize config and script files
#[derive(Command, Debug, Parser)]
pub struct GenerateCmd {
    #[clap(subcommand)]
    commands: GenerateSubCmd,
}

impl Runnable for GenerateCmd {
    /// Start the application.
    fn run(&self) {
        self.commands.run();
    }
}
