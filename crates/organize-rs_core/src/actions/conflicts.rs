use displaydoc::Display;
use serde::{Deserialize, Serialize};

#[cfg(feature = "cli")]
use clap::ValueEnum;

/// Actions for conflict resolution
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Display)]
pub enum OnConflictKind {
    /// Keep the biggest item
    Biggest,
    /// Keep the newest item
    KeepNewer,
    /// Keep the oldest item
    KeepOlder,
    /// Move the item to a folder for further inspection
    MoveToFolder,
    /// Overwrite the item
    Overwrite,
    /// Overwrite the item if it is empty
    OverwriteEmpty,
    /// Rename the initial item
    RenameExisting,
    /// Rename the newly created item
    RenameNew,
    /// Skip the item
    Skip,
    /// Keep the smallest item
    Smallest,
    /// Move the item to trash
    Trash,
}

impl OnConflictKind {
    /// Returns `true` if [`OnConflict`] is [`Skip`].
    ///
    /// [`Skip`]: OnConflict::Skip
    #[must_use]
    pub fn is_skip(&self) -> bool {
        matches!(self, Self::Skip)
    }

    /// Returns `true` if [`OnConflict`] is [`Overwrite`].
    ///
    /// [`Overwrite`]: OnConflict::Overwrite
    #[must_use]
    pub fn is_overwrite(&self) -> bool {
        matches!(self, Self::Overwrite)
    }

    /// Returns `true` if [`OnConflict`] is [`Trash`].
    ///
    /// [`Trash`]: OnConflict::Trash
    #[must_use]
    pub fn is_trash(&self) -> bool {
        matches!(self, Self::Trash)
    }

    /// Returns `true` if [`OnConflict`] is [`RenameNew`].
    ///
    /// [`RenameNew`]: OnConflict::RenameNew
    #[must_use]
    pub fn is_rename_new(&self) -> bool {
        matches!(self, Self::RenameNew)
    }

    /// Returns `true` if [`OnConflict`] is [`RenameExisting`].
    ///
    /// [`RenameExisting`]: OnConflict::RenameExisting
    #[must_use]
    pub fn is_rename_existing(&self) -> bool {
        matches!(self, Self::RenameExisting)
    }
}

impl Default for OnConflictKind {
    fn default() -> Self {
        Self::Skip
    }
}
