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
pub struct IgnoreNameArgs {
    /// Matches for these Strings in the Filename
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    in_name: Vec<String>,
}

impl Filter for IgnoreNameArgs {
    fn apply(&self, entry: &jwalk::DirEntry<((), ())>) -> bool {
        // this filter is negated, meaning, that if a keyword is found
        // this filter tells us to not include the file
        if entry.file_type().is_file() {
            entry.file_name().to_str().map_or(true, |file_name| {
                self.in_name
                    .iter()
                    .any(|pat| file_name.to_lowercase().contains(&pat.to_lowercase()))
                    .not()
            })
        } else {
            true
        }
    }
}
