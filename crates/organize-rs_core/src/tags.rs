//! tags

use displaydoc::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Display, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct TagCollection(Vec<Tag>);

impl std::ops::DerefMut for TagCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for TagCollection {
    fn default() -> Self {
        Self(vec![Tag::default()])
    }
}

impl std::ops::Deref for TagCollection {
    type Target = Vec<Tag>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Tags that can be applied to rules
#[derive(Debug, Clone, Deserialize, Serialize, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tag {
    /// Always run filters/actions with this tag
    Always,
    /// Custom tag for running filters/actions
    Custom(String),
    /// Never run filters/actions with this tag
    Never,
}

impl Default for Tag {
    fn default() -> Self {
        Self::Always
    }
}

impl Tag {
    /// Returns `true` if the organize tag is [`Always`].
    ///
    /// [`Always`]: OrganizeTag::Always
    #[must_use]
    pub fn is_always(&self) -> bool {
        matches!(self, Self::Always)
    }

    /// Returns `true` if the organize tag is [`Never`].
    ///
    /// [`Never`]: OrganizeTag::Never
    #[must_use]
    pub fn is_never(&self) -> bool {
        matches!(self, Self::Never)
    }

    /// Returns `true` if the organize tag is [`Custom`].
    ///
    /// [`Custom`]: OrganizeTag::Custom
    #[must_use]
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(..))
    }

    pub fn as_custom(&self) -> Option<&String> {
        if let Self::Custom(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_custom(self) -> Result<String, Self> {
        if let Self::Custom(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}
