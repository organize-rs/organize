#[cfg(feature = "cli")]
use clap::Args;
use serde::{Deserialize, Serialize};

use crate::filters::Filter;

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AllItemsArgs {
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(default = "bool::default")]
    i_agree_it_is_dangerous: bool,
}

impl Filter for AllItemsArgs {
    fn apply(&self, entry: &jwalk::DirEntry<((), ())>) -> bool {
        self.i_agree_it_is_dangerous.to_owned()
    }
}
