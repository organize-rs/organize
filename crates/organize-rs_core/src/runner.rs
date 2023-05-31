use std::path::Path;

use crate::{
    config::OrganizeConfig,
    state::{HandleConflicts, Init, Inspect, ProcessingState, Start},
    FilteredFileWalker,
};

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
    pub fn apply_filters(self) -> Runner<Inspect> {
        let mut entries = vec![];
        self.config.rules().iter().for_each(|rule| {
            if rule.enabled() {
                let mut walker = FilteredFileWalker::new();
                walker.get_applicable_items(rule.locations(), rule.filters());
                entries.push((rule.clone(), walker))
            } else {
                println!("Not enabled, skipping ... {rule}")
            }
        });

        Runner::<Inspect> {
            config: self.config,
            extra: Inspect::with_entries(entries),
        }
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
