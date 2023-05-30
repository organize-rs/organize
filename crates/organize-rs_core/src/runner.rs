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
    walker: Vec<(Rule, FilteredFileWalker)>,
    extra: S,
}

impl Runner<Init> {
    pub fn load(config: OrganizeConfig) -> Runner<Start> {
        Runner::<Start> {
            config,
            walker: vec![],
            extra: Start::default(),
        }
    }
}

impl Runner<Start> {
    pub fn run(mut self) -> Runner<EntriesCollected> {
        self.config.rules().iter().for_each(|rule| {
            let mut walker = FilteredFileWalker::new();
            walker.get_applicable_items(rule.locations(), rule.filters());
            self.walker.push((rule.clone(), walker))
        });

        Runner::<EntriesCollected> {
            config: self.config,
            walker: self.walker,
            extra: EntriesCollected,
        }
    }
}
