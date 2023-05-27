//! organize-py config

use displaydoc::Display;
use serde::{Deserialize, Serialize};

use crate::{
    actions::ActionApplicationCollection,
    filters::{FilterKind, FilterModeKind},
    locations::LocationCollection,
    tags::TagCollection,
};

/// Organize Configuration
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
// #[serde(deny_unknown_fields)]
#[serde(default)]
pub struct PyOrganizeConfig {
    // aliases: Vec<Reference>,
    rules: PyRules,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display, Default)]
#[serde(transparent)]
pub struct PyFilterApplicationCollection(Vec<FilterKind>);

/// [`PyRules`] contains a list of [`PyRule`] objects with the required keys
/// "locations" and "actions". One config can have many [`PyRules`].
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(transparent)]
pub struct PyRules(Vec<PyRule>);

/// [`PyRule`] contains a objects with the required keys
/// "locations" and "actions". One config can have many [`PyRules].
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default, rename = "rule")]
pub struct PyRule {
    /// rule name
    name: String,
    /// tag for a rule, so you can run a set of rules by passing `--tags` or `--skip-tags`
    tags: TagCollection,
    /// whether the rule is enabled / disabled
    enabled: bool,
    /// whether "all", "any" or "none" of the filters must apply
    filter_mode: FilterModeKind,
    /// list of locations
    locations: LocationCollection,
    /// supported filters
    filters: PyFilterApplicationCollection,
    /// supported actions
    actions: ActionApplicationCollection,
}
