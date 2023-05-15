//! Aliases for folders that can be used in the config file
//! to keep it DRY.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Alias {
    name: String,
    folders: Vec<String>,
}
