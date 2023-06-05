use std::{collections::HashSet, path::Path};

use itertools::Itertools;

use crate::{
    actions::{ActionApplicationKind, ActionResultKind},
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

pub struct Runner<S>
where
    S: ProcessingStage,
{
    configs: Vec<OrganizeConfig>,
    extra: S,
}

impl Runner<Initialize> {
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
    pub fn finish_inspection(self) -> Runner<ActionPreview> {
        Runner::<ActionPreview> {
            configs: self.configs,
            extra: ActionPreview::with_entries(self.extra.entries()),
        }
    }

    pub fn inspect_entries(self) -> Runner<Inspection> {
        self.extra.print_entries();
        self
    }
}

impl Runner<ActionPreview> {
    pub fn preview_actions(self) -> OrganizeResult<()> {
        // * if action::mode == `is_preview()` we return Runner<Reporting>
        // * and only println!() what an action might do
        // * else we continue to Runner<ActionApplication>
        self.extra.entries().iter().for_each(|(rule, entry)| {
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
        Ok(())
    }

    pub fn ask_confirmation(self) -> OrganizeResult<Runner<ActionApplication>> {
        todo!()
    }
}

impl Runner<ActionApplication> {
    pub fn apply_actions(self) -> OrganizeResult<Runner<ActionApplication>> {
        todo!()
    }

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
    pub fn view_conflicts(self) -> Runner<ActionPreview> {
        todo!()
    }
}
