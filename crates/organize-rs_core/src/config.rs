//! organize-rs config

use std::{fmt::Display, panic, path::Path};

use serde::{Deserialize, Serialize};

use crate::rules::{Rule, Rules};

/// Organize Configuration
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
// #[serde(deny_unknown_fields)]
#[serde(default)]
pub struct OrganizeConfig {
    // aliases: Vec<Reference>,
    rules: Rules,
}

impl Display for OrganizeConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rule in self.rules.iter() {
            write!(f, "{rule}")?;
        }
        Ok(())
    }
}

impl OrganizeConfig {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> Self {
        let file = std::fs::File::open(path.as_ref()).expect("opening file shouldn't fail.");

        let ext = path
            .as_ref()
            .extension()
            .expect("getting file extension shouldn't fail.")
            .to_str()
            .expect("conversion from OsStr to String shouldn't fail.");

        match ext {
            "ron" => ron::de::from_reader(file).expect("config file parsing shouldn't fail."),
            "yaml" => todo!("yaml config file support not implemented (yet)"),
            "toml" => todo!("toml config file support not implemented (yet)"),
            "json" => todo!("json config file support not implemented (yet)"),
            _ => panic!("only ron, json, yaml, and toml config formats are supported."),
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule)
    }

    pub fn add_rules(&mut self, rules: Rules) {
        self.rules.extend_from_slice(&rules)
    }

    pub fn rules(&self) -> &Rules {
        &self.rules
    }
}
