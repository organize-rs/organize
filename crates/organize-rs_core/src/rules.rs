//! Rules that can be used in the config file and
//! `organize` operates on

pub mod actions;
pub mod aliases;
pub mod filters;

// Generated from py-organize
// pub mod py_organize;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::rules::{actions::OrganizeAction, filters::OrganizeFilter};

/// Should filters be negated
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ApplyOrNegateFilter {
    Apply(OrganizeFilter),
    Negate(OrganizeFilter),
}

impl ApplyOrNegateFilter {
    /// Returns `true` if the apply or negate filter is [`Apply`].
    ///
    /// [`Apply`]: ApplyOrNegateFilter::Apply
    #[must_use]
    pub fn is_apply(&self) -> bool {
        matches!(self, Self::Apply(..))
    }

    pub fn as_apply(&self) -> Option<&OrganizeFilter> {
        if let Self::Apply(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_apply(self) -> Result<OrganizeFilter, Self> {
        if let Self::Apply(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the apply or negate filter is [`Negate`].
    ///
    /// [`Negate`]: ApplyOrNegateFilter::Negate
    #[must_use]
    pub fn is_negate(&self) -> bool {
        matches!(self, Self::Negate(..))
    }

    pub fn as_negate(&self) -> Option<&OrganizeFilter> {
        if let Self::Negate(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_negate(self) -> Result<OrganizeFilter, Self> {
        if let Self::Negate(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

/// Should we go recursive into folders
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Recurse {
    #[serde(rename = "false")]
    Flat,
    #[serde(rename = "true")]
    Recursive,
}

impl Recurse {
    /// Returns `true` if the recurse is [`Flat`].
    ///
    /// [`Flat`]: Recurse::Flat
    #[must_use]
    pub fn is_flat(&self) -> bool {
        matches!(self, Self::Flat)
    }

    /// Returns `true` if the recurse is [`Recursive`].
    ///
    /// [`Recursive`]: Recurse::Recursive
    #[must_use]
    pub fn is_recursive(&self) -> bool {
        matches!(self, Self::Recursive)
    }
}

impl Default for Recurse {
    fn default() -> Self {
        Self::Flat
    }
}

/// Tags that can be applied to rules
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OrganizeTag {
    Always,
    Never,
    Custom(String),
}

impl OrganizeTag {
    /// Returns `true` if the organize tag is [`Always`].
    ///
    /// [`Always`]: OrganizeTag::Always
    #[must_use]
    pub fn is_always(&self) -> bool {
        matches!(self, Self::Always)
    }

    /// Returns `true` if the organize tag is [`Never`].
    ///
    /// [`Never`]: OrganizeTag::Never
    #[must_use]
    pub fn is_never(&self) -> bool {
        matches!(self, Self::Never)
    }

    /// Returns `true` if the organize tag is [`Custom`].
    ///
    /// [`Custom`]: OrganizeTag::Custom
    #[must_use]
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(..))
    }

    pub fn as_custom(&self) -> Option<&String> {
        if let Self::Custom(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_custom(self) -> Result<String, Self> {
        if let Self::Custom(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

/// Application of filters, so whether "all", "any" or "none"
/// of the filters must apply
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OrganizeFilterMode {
    All,
    Any,
    None,
}

impl Default for OrganizeFilterMode {
    fn default() -> Self {
        Self::Any
    }
}

impl OrganizeFilterMode {
    /// Returns `true` if the organize filter mode is [`All`].
    ///
    /// [`All`]: OrganizeFilterMode::All
    #[must_use]
    pub fn is_all(&self) -> bool {
        matches!(self, Self::All)
    }

    /// Returns `true` if the organize filter mode is [`Any`].
    ///
    /// [`Any`]: OrganizeFilterMode::Any
    #[must_use]
    pub fn is_any(&self) -> bool {
        matches!(self, Self::Any)
    }

    /// Returns `true` if the organize filter mode is [`None`].
    ///
    /// [`None`]: OrganizeFilterMode::None
    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

/// Targets `organize` operates on.
///
/// When targets is set to dirs, organize will work on
/// the folders, not on files.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OrganizeTargets {
    Dirs,
    Files,
}

impl OrganizeTargets {
    /// Returns `true` if the organize targets is [`Files`].
    ///
    /// [`Files`]: OrganizeTargets::Files
    #[must_use]
    pub fn is_files(&self) -> bool {
        matches!(self, Self::Files)
    }

    /// Returns `true` if the organize targets is [`Dirs`].
    ///
    /// [`Dirs`]: OrganizeTargets::Dirs
    #[must_use]
    pub fn is_dirs(&self) -> bool {
        matches!(self, Self::Dirs)
    }
}
/// [`OrganizeRule`] contains a list of objects with the required keys
/// "locations" and "actions". One config can have many [`OrganizeRule`]s.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrganizeRule {
    /// rule name
    name: String,
    /// whether the rule is enabled / disabled
    enabled: bool,
    /// when targets is set to dirs, organize will work on
    /// the folders, not on files
    targets: OrganizeTargets,
    /// list of locations
    locations: Vec<String>,
    /// whether to recurse into subfolders of all locations
    #[serde(rename = "subfolders")]
    recursive: Recurse,
    /// whether "all", "any" or "none" of the filters must apply
    filter_mode: OrganizeFilterMode,
    /// supported filters
    filters: Vec<ApplyOrNegateFilter>,
    /// supported actions
    actions: Vec<OrganizeAction>,
    /// tag for a rule, so you can run a set of rules by passing `--tags` or `--skip-tags`
    tags: Vec<OrganizeTag>,
}

impl OrganizeRule {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn tags(&self) -> &[OrganizeTag] {
        self.tags.as_ref()
    }

    pub fn actions(&self) -> &[OrganizeAction] {
        self.actions.as_ref()
    }

    pub fn filters(&self) -> &[ApplyOrNegateFilter] {
        self.filters.as_ref()
    }

    pub fn filter_mode(&self) -> &OrganizeFilterMode {
        &self.filter_mode
    }

    pub fn subfolders(&self) -> &Recurse {
        &self.recursive
    }

    pub fn targets(&self) -> &OrganizeTargets {
        &self.targets
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn locations(&self) -> &[String] {
        self.locations.as_ref()
    }
}
