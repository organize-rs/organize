use std::{collections::HashSet, path::Path};

use crate::{
    config::OrganizeConfig,
    state::{HandleConflicts, Init, Inspect, ProcessingState, Start},
    tags::Tag,
    FilteredFileWalker,
};

use std::iter::FromIterator;

pub struct Runner<S>
where
    S: ProcessingState,
{
    config: OrganizeConfig,
    extra: S,
}

impl Runner<Init> {
    pub fn load_config(path: impl AsRef<Path>) -> Runner<Start> {
        let config = OrganizeConfig::load_from_file(path);

        Runner::<Start> {
            config,
            extra: Start::default(),
        }
    }
}

impl Runner<Start> {
    pub fn apply_filters(self, tags: Vec<Tag>) -> Runner<Inspect> {
        let mut entries = vec![];
        self.config.rules().iter().for_each(|rule| {
            if rule.enabled() & !rule.tags().contains(&Tag::Never) {
                // check the tags in a rule
                let tag_applies = if !tags.is_empty() {
                    Self::tag_applies(rule, &tags)
                } else {
                    None
                };

                match tag_applies {
                    Some(true) | None => {
                        let mut walker = FilteredFileWalker::new();
                        walker.get_applicable_items(rule.locations(), rule.filters());
                        entries.push((rule.clone(), walker))
                    }
                    Some(false) => println!("Given tags don't apply, skipping ... {rule}"),
                }
            } else {
                println!("Not enabled or should be never run, skipping ... {rule}")
            }
        });

        Runner::<Inspect> {
            config: self.config,
            extra: Inspect::with_entries(entries),
        }
    }

    fn tag_applies(rule: &crate::rules::Rule, tags: &[Tag]) -> Option<bool> {
        let tag_collection = rule.tags();
        let rule_tag_set: HashSet<&Tag> = HashSet::from_iter(tag_collection.iter());
        let cli_tag_set: HashSet<&Tag> = HashSet::from_iter(tags.iter());
        Some(!rule_tag_set.is_disjoint(&cli_tag_set))
    }
}

impl Runner<Inspect> {
    pub fn advance(self) -> Runner<HandleConflicts> {
        let entries = self.extra.entries();

        Runner::<HandleConflicts> {
            config: self.config,
            extra: HandleConflicts::with_entries(entries),
        }
    }

    pub fn preview_entries(&self) {
        self.extra.print_entries();
    }
}

impl Runner<HandleConflicts> {}
