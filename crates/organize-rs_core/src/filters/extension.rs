#[cfg(feature = "cli")]
use clap::Args;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct ExtensionArgs {
    /// The file extensions to match (without dot)
    #[cfg_attr(feature = "cli", arg(long))]
    exts: Vec<String>,
}
