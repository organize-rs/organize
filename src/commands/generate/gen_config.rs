//! `gen_config` subcommand

use std::{fs::File, path::PathBuf};

use abscissa_core::{status_err, Application, Command, Runnable, Shutdown};
use anyhow::{bail, Result};
use clap::{Args, Parser};
use dialoguer::Confirm;
use organize_rs_core::{
    actions::{ActionApplicationKind, ActionKind},
    config::OrganizeConfig,
    filters::{FilterGroup, FilterKind, FilterModeKind, RawFilterApplicationKind},
    locations::{LocationKind, MaxDepth, TargetKind},
    rules::{Rule, Rules},
    tags::Tag,
};
use ron::ser::PrettyConfig;

use crate::application::ORGANIZE_APP;

#[derive(Command, Debug, Parser, Clone)]
pub struct GenConfigCmd {
    /// path to an existing or to be created config file
    #[clap(short, long)]
    path: PathBuf,

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
        let mut rules = Rules::new();

        rules.extend(vec![
            // empty_file_rule(),
            // empty_folder_rule(),
            pdf_on_desktop_rule(),
        ]);

        config.add_rules(rules);

        if File::open(&self.path).is_ok() {
            if Confirm::new().with_prompt("Config file already exists. We will overwrite it, do you have a backup and want to continue?").interact()? {
                let file = File::create(&self.path)?;
                ron::ser::to_writer_pretty(file, &config, PrettyConfig::default())?;
            } else {
                bail!("Config file already exists. We will overwrite it, make sure you have a backup and agree in the dialog.");
            }
        } else {
            let file = File::create(&self.path)?;
            ron::ser::to_writer_pretty(file, &config, PrettyConfig::default())?;
            // file.write_all(toml::to_string_pretty(&rule)?.as_bytes())?;
            // file.write_all(serde_yaml::to_string(&rule)?.as_bytes())?;
        };

        Ok(())
    }
}

pub fn empty_file_rule() -> Rule {
    Rule::builder()
        .name("Empty File")
        .filter(FilterGroup {
            invert: RawFilterApplicationKind::Apply,
            mode: FilterModeKind::All,
            filters: vec![FilterKind::Empty],
        })
        .action(ActionApplicationKind::Preview(ActionKind::Trash))
        .location(LocationKind::RecursiveWithMaxDepth {
            path: r"crates\organize-rs_core\tests\fixtures\filters\empty_file".into(),
            target: TargetKind::Files,
            max_depth: MaxDepth::new(1),
        })
        .tag(Tag::Custom("Test::EmptyFile".to_string()))
        .build()
}

pub fn empty_folder_rule() -> Rule {
    Rule::builder()
        .name("Empty Directory")
        .filter(FilterGroup {
            invert: RawFilterApplicationKind::Apply,
            mode: FilterModeKind::All,
            filters: vec![FilterKind::Empty],
        })
        .action(ActionApplicationKind::Preview(ActionKind::Trash))
        .location(LocationKind::RecursiveWithMaxDepth {
            path: r"crates\organize-rs_core\tests\fixtures\filters\empty_folder".into(),
            target: TargetKind::Directories,
            max_depth: MaxDepth::new(1),
        })
        .tag(Tag::Custom("Test::EmptyDirectory".to_string()))
        .build()
}

pub fn pdf_on_desktop_rule() -> Rule {
    Rule::builder()
        .name("PDFs on Desktop")
        .filter(FilterGroup {
            invert: RawFilterApplicationKind::Apply,
            mode: FilterModeKind::All,
            filters: vec![FilterKind::Extension {
                exts: vec![String::from("pdf")],
            }],
        })
        .action(ActionApplicationKind::Preview(ActionKind::NoAction))
        .location(LocationKind::RecursiveWithMaxDepth {
            path: r"C:\Users\dailyuse\Desktop".into(),
            target: TargetKind::Directories,
            max_depth: MaxDepth::new(4),
        })
        .tag(Tag::Custom("Documents::PDF".to_string()))
        .build()
}
