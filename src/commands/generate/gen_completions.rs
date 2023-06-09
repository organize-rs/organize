//! `completions` subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use abscissa_core::{Command, Runnable};

use std::io::Write;

use clap::{Args, CommandFactory, ValueEnum};

use clap_complete::{generate, shells, Generator};

/// `completions` subcommand
#[derive(Args, Command, Debug, Clone)]
pub struct GenCompletionsCmd {
    /// Shell to generate completions for
    #[arg(value_enum)]
    shell: Variant,
    // TODO: does it make sense to also write
    // TODO  the completions to a file, mostly
    // TODO  for usage in integration tests
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Variant {
    Bash,
    Fish,
    Powershell,
    Zsh,
}

impl Runnable for GenCompletionsCmd {
    fn run(&self) {
        match self.shell {
            Variant::Bash => generate_completion(shells::Bash, &mut std::io::stdout()),
            Variant::Fish => generate_completion(shells::Fish, &mut std::io::stdout()),
            Variant::Zsh => generate_completion(shells::Zsh, &mut std::io::stdout()),
            Variant::Powershell => generate_completion(shells::PowerShell, &mut std::io::stdout()),
        }
    }
}

pub fn generate_completion<G: Generator>(shell: G, buf: &mut dyn Write) {
    let mut command = crate::commands::EntryPoint::command();
    generate(
        shell,
        &mut command,
        option_env!("CARGO_BIN_NAME").unwrap_or("organize"),
        buf,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completions() {
        generate_completion(shells::Bash, &mut std::io::sink());
        generate_completion(shells::Fish, &mut std::io::sink());
        generate_completion(shells::Zsh, &mut std::io::sink());
    }
}
