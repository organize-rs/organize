use crate::filters::Filter;

#[cfg(feature = "cli")]
use clap::Args;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct EmptyArgs;

impl Filter for EmptyArgs {
    fn apply(&self, entry: &jwalk::DirEntry<((), ())>) -> bool {
        entry.metadata().map_or(false, |meta_data| {
            if entry.path().is_file() {
                meta_data.len() == 0
            } else if entry.path().is_dir() {
                entry.path().read_dir().map_or(false, |f| f.count() == 0)
            } else {
                false
            }
        })
    }
}
