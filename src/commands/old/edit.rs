//! `edit` subcommand

use crate::application::ORGANIZE_APP;
use crate::commands::CONFIG_FILE;
use abscissa_core::{Application, Command, Runnable};
use clap::Parser;
use duct::cmd;

/// Edit the rules.
///
/// If called without arguments it will open the default config file in $EDITOR.
#[derive(Command, Debug, Parser)]
pub struct EditCmd {
    /// editor used to edit the config file
    #[clap(long, env = "EDITOR")]
    pub editor: Option<String>,
}

impl Runnable for EditCmd {
    fn run(&self) {
        // let path = CONFIG_FILE.as_path();

        // let open_editor = self.editor.as_ref().map_or_else(
        //     || {
        //         #[cfg(target_os = "osx")]
        //         // Could probably be also something like `open -a TextEdit`
        //         let editor = "open".to_string();

        //         #[cfg(target_os = "windows")]
        //         let editor = "notepad".to_string();

        //         #[cfg(target_os = "unix")]
        //         let editor = "xdg-open".to_string();

        //         editor
        //     },
        //     std::clone::Clone::clone,
        // );

        // match cmd!(open_editor, path).run() {
        //     Ok(_) => {}
        //     Err(_) => {
        //         // println!("{err}")
        //     }
        // };
    }
}
