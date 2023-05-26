//! organize-rs config

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

impl OrganizeConfig {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule)
    }
}
