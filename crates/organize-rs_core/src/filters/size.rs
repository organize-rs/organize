#[cfg(feature = "cli")]
use clap::Args;
use serde::{Deserialize, Serialize};

use crate::parsers::size_range::SizeRange;

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct SizeArgs {
    /// This filter uses the `range` syntax (always inclusive) of Rust.
    /// ..11MB => smaller than; 15MB.. => bigger than; 10KB..20MiB =>
    /// bigger than 10 KB, but smaller than 20 MiB
    ///
    /// **NOTE**: You can use `decimal` (metric) and `binary` (IEC)
    /// multiple-byte units. E.g., `KiB` or `KB`, `GB` or `GiB`. They
    /// will be **converted** accordingly and are **case-insensitive**.
    #[cfg_attr(feature = "cli", arg(long, value_parser = clap::value_parser!(SizeRange)))]
    range: Option<SizeRange>,
}
