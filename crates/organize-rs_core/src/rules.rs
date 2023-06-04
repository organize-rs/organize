//! Rules that can be used in the config file and
//! `organize` operates on

use std::{
    fmt::Display,
    path::PathBuf,
    slice::{Iter, IterMut},
};

use serde::{Deserialize, Serialize};

use crate::{
    actions::{
        conflicts::OnConflictKind, ActionApplicationCollection, ActionApplicationKind,
        ActionContainer, ActionKind,
    },
    filters::{
        FilterApplicationKind, FilterGroup, FilterGroupCollection, FilterGroupOperationKind,
        FilterKind,
    },
    locations::{LocationCollection, LocationKind, MaxDepth, TargetKind},
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

    pub fn iter(&self) -> Iter<'_, Rule> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Rule> {
        self.0.iter_mut()
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
    /// list of locations
    locations: LocationCollection,
    /// supported filters
    filter_groups: FilterGroupCollection,
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
            filter_groups: filters,
            actions,
        } = self;

        write!(
            f,
            "
    Rule - {name} ({enabled})

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

    pub fn locations(&self) -> LocationCollection {
        self.locations.clone()
    }

    pub fn tags(&self) -> TagCollection {
        self.tags.clone()
    }

    pub fn filters(&self) -> FilterGroupCollection {
        self.filter_groups.clone()
    }

    pub fn actions(&self) -> ActionApplicationCollection {
        self.actions.clone()
    }

    pub fn enabled(&self) -> bool {
        self.enabled
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
    /// list of locations
    locations: LocationCollection,
    /// supported filters
    filter_groups: FilterGroupCollection,
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
            filter_groups: self.filter_groups,
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

    /// Add a single filter
    pub fn filter_group(mut self, filter: FilterGroup<Vec<FilterKind>>) -> RuleBuilder {
        self.filter_groups.push(filter);
        self
    }

    /// Add multiple filters
    pub fn filter_groups(mut self, mut filters: FilterGroupCollection) -> RuleBuilder {
        self.filter_groups.append(&mut filters);
        self
    }

    /// Add single action
    pub fn action(mut self, action: ActionContainer) -> RuleBuilder {
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

pub fn empty_file_rule() -> Rule {
    Rule::builder()
        .name("Empty File")
        .filter_group(FilterGroup {
            operation: FilterGroupOperationKind::Include,
            mode: FilterApplicationKind::All,
            filters: vec![FilterKind::Empty],
        })
        .action(ActionContainer {
            mode: ActionApplicationKind::Preview,
            action: ActionKind::Trash,
        })
        .location(LocationKind::RecursiveWithMaxDepth {
            path: r"crates\organize-rs_core\tests\fixtures\filters\empty_file".into(),
            target: TargetKind::Files,
            max_depth: MaxDepth::new(1),
        })
        .tag(Tag::Custom("Test::EmptyFile".to_string()))
        .build()
}

pub fn empty_folder_rule() -> Rule {
    Rule::builder()
        .name("Empty Directory")
        .filter_group(FilterGroup {
            operation: FilterGroupOperationKind::Include,
            mode: FilterApplicationKind::All,
            filters: vec![FilterKind::Empty],
        })
        .action(ActionContainer {
            mode: ActionApplicationKind::Preview,
            action: ActionKind::Trash,
        })
        .location(LocationKind::RecursiveWithMaxDepth {
            path: r"crates\organize-rs_core\tests\fixtures\filters\empty_folder".into(),
            target: TargetKind::Directories,
            max_depth: MaxDepth::new(1),
        })
        .tag(Tag::Custom("Test::EmptyDirectory".to_string()))
        .build()
}

pub fn pdf_on_desktop_rule() -> Rule {
    Rule::builder()
        .name("PDFs on Desktop")
        .filter_group(FilterGroup {
            operation: FilterGroupOperationKind::Include,
            mode: FilterApplicationKind::All,
            filters: vec![FilterKind::Extension {
                exts: vec![String::from("pdf")],
            }],
        })
        .action(ActionContainer {
            mode: ActionApplicationKind::Preview,
            action: ActionKind::Move {
                dst: "~/Desktop/PDFs".into(),
                on_conflict: OnConflictKind::Skip,
                rename_template: None,
                filesystem: None,
            },
        })
        .location(LocationKind::RecursiveWithMaxDepth {
            path: r"C:\Users\dailyuse\Desktop".into(),
            target: TargetKind::Files,
            max_depth: MaxDepth::new(4),
        })
        .tag(Tag::Custom("Documents::PDF".to_string()))
        .build()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_empty_folder_rule_passes() {
        let rule = empty_folder_rule();
        insta::assert_yaml_snapshot!(rule, @r###"
        ---
        name: Empty Directory
        tags:
          - custom: "Test::EmptyDirectory"
        enabled: false
        locations:
          - recursive:
              path: "crates\\organize-rs_core\\tests\\fixtures\\filters\\empty_folder"
              max_depth: 1
              target: folders
        filter_groups:
          - results: exclude
            match: all
            filters:
              - empty
        actions:
          - mode: preview
            action: trash
        "###);
    }

    #[test]
    fn test_pdf_on_desktop_rule_passes() {
        let rule = pdf_on_desktop_rule();
        insta::assert_yaml_snapshot!(rule, @r###"
        ---
        name: PDFs on Desktop
        tags:
          - custom: "Documents::PDF"
        enabled: false
        locations:
          - recursive:
              path: "C:\\Users\\dailyuse\\Desktop"
              max_depth: 4
              target: files
        filter_groups:
          - results: exclude
            match: all
            filters:
              - extension:
                  exts:
                    - pdf
        actions:
          - mode: preview
            action:
              move:
                dst: ~/Desktop/PDFs
                on_conflict: skip
                rename_template: ~
                filesystem: ~
        "###);
    }
}
