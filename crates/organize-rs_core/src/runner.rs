use std::{collections::HashSet, path::Path};

use itertools::Itertools;

use crate::{
    actions::ActionResultKind,
    actors::{filter_applicator::FilterApplicator, location_walker::LocationWalker},
    config::OrganizeConfig,
    error::OrganizeResult,
    state::{
        ActionApplication, ActionPreview, ConflictHandling, Filtering, Initialize, Inspection,
        ProcessingStage,
    },
    tags::{Tag, TagCollection},
};

use std::iter::FromIterator;

/// A state machine that runs through the different stages of the organize process.
///
/// # Type Parameters
///
/// * `S` - The current stage of the organize process.
pub struct Runner<S>
where
    S: ProcessingStage,
{
    /// The organize configurations to use.
    configs: Vec<OrganizeConfig>,
    /// The extra data for the current stage.
    extra: S,
}

impl Runner<Initialize> {
    /// Create a runner from a list of organize configuration files.
    ///
    /// # Arguments
    ///
    /// * `paths` - The paths to the organize configuration files.
    ///
    /// # Returns
    ///
    /// A runner in the filtering stage.
    pub fn load_configs(paths: &[impl AsRef<Path>]) -> Runner<Filtering> {
        let mut configs = vec![];
        paths.iter().for_each(|path| {
            let config = OrganizeConfig::load_from_file(path);
            configs.push(config);
        });

        Runner::<Filtering> {
            configs,
            extra: Filtering::default(),
        }
    }
}

impl Runner<Filtering> {
    /// Apply the filters to the entries.
    ///
    /// # Arguments
    ///
    /// * `tags` - The tags to apply to the filters.
    ///
    /// # Returns
    ///
    /// A runner in the inspection stage.
    pub fn apply_filters(self, tags: Vec<Tag>) -> Runner<Inspection> {
        let mut entries = vec![];
        self.configs.iter().for_each(|config| {
            config.rules().iter().for_each(|rule| {
                if rule.enabled() & !rule.tags().contains(&Tag::Never) {
                    let tag_collection = rule.tags();
                    let tag_applies = Self::tag_applies(&tag_collection, &tags);

                    match tag_applies {
                        Some(true) | None => {
                            let data =
                                LocationWalker::new(rule.locations()).collect_dir_entry_data();
                            let filtered_data =
                                FilterApplicator::new(rule.filters()).get_applicable_items(data);
                            entries.push((rule.clone(), filtered_data))
                        }
                        Some(false) => println!("Given tags don't apply, skipping ... {rule}"),
                    }
                } else {
                    println!("Not enabled or should be never run, skipping ... {rule}")
                }
            })
        });

        Runner::<Inspection> {
            configs: self.configs,
            extra: Inspection::with_entries(entries),
        }
    }

    /// Checks if a given [`Tag`] is in a [`TagCollection`] of a [`crate::rules::Rule`]
    ///
    /// # Arguments
    ///
    /// * `tag_collection` - The [`TagCollection`] to check if the [`Tag`] is in.
    /// * `tags` - The [`Tag`]s to check if they are in the [`TagCollection`].
    ///
    /// # Returns
    ///
    /// * `Some(true)` - If the [`TagCollection`] is empty or the [`Tag`]s are empty.
    /// * `Some(false)` - If the [`TagCollection`] is not empty and the [`Tag`]s are not empty.
    /// * `None` - If the [`TagCollection`] is empty and the [`Tag`]s are not empty.
    fn tag_applies(tag_collection: &TagCollection, tags: &[Tag]) -> Option<bool> {
        if !tags.is_empty() {
            let rule_tag_set: HashSet<&Tag> = HashSet::from_iter(tag_collection.iter());
            let cli_tag_set: HashSet<&Tag> = HashSet::from_iter(tags.iter());
            Some(!rule_tag_set.is_disjoint(&cli_tag_set))
        } else {
            None
        }
    }
}

impl Runner<Inspection> {
    /// Finish the inspection stage and move to the preview stage.
    ///
    /// # Returns
    ///
    /// A runner in the preview stage.
    pub fn finish_inspection(self) -> Runner<ActionPreview> {
        Runner::<ActionPreview> {
            configs: self.configs,
            extra: ActionPreview::with_entries(self.extra.entries()),
        }
    }

    /// Inspect the entries.
    ///
    /// # Returns
    ///
    /// A runner in the inspection stage.
    pub fn inspect_entries(self) -> Runner<Inspection> {
        self.extra.print_entries();
        self
    }
}

impl Runner<ActionPreview> {
    /// Preview the actions.
    ///
    /// # Errors
    ///
    /// * [`OrganizeError::ActionError`] - If an action fails.
    ///
    /// # Returns
    ///
    /// A runner in the action application stage.

    pub fn preview_actions(self) -> OrganizeResult<Runner<ActionApplication>> {
        let entries = self.extra.entries();
        entries.iter().for_each(|(rule, entry)| {
            rule.actions().iter().for_each(|action_container| {
                entry.iter().for_each(|entry| {
                    match action_container.action.get_action()(
                        entry,
                        true, // * is always true, as it's preview
                             // ! for application we can use:
                             // ! `action_container.mode.is_preview()`
                    ) {
                        Ok(ActionResultKind::Preview {
                            msg,
                            path: _,
                            action: _,
                        }) => println!("{msg}"),
                        Err(err) => eprintln!("{err}"),
                        _ => (),
                    }
                })
            })
        });

        Ok(Runner::<ActionApplication> {
            configs: self.configs,
            extra: ActionApplication::with_entries(entries),
        })
    }
}

impl Runner<ActionApplication> {
    /// Apply the actions to the entries.
    pub fn apply_actions(self) -> OrganizeResult<Runner<ActionApplication>> {
        todo!()
    }

    /// Check for conflicts if the actions would be applied.
    pub fn check_conflicts(self) -> Runner<ConflictHandling> {
        todo!()
    }
}

// pub fn handle_conflicts(self) -> Runner<HandleConflicts> {
//     let entries = self.extra.entries();

//     Runner::<HandleConflicts> {
//         configs: self.configs,
//         extra: HandleConflicts::with_entries(entries),
//     }
// }
impl Runner<ConflictHandling> {
    /// View the conflicts that would occur if the actions were applied.
    pub fn view_conflicts(self) -> Runner<ActionPreview> {
        todo!()
    }
}
