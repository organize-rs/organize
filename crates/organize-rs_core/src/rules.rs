//! Rules that can be used in the config file and
//! `organize` operates on

pub mod actions;
pub mod aliases;
pub mod de;
pub mod filters;
pub mod ser;
pub mod tags;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    locations::{LocationCollection, LocationKind},
    rules::{
        actions::{ActionApplicationCollection, ActionApplicationKind},
        filters::{FilterApplicationCollection, FilterApplicationKind, FilterModeGroupKind},
        tags::{Tag, TagCollection},
    },
};

/// [`Rule`] contains a list of objects with the required keys
/// "locations" and "actions". One config can have many [`Rule`]s.
#[derive(Debug, Clone, Deserialize, Serialize)]
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

    pub fn preview(&self) {
        todo!()
    }

    pub fn execute(&self) {
        todo!()
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
    pub fn filter(mut self, filter: FilterApplicationKind) -> RuleBuilder {
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
