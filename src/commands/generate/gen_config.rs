//! `gen_config` subcommand

use std::{fs::File, io::Write, path::PathBuf, str::FromStr};

use abscissa_core::{status_err, Application, Command, Runnable, Shutdown};
use anyhow::{bail, Result};
use clap::{Args, Parser};
use dialoguer::Confirm;
use organize_rs_core::{
    actions::{ActionApplicationKind, ActionKind},
    config::OrganizeConfig,
    filters::{FilterApplicationKind, FilterKind, FilterModeGroupKind},
    locations::{LocationKind, MaxDepth, TargetKind},
    parsers::SizeRange,
    rules::Rule,
    tags::Tag,
};
use ron::ser::PrettyConfig;

use crate::application::ORGANIZE_APP;

#[derive(Command, Debug, Parser, Clone)]
pub struct GenConfigCmd {
    /// path to an existing or to be created config file
    #[clap(short, long)]
    config_path: PathBuf,

    #[clap(flatten)]
    config_opts: GenConfigOpts,
}

#[derive(Debug, Args, Clone)]
#[group(required = false, multiple = false)]
pub struct GenConfigOpts {
    /// generate a config interactively
    #[clap(short, long)]
    interactive: bool,

    /// generate a config interactively
    #[clap(short, long)]
    template: bool,
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
        let mut config = OrganizeConfig::new();

        let rule = Rule::builder()
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

        config.add_rule(rule.clone());

        if File::open(&self.config_path).is_ok() {
            if Confirm::new().with_prompt("Config file already exists. We will overwrite it, do you have a backup and want to continue?").interact()? {
                let file = File::create(&self.config_path)?;
                ron::ser::to_writer_pretty(file, &config, PrettyConfig::default())?;
            } else {
                bail!("Config file already exists. We will overwrite it, make sure you have a backup and agree in the dialog.");
            }
        } else {
            let file = File::create(&self.config_path)?;
            ron::ser::to_writer_pretty(file, &rule, PrettyConfig::default())?;
            // file.write_all(toml::to_string_pretty(&rule)?.as_bytes())?;
            // file.write_all(serde_yaml::to_string(&rule)?.as_bytes())?;
        };

        Ok(())
    }
}
