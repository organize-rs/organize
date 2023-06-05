use std::ops::Not;

use filetime::FileTime;
use itertools::Itertools;
use jwalk::{ClientState, DirEntry};

use crate::{
    error::FilterErrorKind,
    filters::{
        all_items::AllItemsArgs, name::NameFilterArgs, CullKind, DateUnitKind, Filter,
        FilterApplicationKind, FilterCollection, FilterGroup, FilterGroupOperationKind, FilterKind,
        FilterOperationKind, RecursiveFilterArgs,
    },
    parsers::{period_range::PeriodRange, size_range::SizeRange},
};

impl FilterKind {
    pub fn get_filter(&self) -> Box<dyn Filter> {
        match self {
            FilterKind::NoFilter(arg) => Box::new(arg.to_owned()),
            FilterKind::AllItems(arg) => Box::new(arg.to_owned()),
            FilterKind::IgnorePath(arg) => Box::new(arg.to_owned()),
            FilterKind::IgnoreName(arg) => Box::new(arg.to_owned()),
            FilterKind::Extension(arg) => Box::new(arg.to_owned()),
            FilterKind::Name(arg) => Box::new(arg.to_owned()),
            FilterKind::Empty(arg) => Box::new(arg.to_owned()),
            FilterKind::Created(arg) => Box::new(arg.to_owned()),
            FilterKind::LastModified(arg) => Box::new(arg.to_owned()),
            FilterKind::LastAccessed(arg) => Box::new(arg.to_owned()),
            FilterKind::Mimetype(arg) => Box::new(arg.to_owned()),
            FilterKind::Size(arg) => Box::new(arg.to_owned()),
            FilterKind::Regex(arg) => todo!("not implemented (yet)!"),
            FilterKind::Exif(arg) => todo!("not implemented (yet)!"),
            FilterKind::FileContent(arg) => todo!("not implemented (yet)!"),
            FilterKind::Duplicate(arg) => todo!("not implemented (yet)!"),
            #[cfg(target_os = "osx")]
            FilterKind::Added(arg) => todo!("not implemented (yet)!"),
            #[cfg(target_os = "osx")]
            FilterKind::LastUsed(arg) => todo!("not implemented (yet)!"),
        }
    }

    /// Returns `true` if the organize filter is [`IgnoreName`].
    ///
    /// [`IgnoreName`]: OrganizeFilter::IgnoreName
    #[must_use]
    pub fn is_ignore_name(&self) -> bool {
        matches!(self, Self::IgnoreName(_))
    }

    /// Returns `true` if the organize filter is [`IgnorePath`].
    ///
    /// [`IgnorePath`]: OrganizeFilter::IgnorePath
    #[must_use]
    pub fn is_ignore_path(&self) -> bool {
        matches!(self, Self::IgnorePath(_))
    }

    #[must_use]
    pub fn is_bump(f: Option<CullKind>) -> bool {
        matches!(f, Some(CullKind::Bump))
    }
}

impl RecursiveFilterArgs {
    pub fn recursive(&self) -> bool {
        self.recursive
    }

    pub fn max_depth(&self) -> u64 {
        self.max_depth
    }
}

impl DateUnitKind {
    pub fn into_seconds(&self) -> f64 {
        match self {
            DateUnitKind::Years(y) => *y * 52f64 * 7f64 * 24f64 * 60f64 * 60f64,
            DateUnitKind::Months(mo) => *mo * 4f64 * 7f64 * 24f64 * 60f64 * 60f64,
            DateUnitKind::Weeks(w) => *w * 7f64 * 24f64 * 60f64 * 60f64,
            DateUnitKind::Days(d) => *d * 24f64 * 60f64 * 60f64,
            DateUnitKind::Hours(h) => *h * 60f64 * 60f64,
            DateUnitKind::Minutes(m) => *m * 60f64,
            DateUnitKind::Seconds(s) => *s,
        }
    }
}

impl<T> FilterOperationKind<T> {
    /// Returns `true` if the apply or negate filter is [`Apply`].
    ///
    /// [`Apply`]: ApplyOrNegateFilter::Apply
    #[must_use]
    pub fn is_apply(&self) -> bool {
        matches!(self, Self::Apply(..))
    }

    pub fn as_apply(&self) -> Option<&T> {
        if let Self::Apply(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_apply(self) -> Result<T, Self> {
        if let Self::Apply(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the apply or negate filter is [`Negate`].
    ///
    /// [`Negate`]: ApplyOrNegateFilter::Negate
    #[must_use]
    pub fn is_invert(&self) -> bool {
        matches!(self, Self::Invert(..))
    }

    pub fn as_invert(&self) -> Option<&T> {
        if let Self::Invert(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_invert(self) -> Result<T, Self> {
        if let Self::Invert(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

impl FilterCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_vec(
        filter_collection: Vec<(FilterApplicationKind, FilterOperationKind<FilterKind>)>,
    ) -> Self {
        Self(filter_collection)
    }

    pub fn decompose(self) -> Vec<(FilterApplicationKind, FilterOperationKind<FilterKind>)> {
        self.0
    }

    pub fn push(
        &mut self,
        filter_collection: (FilterApplicationKind, FilterOperationKind<FilterKind>),
    ) {
        self.0.push(filter_collection)
    }
}

impl FilterApplicationKind {
    /// Returns `true` if the organize filter mode is [`All`].
    ///
    /// [`All`]: OrganizeFilterMode::All
    #[must_use]
    pub fn is_all(&self) -> bool {
        matches!(self, Self::All)
    }

    /// Returns `true` if the organize filter mode is [`Any`].
    ///
    /// [`Any`]: OrganizeFilterMode::Any
    #[must_use]
    pub fn is_any(&self) -> bool {
        matches!(self, Self::Any)
    }

    /// Returns `true` if the organize filter mode is [`None`].
    ///
    /// [`None`]: OrganizeFilterMode::None
    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl FilterGroup<Vec<FilterKind>> {
    pub fn set_mode(&mut self, mode: FilterApplicationKind) {
        self.mode = mode;
    }

    pub fn set_apply(&mut self, apply: FilterGroupOperationKind) {
        self.operation = apply;
    }

    pub fn set_filters(&mut self, filters: Vec<FilterKind>) {
        self.filters = filters;
    }
}

impl<T> FilterGroup<T> {
    pub fn new(apply: FilterGroupOperationKind, mode: FilterApplicationKind, filters: T) -> Self {
        Self {
            operation: apply,
            mode,
            filters,
        }
    }

    pub fn apply(&self) -> FilterGroupOperationKind {
        self.operation
    }

    pub fn mode(&self) -> FilterApplicationKind {
        self.mode
    }

    pub fn filters(&self) -> &T {
        &self.filters
    }
}

pub fn matches_date(item_date_secs: i64, range: &Option<PeriodRange>) -> bool {
    let Some(range) = range else {
            return false
        };

    let now = FileTime::now();
    let seconds_since_created = match u64::try_from(now.seconds() - item_date_secs) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("subtraction of two datetimes can't be negative: {err}");
            return false;
        }
    } as f64;

    range.in_range(seconds_since_created)
}
