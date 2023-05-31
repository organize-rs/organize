//! Organize Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use organize_rs_core::aliases::Reference;
use organize_rs_core::rules::Rule;
use serde::{Deserialize, Serialize};

pub static CONFIG_TEMPLATE_YAML: &str = include_str!("../config/template.yaml");

/// Organize Configuration
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct OrganizeAppConfig {
    aliases: Vec<Reference>,
    rules: Vec<Rule>,
}
