use std::{collections::HashSet, path::Path};

use itertools::Itertools;

use crate::{
    actors::{filter_applicator::FilterApplicator, location_walker::LocationWalker},
    config::OrganizeConfig,
    state::{HandleConflicts, Init, Inspect, ProcessingState, Start},
    tags::{Tag, TagCollection},
};

use std::iter::FromIterator;

pub struct Runner<S>
where
    S: ProcessingState,
{
    configs: Vec<OrganizeConfig>,
    extra: S,
}

impl Runner<Init> {
    pub fn load_configs(paths: &[impl AsRef<Path>]) -> Runner<Start> {
        let mut configs = vec![];
        paths.into_iter().for_each(|path| {
            let config = OrganizeConfig::load_from_file(path);
            configs.push(config);
        });

        Runner::<Start> {
            configs,
            extra: Start::default(),
        }
    }
}

impl Runner<Start> {
    pub fn apply_filters(self, tags: Vec<Tag>) -> Runner<Inspect> {
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

        Runner::<Inspect> {
            configs: self.configs,
            extra: Inspect::with_entries(entries),
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

impl Runner<Inspect> {
    pub fn handle_conflicts(self) -> Runner<HandleConflicts> {
        let entries = self.extra.entries();

        Runner::<HandleConflicts> {
            configs: self.configs,
            extra: HandleConflicts::with_entries(entries),
        }
    }

    pub fn inspect_entries(self) -> Runner<Inspect> {
        self.extra.print_entries();
        self
    }
}

impl Runner<HandleConflicts> {}
