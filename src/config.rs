//! Organize Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use organize_rs_core::rules::{aliases::Alias, OrganizeRule};
use serde::{Deserialize, Serialize};

/// Organize Configuration
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct OrganizeConfig {
    aliases: Vec<Alias>,
    rules: Vec<OrganizeRule>,
}
