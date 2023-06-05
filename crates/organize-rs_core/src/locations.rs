//! locations

use std::{fmt::Display, fs::FileType, path::PathBuf};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[cfg(feature = "cli")]
use clap::ValueEnum;

use displaydoc::Display;

use crate::error::{OrganizeResult, WalkerErrorKind};

pub mod constants {
    pub const DEFAULT_MAX_DEPTH: usize = 0;
}

pub trait DirWalk {
    fn populate_entries(&self) -> OrganizeResult<Vec<jwalk::DirEntry<((), ())>>>;
}

#[derive(Debug, Clone, Deserialize, Serialize, Display, Default)]
#[serde(transparent)]
pub struct LocationCollection(Vec<LocationKind>);

impl LocationCollection {
    pub fn from_vec(vec: Vec<LocationKind>) -> Self {
        Self(vec)
    }
}

impl std::ops::DerefMut for LocationCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for LocationCollection {
    type Target = Vec<LocationKind>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [`OrganizeTarget`] defines targets `organize` operates on.
///
/// When targets is set to dirs, organize will work on
/// the folders, not on files.
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(
    Debug, Clone, Deserialize, Serialize, Display, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum TargetKind {
    /// operate on both
    #[serde(rename = "both")]
    Both,
    /// operate only on directories
    #[serde(rename = "folders")]
    Directories,
    /// operate only on files
    #[serde(rename = "files")]
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
        matches!(self, Self::Directories)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Copy, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct MaxDepth(pub(crate) u64);

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
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum LocationKind {
    /// Non-recursive discovery of directory entries
    #[serde(rename = "non_recursive")]
    NonRecursive(NonRecursiveArgs),
    /// Recursive discovery of directory entries
    #[serde(rename = "recursive")]
    RecursiveWithMaxDepth(RecursiveWithMaxDepthArgs),
    /// Just a bare path, takes default settings
    ///
    /// Default settings:
    ///
    /// max_depth: 1
    /// target: files
    #[serde(rename = "default_settings")]
    BarePath(BarePathArgs),
}

impl LocationKind {
    pub fn get_location(self) -> Box<dyn DirWalk> {
        match self {
            LocationKind::NonRecursive(arg) => Box::new(arg),
            LocationKind::RecursiveWithMaxDepth(arg) => Box::new(arg),
            LocationKind::BarePath(arg) => Box::new(arg),
        }
    }
}

impl Default for LocationKind {
    fn default() -> Self {
        Self::NonRecursive(NonRecursiveArgs {
            path: PathBuf::default(),
            target: TargetKind::default(),
        })
    }
}

impl Display for LocationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationKind::RecursiveWithMaxDepth(RecursiveWithMaxDepthArgs {
                path,
                max_depth,
                target,
            }) => {
                write!(
                    f,
                    "
    Recursive Location with max-depth
        location: {}
        max-depth: {max_depth}
        target: {target}
            ",
                    path.display()
                )
            }
            LocationKind::NonRecursive(NonRecursiveArgs { path, target }) => write!(
                f,
                "
    Non-Recursive Location
        location: {}
        target: {target}
            ",
                path.display()
            ),
            LocationKind::BarePath(BarePathArgs { path }) => write!(
                f,
                "
    Bare Location
        location: {}
        target: {}
            ",
                path.display(),
                TargetKind::default()
            ),
        }
    }
}

impl From<(PathBuf, TargetKind)> for LocationKind {
    fn from(value: (PathBuf, TargetKind)) -> Self {
        Self::NonRecursive(NonRecursiveArgs {
            path: value.0,
            target: value.1,
        })
    }
}

impl From<(PathBuf, MaxDepth, TargetKind)> for LocationKind {
    fn from(value: (PathBuf, MaxDepth, TargetKind)) -> Self {
        Self::RecursiveWithMaxDepth(RecursiveWithMaxDepthArgs {
            path: value.0,
            max_depth: value.1,
            target: value.2,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Display)]
pub struct BarePathArgs {
    /// path to the location that should be filtered
    path: PathBuf,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Display)]
pub struct NonRecursiveArgs {
    /// path to the location that should be filtered
    path: PathBuf,
    /// when targets is set to dirs, organize will work on
    /// the folders, not on files
    target: TargetKind,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Display)]
pub struct RecursiveWithMaxDepthArgs {
    /// path to the location that should be filtered
    path: PathBuf,
    /// maximum recursion depth
    max_depth: MaxDepth,
    /// when targets is set to dirs, organize will work on
    /// the folders, not on files
    target: TargetKind,
}

impl DirWalk for RecursiveWithMaxDepthArgs {
    fn populate_entries(&self) -> OrganizeResult<Vec<jwalk::DirEntry<((), ())>>> {
        let depth = if let Some(max_depth) = self.max_depth.into() {
            usize::try_from(*max_depth).map_err(WalkerErrorKind::FailedToConvertNumbers)?
        } else {
            constants::DEFAULT_MAX_DEPTH
        };

        // TODO: Initialize indicatif progress bar

        let files = jwalk::WalkDir::new(self.path)
            .max_depth(depth)
            .into_iter()
            .flat_map(|f| f.ok())
            .filter(|f| match self.target {
                TargetKind::Directories => FileType::is_dir(&f.file_type()),
                TargetKind::Files => FileType::is_file(&f.file_type()),
                TargetKind::Both => true,
            })
            .collect_vec();

        Ok(files)
    }
}
impl DirWalk for NonRecursiveArgs {
    fn populate_entries(&self) -> OrganizeResult<Vec<jwalk::DirEntry<((), ())>>> {
        // TODO: Initialize indicatif progress bar

        let files = jwalk::WalkDir::new(self.path)
            .max_depth(0)
            .into_iter()
            .flat_map(|f| f.ok())
            .filter(|f| match self.target {
                TargetKind::Directories => FileType::is_dir(&f.file_type()),
                TargetKind::Files => FileType::is_file(&f.file_type()),
                TargetKind::Both => true,
            })
            .collect_vec();

        Ok(files)
    }
}
impl DirWalk for BarePathArgs {
    fn populate_entries(&self) -> OrganizeResult<Vec<jwalk::DirEntry<((), ())>>> {
        // TODO: Initialize indicatif progress bar

        let files = jwalk::WalkDir::new(self.path)
            .max_depth(constants::DEFAULT_MAX_DEPTH)
            .into_iter()
            .flat_map(|f| f.ok())
            .filter(|f| match TargetKind::default() {
                TargetKind::Directories => FileType::is_dir(&f.file_type()),
                TargetKind::Files => FileType::is_file(&f.file_type()),
                TargetKind::Both => true,
            })
            .collect_vec();

        Ok(files)
    }
}
