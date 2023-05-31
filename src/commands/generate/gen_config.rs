//! `gen_config` subcommand

use abscissa_core::{status_err, Application, Command, Runnable, Shutdown};
use anyhow::{bail, Result};
use clap::{Args, Parser};
use dialoguer::Confirm;
use organize_rs_core::{
    actions::{ActionApplicationKind, ActionKind},
    config::{ConfigFileFormat, OrganizeConfig},
    error::ConfigErrorKind,
    filters::{FilterGroup, FilterKind, FilterModeKind, RawFilterApplicationKind},
    locations::{LocationKind, MaxDepth, TargetKind},
    rules::{empty_file_rule, empty_folder_rule, pdf_on_desktop_rule, Rule, Rules},
    tags::Tag,
};
use ron::ser::PrettyConfig;
use std::io::Write;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use crate::application::ORGANIZE_APP;

#[derive(Command, Debug, Parser, Clone)]
pub struct GenConfigCmd {
    /// path to an existing or to be created config file
    #[clap(short, long)]
    path: PathBuf,

    #[clap(flatten)]
    config_opts: GenConfigOpts,
    // TODO: maybe explicitly chose?
    // #[clap(long, default_value_t = ConfigFileFormat::Ron)]
    // format: ConfigFileFormat,
}

#[derive(Debug, Args, Clone)]
#[group(required = false, multiple = false)]
pub struct GenConfigOpts {
    /// generate a config interactively
    #[clap(short, long)]
    interactive: bool,

    /// output template config
    #[clap(short, long)]
    template: bool,
}

impl Runnable for GenConfigCmd {
    /// Start the application.
    fn run(&self) {
        let result = if self.config_opts.template {
            self.generate_config_template_yaml()
        } else {
            self.generate_example_config()
        };

        match result {
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
            empty_file_rule(),
            empty_folder_rule(),
            pdf_on_desktop_rule(),
        ]);

        config.add_rules(rules);

        if File::open(&self.path).is_ok() {
            if Confirm::new().with_prompt("Config file already exists. We will overwrite it, do you have a backup and want to continue?").default(false).interact()? {
                config.write_to_file(&self.path, true)?;
            } else {
                bail!("Config file already exists. We will overwrite it, make sure you have a backup and agree in the dialog.");
            }
        } else {
            config.write_to_file(&self.path, true)?;
        };

        Ok(())
    }

    fn generate_config_template_yaml(&self) -> Result<()> {
        // TODO: Handle in a better way
        let path = format!("{}{}", self.path.as_path().display(), ".tmpl.yaml");

        if File::open(&path).is_ok() {
            if Confirm::new().with_prompt("Config file already exists. We will overwrite it, do you have a backup and want to continue?").default(false).interact()? {
                write!(&mut File::open(&path)?, "{}", crate::config::CONFIG_TEMPLATE_YAML)?;
            } else {
                bail!("Config file already exists at:{path}. We will overwrite it, make sure you have a backup and agree in the dialog.");
            }
        } else {
            let mut file = File::create(&path)?;
            write!(&mut file, "{}", crate::config::CONFIG_TEMPLATE_YAML)?;
        };

        println!("organize config template has been generated successfully!");
        println!("You can find the config template here: {path}");

        Ok(())
    }
}
