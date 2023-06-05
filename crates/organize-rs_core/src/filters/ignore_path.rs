use std::ops::Not;

#[cfg(feature = "cli")]
use clap::Args;
use serde::{Deserialize, Serialize};
use serde_with::formats::CommaSeparator;
use serde_with::serde_as;
use serde_with::StringWithSeparator;

use crate::filters::Filter;

#[cfg_attr(feature = "cli", derive(Args))]
#[serde_as]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(transparent)]
pub struct IgnorePathArgs {
    /// Matches for these Strings in the whole Path
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    in_path: Vec<String>,
}

impl Filter for IgnorePathArgs {
    fn apply(&self, entry: &jwalk::DirEntry<((), ())>) -> bool {
        // this filter is negated, meaning, that if a keyword is found
        // this filter tells us to not include the file
        entry.path().to_str().map_or(true, |path| {
            self.in_path
                .iter()
                .any(|pat| path.to_lowercase().contains(&pat.to_lowercase()))
                .not()
        })
    }
}
