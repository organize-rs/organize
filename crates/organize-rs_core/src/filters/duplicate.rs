#[cfg(feature = "cli")]
use clap::{Args, ValueEnum};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DuplicateArgs {
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(default = "DuplicateKind::default")]
    detect_original_by: DuplicateKind,
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(default = "bool::default")]
    reverse: bool,
}

/// Duplication detection
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DuplicateKind {
    /// The first entry sorted by creation date is the original.
    #[serde(rename = "created")]
    Created,
    /// Whatever file is visited first is the original.
    ///
    /// This depends on the order of your location entries.
    #[serde(rename = "first_seen")]
    FirstSeen,
    // TODO
    #[serde(rename = "hash")]
    Hash,
    /// The first file sorted by date of last modification is the original.
    #[serde(rename = "last_modified")]
    LastModified,
    /// The first entry sorted by name is the original.
    #[serde(rename = "name")]
    Name,
}

impl Default for DuplicateKind {
    fn default() -> Self {
        Self::Name
    }
}
