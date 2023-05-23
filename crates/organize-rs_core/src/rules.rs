//! Rules that can be used in the config file and
//! `organize` operates on

pub mod actions;
pub mod aliases;
pub mod filters;

use displaydoc::Display;
use serde::{Deserialize, Serialize};

use crate::{
    locations::OrganizeLocation,
    rules::{actions::OrganizeAction, filters::OrganizeFilter},
};

/// Should filters be negated
#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum ApplyOrNegateFilter {
    /// Apply a filter
    Apply(OrganizeFilter),
    /// Negate this filter
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

/// Tags that can be applied to rules
#[derive(Debug, Clone, Deserialize, Serialize, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrganizeTag {
    /// Always run filters/actions with this tag
    Always,
    /// Never run filters/actions with this tag
    Never,
    /// Custom tag for running filters/actions
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
#[derive(Debug, Clone, Deserialize, Serialize, Display, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum OrganizeFilterMode {
    /// All of the filters need to apply
    All,
    /// Any of the filters need to apply
    Any,
    /// None of the filters need to apply
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

/// [`OrganizeRule`] contains a list of objects with the required keys
/// "locations" and "actions". One config can have many [`OrganizeRule`]s.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrganizeRule {
    /// rule name
    name: String,
    /// tag for a rule, so you can run a set of rules by passing `--tags` or `--skip-tags`
    tags: Vec<OrganizeTag>,
    /// whether the rule is enabled / disabled
    enabled: bool,
    /// list of locations
    locations: Vec<OrganizeLocation>,
    /// whether "all", "any" or "none" of the filters must apply
    filter_mode: OrganizeFilterMode,
    /// supported filters
    filters: Vec<ApplyOrNegateFilter>,
    /// supported actions
    actions: Vec<OrganizeAction>,
}

impl OrganizeRule {
    // This method will help users to discover the builder
    pub fn builder() -> OrganizeRuleBuilder {
        OrganizeRuleBuilder::default()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct OrganizeRuleBuilder {
    /// rule name
    name: String,
    /// tag for a rule, so you can run a set of rules by passing `--tags` or `--skip-tags`
    tags: Vec<OrganizeTag>,
    /// whether the rule is enabled / disabled
    enabled: bool,
    /// whether "all", "any" or "none" of the filters must apply
    filter_mode: OrganizeFilterMode,
    /// list of locations
    locations: Vec<OrganizeLocation>,
    /// supported filters
    filters: Vec<ApplyOrNegateFilter>,
    /// supported actions
    actions: Vec<OrganizeAction>,
}

impl OrganizeRuleBuilder {
    pub fn new(name: &str) -> OrganizeRuleBuilder {
        OrganizeRuleBuilder {
            name: String::from(name),
            ..Default::default()
        }
    }

    pub fn build(self) -> OrganizeRule {
        OrganizeRule {
            name: self.name,
            tags: self.tags,
            enabled: self.enabled,
            locations: self.locations,
            filter_mode: self.filter_mode,
            filters: self.filters,
            actions: self.actions,
        }
    }

    /// Set name
    pub fn name(mut self, name: &str) -> OrganizeRuleBuilder {
        self.name = String::from(name);
        self
    }

    /// Set enabled
    pub fn enabled(mut self, enabled: bool) -> OrganizeRuleBuilder {
        self.enabled = enabled;
        self
    }

    /// Add a single location
    pub fn location(mut self, location: OrganizeLocation) -> OrganizeRuleBuilder {
        self.locations.push(location);
        self
    }

    /// Add multiple locations
    pub fn locations(mut self, mut locations: Vec<OrganizeLocation>) -> OrganizeRuleBuilder {
        self.locations.append(&mut locations);
        self
    }

    /// Set filter mode
    pub fn filter_mode(mut self, filter_mode: OrganizeFilterMode) -> OrganizeRuleBuilder {
        self.filter_mode = filter_mode;
        self
    }

    /// Add a single filter
    pub fn filter(mut self, filter: ApplyOrNegateFilter) -> OrganizeRuleBuilder {
        self.filters.push(filter);
        self
    }

    /// Add multiple filters
    pub fn filters(mut self, mut filters: Vec<ApplyOrNegateFilter>) -> OrganizeRuleBuilder {
        self.filters.append(&mut filters);
        self
    }

    /// Add single action
    pub fn action(mut self, action: OrganizeAction) -> OrganizeRuleBuilder {
        self.actions.push(action);
        self
    }

    /// Add multiple actions
    pub fn actions(mut self, mut actions: Vec<OrganizeAction>) -> OrganizeRuleBuilder {
        self.actions.append(&mut actions);
        self
    }

    /// Add single tag
    pub fn tag(mut self, tag: OrganizeTag) -> OrganizeRuleBuilder {
        self.tags.push(tag);
        self
    }

    /// Add multiple actions
    pub fn tags(mut self, mut tags: Vec<OrganizeTag>) -> OrganizeRuleBuilder {
        self.tags.append(&mut tags);
        self
    }
}
