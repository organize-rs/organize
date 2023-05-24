use displaydoc::Display;
use std::error::Error as StdError;
use thiserror::Error as ThisError;

pub type OrganizeResult<T> = std::result::Result<T, Box<dyn StdError>>;

#[derive(Debug, Clone, ThisError, Display)]
pub enum ErrorKind {
    /// extension is not extractable from file: {0:?}
    ExtensionNotExtractable(std::path::PathBuf),
}
