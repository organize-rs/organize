//! `rule` subcommand

use std::str::FromStr;

use abscissa_core::{status_err, Application, Command, Runnable, Shutdown};
use anyhow::Result;
use clap::Parser;
use organize_rs_core::{
    locations::{OrganizeLocation, OrganizeTarget},
    parsers::SizeRange,
    rules::{
        actions::OrganizeAction, filters::OrganizeFilter, ApplyOrNegateFilter, OrganizeFilterMode,
        OrganizeRule, OrganizeTag,
    },
};

use crate::application::ORGANIZE_APP;

/// `rule` subcommand
#[derive(Command, Debug, Parser)]
pub struct RuleCmd {
    #[clap(long)]
    execute: bool,
}

impl Runnable for RuleCmd {
    fn run(&self) {
        match self.inner_run() {
            Ok(_) => {}
            Err(err) => {
                status_err!("{}", err);
                ORGANIZE_APP.shutdown(Shutdown::Crash)
            }
        };
    }
}

impl RuleCmd {
    fn inner_run(&self) -> Result<()> {
        let rule_builder = OrganizeRule::builder();

        let _rule = rule_builder
            .name("test")
            .filter(ApplyOrNegateFilter::Apply(OrganizeFilter::Extension {
                exts: vec!["toml".to_string()],
            }))
            .filter(ApplyOrNegateFilter::Apply(OrganizeFilter::Size {
                range: SizeRange::from_str("1KiB..")?,
            }))
            .filter_mode(OrganizeFilterMode::All)
            .action(OrganizeAction::Trash)
            .location(OrganizeLocation::NonRecursive {
                path: r"C:\Users\dailyuse\dev-src\organize".into(),
                target: OrganizeTarget::Files,
            })
            .tag(OrganizeTag::Custom("my_test_rule".to_string()))
            .build();

        Ok(())
    }
}
