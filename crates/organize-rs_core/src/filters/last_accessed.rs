#[cfg(feature = "cli")]
use clap::Args;
use filetime::FileTime;
use serde::{Deserialize, Serialize};

use crate::{
    filters::{impl_::matches_date, Filter},
    parsers::period_range::PeriodRange,
};

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(transparent)]
pub struct LastAccessedArgs {
    /// This filter uses the `range` syntax (always inclusive) of Rust.
    /// ..7d => in the last 7 days; 2mo.. => older than 2 months and onwards; 1d..2d =>
    /// between 1 to 2 days old. Left and right boundary need to have the same units.
    /// [possible values: y, mo, w, d, h, m, s]
    ///
    /// **NOTE**: You can one of `['y', 'mo', 'w', 'd', 'h', 'm', 's']`. They
    /// will be **converted** to `seconds` accordingly and are **case-insensitive**.
    #[cfg_attr(feature = "cli", arg(long, value_parser = clap::value_parser!(PeriodRange)))]
    range: Option<PeriodRange>,
}

impl Filter for LastAccessedArgs {
    fn apply(&self, entry: &jwalk::DirEntry<((), ())>) -> bool {
        entry.metadata().ok().map_or(false, |metadata| {
            let atime = FileTime::from_last_access_time(&metadata);
            matches_date(atime.seconds(), &self.range.clone())
        })
    }
}
