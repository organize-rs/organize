use serde::{Deserialize, Serialize};

use crate::rules::Rules;

/// Organize Configuration
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
// #[serde(deny_unknown_fields)]
#[serde(default)]
pub struct PyOrganizeConfig {
    // aliases: Vec<Reference>,
    rules: Rules,
}
