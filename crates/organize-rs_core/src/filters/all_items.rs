#[cfg(feature = "cli")]
use clap::Args;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AllItemsArgs {
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(default = "bool::default")]
    i_agree_it_is_dangerous: bool,
}
