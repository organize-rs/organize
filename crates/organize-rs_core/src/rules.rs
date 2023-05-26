//! Rules that can be used in the config file and
//! `organize` operates on

pub mod de;
pub mod ser;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    actions::{ActionApplicationCollection, ActionApplicationKind},
    filters::{
        FilterApplicationCollection, FilterApplicationKind, FilterKind, FilterModeGroupKind,
    },
    locations::{LocationCollection, LocationKind},
    tags::{Tag, TagCollection},
};

/// [`Rules`] contains a list of [`Rule`] objects with the required keys
/// "locations" and "actions". One config can have many [`Rules`].
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(transparent)]
pub struct Rules(Vec<Rule>);

impl Rules {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn preview(&self) {
        todo!()
    }

    pub fn execute(&self) {
        todo!()
    }
}

impl std::ops::DerefMut for Rules {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for Rules {
    type Target = Vec<Rule>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [`Rule`] contains a objects with the required keys
/// "locations" and "actions". One config can have many [`Rules].
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default, rename = "rule")]
pub struct Rule {
    /// rule name
    name: String,
    /// tag for a rule, so you can run a set of rules by passing `--tags` or `--skip-tags`
    tags: TagCollection,
    /// whether the rule is enabled / disabled
    enabled: bool,
    /// whether "all", "any" or "none" of the filters must apply
    filter_mode: FilterModeGroupKind,
    /// list of locations
    locations: LocationCollection,
    /// supported filters
    filters: FilterApplicationCollection,
    /// supported actions
    actions: ActionApplicationCollection,
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Rule {
            name,
            tags,
            enabled,
            locations,
            filter_mode,
            filters,
            actions,
        } = self;

        write!(
            f,
            "
    Rule - {name} ({enabled})
        -> {filter_mode}

    Tags: {tags:?}
    Locations: {locations:?}

    Filters: {filters:?}

    Actions: {actions:?}
        "
        )
    }
}

impl Rule {
    // This method will help users to discover the builder
    pub fn builder() -> RuleBuilder {
        RuleBuilder::default()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RuleBuilder {
    /// rule name
    name: String,
    /// tag for a rule, so you can run a set of rules by passing `--tags` or `--skip-tags`
    tags: TagCollection,
    /// whether the rule is enabled / disabled
    enabled: bool,
    /// whether "all", "any" or "none" of the filters must apply
    filter_mode: FilterModeGroupKind,
    /// list of locations
    locations: LocationCollection,
    /// supported filters
    filters: FilterApplicationCollection,
    /// supported actions
    actions: ActionApplicationCollection,
}

impl RuleBuilder {
    pub fn new(name: &str) -> RuleBuilder {
        RuleBuilder {
            name: String::from(name),
            ..Default::default()
        }
    }

    pub fn build(self) -> Rule {
        Rule {
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
    pub fn name(mut self, name: &str) -> RuleBuilder {
        self.name = String::from(name);
        self
    }

    /// Set enabled
    pub fn enabled(mut self, enabled: bool) -> RuleBuilder {
        self.enabled = enabled;
        self
    }

    /// Add a single location
    pub fn location(mut self, location: LocationKind) -> RuleBuilder {
        self.locations.push(location);
        self
    }

    /// Add multiple locations
    pub fn locations(mut self, mut locations: LocationCollection) -> RuleBuilder {
        self.locations.append(&mut locations);
        self
    }

    /// Set filter mode
    pub fn filter_mode(mut self, filter_mode: FilterModeGroupKind) -> RuleBuilder {
        self.filter_mode = filter_mode;
        self
    }

    /// Add a single filter
    pub fn filter(mut self, filter: FilterApplicationKind<FilterKind>) -> RuleBuilder {
        self.filters.push(filter);
        self
    }

    /// Add multiple filters
    pub fn filters(mut self, mut filters: FilterApplicationCollection) -> RuleBuilder {
        self.filters.append(&mut filters);
        self
    }

    /// Add single action
    pub fn action(mut self, action: ActionApplicationKind) -> RuleBuilder {
        self.actions.push(action);
        self
    }

    /// Add multiple actions
    pub fn actions(mut self, mut actions: ActionApplicationCollection) -> RuleBuilder {
        self.actions.append(&mut actions);
        self
    }

    /// Add single tag
    pub fn tag(mut self, tag: Tag) -> RuleBuilder {
        self.tags.push(tag);
        self
    }

    /// Add multiple tags
    pub fn tags(mut self, mut tags: TagCollection) -> RuleBuilder {
        self.tags.append(&mut tags);
        self
    }
}
