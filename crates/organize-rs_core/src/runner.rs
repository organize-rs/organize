use std::path::Path;

use crate::{
    concurrency::OffThreadExt,
    config::OrganizeConfig,
    rules::{self, Rule},
    state::{EntriesCollected, Init, ProcessingState, Start},
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
    pub fn run(self) -> Runner<EntriesCollected> {
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

        Runner::<EntriesCollected> {
            config: self.config,
            extra: EntriesCollected::with_entries(entries),
        }
    }
}

impl Runner<EntriesCollected> {
    pub fn print_entries(&self) {
        self.extra.print_entries();
    }
}
