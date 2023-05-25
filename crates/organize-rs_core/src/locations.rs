use std::{fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};

#[cfg(feature = "cli")]
use clap::ValueEnum;

use displaydoc::Display;

/// [`OrganizeTarget`] defines targets `organize` operates on.
///
/// When targets is set to dirs, organize will work on
/// the folders, not on files.
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Deserialize, Serialize, Display, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetKind {
    /// operate on both
    Both,
    /// operate only on directories
    Dirs,
    /// operate only on files
    Files,
}

impl Default for TargetKind {
    fn default() -> Self {
        Self::Files
    }
}

impl TargetKind {
    /// Returns `true` if the organize targets is [`Files`].
    ///
    /// [`Files`]: OrganizeTargets::Files
    #[must_use]
    pub fn is_files(&self) -> bool {
        matches!(self, Self::Files)
    }

    /// Returns `true` if the organize targets is [`Dirs`].
    ///
    /// [`Dirs`]: OrganizeTargets::Dirs
    #[must_use]
    pub fn is_dirs(&self) -> bool {
        matches!(self, Self::Dirs)
    }
}

#[derive(Debug, Clone, Deserialize, Copy)]
pub struct MaxDepth(u64);

impl Serialize for MaxDepth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

impl Display for MaxDepth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaxDepth({})", self.0)
    }
}

impl MaxDepth {
    pub fn new(value: u64) -> Self {
        MaxDepth(value)
    }
}

impl std::ops::DerefMut for MaxDepth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for MaxDepth {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for MaxDepth {
    fn default() -> Self {
        Self(1)
    }
}
/// [`OrganizeLocation] contains the directories and files
/// organize should include in the entry discovery
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LocationKind {
    /// Non-recursive discovery of directory entries
    NonRecursive {
        /// path to the location that should be filtered
        path: PathBuf,
        /// when targets is set to dirs, organize will work on
        /// the folders, not on files
        target: TargetKind,
    },
    /// Recursive discovery of directory entries
    RecursiveWithMaxDepth {
        /// path to the location that should be filtered
        path: PathBuf,
        /// maximum recursion depth
        max_depth: MaxDepth,
        /// when targets is set to dirs, organize will work on
        /// the folders, not on files
        target: TargetKind,
    },
}

impl Display for LocationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationKind::RecursiveWithMaxDepth {
                path,
                max_depth,
                target,
            } => {
                write!(
                    f,
                    "
    Recursive Location with max-depth
        location: {}
        max-depth: {}
        target: {}
            ",
                    path.display(),
                    max_depth,
                    target
                )
            }
            LocationKind::NonRecursive { path, target } => write!(
                f,
                "
    Non-Recursive Location
        location: {}
        target: {}
            ",
                path.display(),
                target
            ),
        }
    }
}

impl From<(PathBuf, TargetKind)> for LocationKind {
    fn from(value: (PathBuf, TargetKind)) -> Self {
        Self::NonRecursive {
            path: value.0,
            target: value.1,
        }
    }
}

impl From<(PathBuf, MaxDepth, TargetKind)> for LocationKind {
    fn from(value: (PathBuf, MaxDepth, TargetKind)) -> Self {
        Self::RecursiveWithMaxDepth {
            path: value.0,
            max_depth: value.1,
            target: value.2,
        }
    }
}
