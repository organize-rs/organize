//! Rules that can be used in the config file and
//! `organize` operates on

pub mod actions;
pub mod filters;

use std::path::PathBuf;

use crate::rules::{actions::OrganizeAction, filters::OrganizeFilter};

#[derive(Debug, Clone)]
pub enum ApplyOrNegateFilter {
    Apply(OrganizeFilter),
    Negate(OrganizeFilter),
}

#[derive(Debug, Clone)]
pub enum Recurse {
    Flat,
    Recursive,
}

/// Tags that can be applied to rules
#[derive(Debug, Clone)]
pub enum OrganizeTag {
    Always,
    Never,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum OrganizeFilterMode {
    All,
    Any,
    None,
}

#[derive(Debug, Clone)]
pub enum OrganizeTargets {
    Dirs,
    Files,
}

#[derive(Debug, Clone)]
pub struct OrganizeRule {
    /// rule name
    name: String,
    /// whether the rule is enabled / disabled
    enabled: bool,
    /// when targets is set to dirs, organize will work on the folders, not on files
    targets: OrganizeTargets,
    /// list of locations
    locations: Vec<PathBuf>,
    /// whether to recurse into subfolders of all locations
    subfolders: Recurse,
    /// whether "all", "any" or "none" of the filters must apply
    filter_mode: OrganizeFilterMode,
    /// supported filters
    filters: Vec<ApplyOrNegateFilter>,
    /// supported actions
    actions: Vec<OrganizeAction>,
    /// tag for a rule, so you can run a set of rules by passing `--tags` or `--skip-tags`
    tags: Vec<OrganizeTag>,
}
