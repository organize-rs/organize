use crate::filters::Filter;
#[cfg(feature = "cli")]
use clap::Args;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct NoFilterArgs;

impl Filter for NoFilterArgs {
    fn apply(&self, _entry: &jwalk::DirEntry<((), ())>) -> bool {
        false
    }
}
