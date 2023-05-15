//! Aliases for folders that can be used in the config file
//! to keep it DRY.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Alias {
    name: String,
    folders: Vec<String>,
}

impl Alias {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn folders(&self) -> &[String] {
        self.folders.as_ref()
    }

    pub fn len(&self) -> usize {
        self.folders.len()
    }
}
