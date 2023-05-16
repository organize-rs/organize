//! Aliases for folders that can be used in the config file
//! to keep it DRY.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ItemKind {
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "ext")]
    Extension,
}

impl ItemKind {
    /// Returns `true` if the item kind is [`Extension`].
    ///
    /// [`Extension`]: ItemKind::Extension
    #[must_use]
    pub fn is_extension(&self) -> bool {
        matches!(self, Self::Extension)
    }

    /// Returns `true` if the item kind is [`Folder`].
    ///
    /// [`Folder`]: ItemKind::Folder
    #[must_use]
    pub fn is_folder(&self) -> bool {
        matches!(self, Self::Folder)
    }
}

impl Default for ItemKind {
    fn default() -> Self {
        Self::Folder
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Alias(String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Reference {
    name: String,
    kind: ItemKind,
    items: Vec<String>,
}

impl Reference {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn folders(&self) -> &[String] {
        self.items.as_ref()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
}
