use std::path::PathBuf;

use displaydoc::Display;
use serde::{Deserialize, Serialize};

#[cfg(feature = "cli")]
use clap::ValueEnum;

/// possible conflicts
#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum ConflictKind {
    /// file already exists
    AlreadyExisting,
}

/// Actions for conflict resolution
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum ConflictResolutionKind {
    /// Keep the biggest item
    #[serde(rename = "biggest")]
    Biggest,
    /// Keep the newest item
    #[serde(rename = "keep_newer")]
    KeepNewer,
    /// Keep the oldest item
    #[serde(rename = "keep_older")]
    KeepOlder,
    /// Move the item to a folder for further inspection
    #[cfg_attr(feature = "cli", clap(skip))]
    #[serde(rename = "move_to")]
    MoveToFolder { path: PathBuf },
    /// Overwrite the item
    #[serde(rename = "overwrite")]
    Overwrite,
    /// Overwrite the item if it is empty
    #[serde(rename = "overwrite_empty")]
    OverwriteEmpty,
    /// Rename the initial item
    #[serde(rename = "rename_existing")]
    RenameExisting,
    /// Rename the newly created item
    #[serde(rename = "rename_new")]
    RenameNew,
    /// Skip the item
    #[serde(rename = "skip")]
    Skip,
    /// Keep the smallest item
    #[serde(rename = "smallest")]
    Smallest,
    /// Move all but the first item discovered to the trash
    #[serde(rename = "trash")]
    Trash,
}

impl Default for ConflictResolutionKind {
    fn default() -> Self {
        Self::Skip
    }
}
