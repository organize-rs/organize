#[cfg(feature = "cli")]
use clap::Args;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct ExifArgs {
    #[cfg_attr(feature = "cli", arg(long))]
    contains: Vec<String>,
}
