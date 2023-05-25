//! `config` subcommand

use std::{fs::File, io::Write, path::PathBuf, str::FromStr};

use abscissa_core::{status_err, Application, Command, Runnable, Shutdown};
use anyhow::{bail, Result};
use clap::Parser;
use dialoguer::Confirm;
use organize_rs_core::{
    locations::{LocationKind, MaxDepth, TargetKind},
    parsers::SizeRange,
    rules::{
        actions::{ActionApplicationKind, ActionKind},
        filters::{FilterApplicationKind, FilterKind, FilterModeGroupKind},
        tags::Tag,
        Rule,
    },
};
use ron::ser::PrettyConfig;

use crate::application::ORGANIZE_APP;

#[derive(Command, Debug, Parser, Clone)]
pub struct GenConfigCmd {
    /// generate a config interactively
    #[clap(short, long)]
    interactive: bool,

    /// path to an existing or to be created config file
    #[clap(short, long)]
    config_path: PathBuf,
}

impl Runnable for GenConfigCmd {
    /// Start the application.
    fn run(&self) {
        match self.generate_example_config() {
            Ok(_) => {}
            Err(err) => {
                status_err!("{}", err);
                ORGANIZE_APP.shutdown(Shutdown::Crash)
            }
        }
    }
}

impl GenConfigCmd {
    fn generate_example_config(&self) -> Result<()> {
        let rule_builder = Rule::builder();

        let rule = rule_builder
            .name("test")
            .filter(FilterApplicationKind::Apply(FilterKind::Extension {
                exts: vec!["toml".to_string()],
            }))
            .filter(FilterApplicationKind::Apply(FilterKind::Size {
                range: SizeRange::from_str("1KiB..")?,
            }))
            .filter_mode(FilterModeGroupKind::All)
            .action(ActionApplicationKind::Preview(ActionKind::Trash))
            .location(LocationKind::RecursiveWithMaxDepth {
                path: r"C:\Users\dailyuse\dev-src\organize".into(),
                target: TargetKind::Files,
                max_depth: MaxDepth::new(10),
            })
            .tag(Tag::Custom("my_test_rule".to_string()))
            .build();

        if File::open(&self.config_path).is_ok() {
            if Confirm::new().with_prompt("Config file already exists. We will overwrite it, do you have a backup and want to continue?").interact()? {
                let file = File::create(&self.config_path)?;
                ron::ser::to_writer_pretty(file, &rule, PrettyConfig::default())?;
            } else {
                bail!("Config file already exists. We will overwrite it, make sure you have a backup and agree in the dialog.");
            }
        } else {
            let mut file = File::create(&self.config_path)?;
            // ron::ser::to_writer_pretty(file, &rule, PrettyConfig::default())?;
            // file.write_all(toml::to_string_pretty(&rule)?.as_bytes())?;
            file.write_all(serde_yaml::to_string(&rule)?.as_bytes())?;
        };

        Ok(())
    }
}
