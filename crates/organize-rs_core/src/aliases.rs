//! Aliases for folders that can be used in the config file
//! to keep it DRY.

use serde::{Deserialize, Serialize};

use crate::aliases;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ItemKind {
    #[serde(rename = "extension")]
    Extension,
    #[serde(rename = "folder")]
    Location,
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
        matches!(self, Self::Location)
    }
}

impl Default for ItemKind {
    fn default() -> Self {
        Self::Location
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(transparent)]
pub struct Aliases(Vec<Alias>);

impl Aliases {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_aliases(aliases: Vec<Alias>) -> Self {
        Self(aliases)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Alias {
    name: String,
    kind: ItemKind,
    items: Vec<String>,
}

impl Alias {
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

#[cfg(test)]
mod tests {

    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn test_yaml_aliases_parsing_passed() {
        let yaml = r#"
        - name: downloads_folder
          kind: folder
          items:
            - ~/Downloads/
        - name: img_ext
          kind: extension
          items:
            - png
            - jpg
            - jpeg
            - gif
            - svg
            - jfif
            - tiff
            - bmp
            - webp
        "#;

        let _aliases: Aliases = serde_yaml::from_str(yaml).unwrap();
    }

    #[test]
    fn test_yaml_aliases_serialization_passes() {
        let alias = Alias {
            name: "Config extensions".to_string(),
            kind: ItemKind::Extension,
            items: vec!["toml".to_string(), "json".to_string()],
        };

        let alias2 = Alias {
            name: "Image extensions".to_string(),
            kind: ItemKind::Extension,
            items: vec!["jpg".to_string(), "png".to_string()],
        };

        let aliases = Aliases::with_aliases(vec![alias, alias2]);

        assert_yaml_snapshot!(aliases, @r###"
        ---
        - name: Config extensions
          kind: extension
          items:
            - toml
            - json
        - name: Image extensions
          kind: extension
          items:
            - jpg
            - png
        "###);
    }
}
