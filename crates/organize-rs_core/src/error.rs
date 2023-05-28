//! errors

use displaydoc::Display;
use std::{borrow::Cow, error::Error as StdError};
use thiserror::Error as ThisError;

/// Result type often returned from methods that can have rustic `Error`s.
pub type OrganizeResult<T> = std::result::Result<T, OrganizeError>;

// [`Error`] is public, but opaque and easy to keep compatible.
#[derive(ThisError, Debug)]
#[error(transparent)]
pub struct OrganizeError(#[from] OrganizeErrorKind);

// Accessors for anything we do want to expose publicly.
impl OrganizeError {
    pub fn into_inner(self) -> OrganizeErrorKind {
        self.0
    }
}
// Private (pub(crate)) and free to change across minor version of the crate.
#[non_exhaustive]
#[derive(Debug, ThisError)]
pub enum OrganizeErrorKind {
    /// [`WalkerErrorKind`] describes the errors that can be returned for a walking through the files
    #[error(transparent)]
    Walker(#[from] WalkerErrorKind),
    /// [`FilterErrorKind`] describes the errors that can happen while dealing with filter functions/closures
    #[error(transparent)]
    Filter(#[from] FilterErrorKind),
    /// [`ActionErrorKind`] describes the errors that can happen while dealing with action functions/closures
    #[error(transparent)]
    Action(#[from] ActionErrorKind),
    /// [`ConfigErrorKind`] describes the errors that can happen while dealing with config functions/closures
    #[error(transparent)]
    Config(#[from] ConfigErrorKind),
    /// [`std::io::Error`]
    #[error(transparent)]
    StdIo(#[from] std::io::Error),
}

/// [`WalkerErrorKind`] describes the errors that can be returned for a walking through the files
#[derive(ThisError, Debug, Display)]
pub enum WalkerErrorKind {
    /// failed to convert numbers: {0}
    FailedToConvertNumbers(std::num::TryFromIntError),
}

/// [`FilterErrorKind`] describes the errors that can be returned for a filter
#[derive(ThisError, Debug, Display)]
pub enum FilterErrorKind {
    /// extension is not extractable from file: {0:?}
    ExtensionNotExtractable(std::path::PathBuf),
    /// discovered and inverted item: {0}
    InvertedItem(String),
}

/// [`ActionErrorKind`] describes the errors that can be returned for an action
#[derive(ThisError, Debug, Display)]
pub enum ActionErrorKind {
    /// failed to open file: {0}
    FailedToOpenFile(#[from] std::io::Error),
}

/// [`ConfigErrorKind`] describes the errors that can be returned for configs
#[derive(ThisError, Debug, Display)]
pub enum ConfigErrorKind {
    /// parsing the config file failed: {0}
    FailedToParseConfigFile(#[from] serde_yaml::Error),
}

trait ErrorMarker: StdError {}

impl ErrorMarker for WalkerErrorKind {}
impl ErrorMarker for FilterErrorKind {}
impl ErrorMarker for ActionErrorKind {}
impl ErrorMarker for ConfigErrorKind {}

impl<E> From<E> for OrganizeError
where
    E: ErrorMarker,
    OrganizeErrorKind: From<E>,
{
    fn from(value: E) -> Self {
        Self(OrganizeErrorKind::from(value))
    }
}
