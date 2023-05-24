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
pub enum OrganizeTarget {
    /// operate only on directories
    Dirs,
    /// operate only on files
    Files,
    /// operate on both
    Both,
}

impl Default for OrganizeTarget {
    fn default() -> Self {
        Self::Files
    }
}

impl OrganizeTarget {
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

#[derive(Debug, Clone, Deserialize, Serialize, Copy)]
pub struct MaxDepth(u64);

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
pub enum OrganizeLocation {
    /// Recursive discovery of directory entries
    RecursiveWithMaxDepth {
        /// path to the location that should be filtered
        path: PathBuf,
        /// maximum recursion depth
        max_depth: MaxDepth,
        /// when targets is set to dirs, organize will work on
        /// the folders, not on files
        target: OrganizeTarget,
    },
    /// Non-recursive discovery of directory entries
    NonRecursive {
        /// path to the location that should be filtered
        path: PathBuf,
        /// when targets is set to dirs, organize will work on
        /// the folders, not on files
        target: OrganizeTarget,
    },
}

impl Display for OrganizeLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrganizeLocation::RecursiveWithMaxDepth {
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
            OrganizeLocation::NonRecursive { path, target } => write!(
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

impl From<(PathBuf, OrganizeTarget)> for OrganizeLocation {
    fn from(value: (PathBuf, OrganizeTarget)) -> Self {
        Self::NonRecursive {
            path: value.0,
            target: value.1,
        }
    }
}

impl From<(PathBuf, MaxDepth, OrganizeTarget)> for OrganizeLocation {
    fn from(value: (PathBuf, MaxDepth, OrganizeTarget)) -> Self {
        Self::RecursiveWithMaxDepth {
            path: value.0,
            max_depth: value.1,
            target: value.2,
        }
    }
}
