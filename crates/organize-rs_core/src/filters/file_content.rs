#[cfg(feature = "cli")]
use clap::Args;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct FileContentArgs {
    #[cfg_attr(feature = "cli", arg(long))]
    expr: String,
}
