#[cfg(feature = "cli")]
use clap::Args;
use displaydoc::Display;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct NameArgs {
    #[cfg_attr(feature = "cli", command(flatten))]
    #[serde(flatten)]
    arguments: NameFilterArgs,
    /// By default, the matching is case sensitive.
    ///
    /// Change this to `False` to use case insensitive matching.
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(rename = "case_insensitive")]
    #[serde(default = "bool::default")]
    case_insensitive: bool,
}

/// Arguments for `name` filter
#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Display, Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "cli", group(required = true, multiple = false))]
pub struct NameFilterArgs {
    // TODO: Not implemented, searching for alternatives
    /// A matching string in [simplematch-syntax](https://github.com/tfeldmann/simplematch)
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(default = "Option::default")]
    simple_match: Option<Vec<String>>,
    /// The filename must begin with the given string
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(default = "Option::default")]
    starts_with: Option<Vec<String>>,
    /// The filename must contain the given string
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(default = "Option::default")]
    contains: Option<Vec<String>>,
    /// The filename (without extension) must end with the given string
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(default = "Option::default")]
    ends_with: Option<Vec<String>>,
}
