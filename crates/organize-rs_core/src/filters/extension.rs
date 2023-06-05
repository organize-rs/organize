#[cfg(feature = "cli")]
use clap::Args;
use serde::{Deserialize, Serialize};

use crate::filters::Filter;

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(transparent)]
pub struct ExtensionArgs {
    /// The file extensions to match (without dot)
    #[cfg_attr(feature = "cli", arg(long))]
    exts: Vec<String>,
}

impl Filter for ExtensionArgs {
    fn apply(&self, entry: &jwalk::DirEntry<((), ())>) -> bool {
        entry
            .path()
            .extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |extension_str| {
                self.exts.iter().any(|f| {
                    if f.starts_with('.') {
                        f.strip_prefix('.')
                            .map_or_else(|| false, |f| extension_str == f)
                    } else {
                        f == extension_str
                    }
                })
            })
    }
}
