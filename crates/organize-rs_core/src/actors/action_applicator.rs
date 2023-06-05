use crate::actions::ActionApplicationCollection;

#[derive(Debug, Default)]
pub struct ActionApplicator {
    actions: ActionApplicationCollection,
}

impl ActionApplicator {
    pub fn new(actions: ActionApplicationCollection) -> Self {
        ActionApplicator { actions }
    }
}
