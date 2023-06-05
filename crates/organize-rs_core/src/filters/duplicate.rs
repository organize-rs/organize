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

impl DuplicateKind {
    /// Returns `true` if the detect duplicate by is [`FirstSeen`].
    ///
    /// [`FirstSeen`]: DetectDuplicateBy::FirstSeen
    #[must_use]
    pub fn is_first_seen(&self) -> bool {
        matches!(self, Self::FirstSeen)
    }

    /// Returns `true` if the detect duplicate by is [`Name`].
    ///
    /// [`Name`]: DetectDuplicateBy::Name
    #[must_use]
    pub fn is_name(&self) -> bool {
        matches!(self, Self::Name)
    }

    /// Returns `true` if the detect duplicate by is [`Created`].
    ///
    /// [`Created`]: DetectDuplicateBy::Created
    #[must_use]
    pub fn is_created(&self) -> bool {
        matches!(self, Self::Created)
    }

    /// Returns `true` if the detect duplicate by is [`LastModified`].
    ///
    /// [`LastModified`]: DetectDuplicateBy::LastModified
    #[must_use]
    pub fn is_last_modified(&self) -> bool {
        matches!(self, Self::LastModified)
    }
}
