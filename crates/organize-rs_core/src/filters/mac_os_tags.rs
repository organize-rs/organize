#[cfg(feature = "cli")]
use clap::Args;
use serde::{Deserialize, Serialize};
use serde_with::formats::CommaSeparator;
use serde_with::serde_as;
use serde_with::StringWithSeparator;

#[cfg_attr(feature = "cli", derive(Args))]
#[serde_as]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct MacOsTagsArgs {
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    tags: Vec<String>,
}
