use std::ops::Not;

use filetime::FileTime;
use itertools::Itertools;
use jwalk::{ClientState, DirEntry};

use crate::{
    error::FilterErrorKind,
    filters::{
        CullKind, DateUnitKind, DuplicateKind, FilterApplicationKind, FilterClosure,
        FilterCollection, FilterGroup, FilterGroupOperationKind, FilterKind, FilterOperationKind,
        NameFilterArgs, RecursiveFilterArgs,
    },
    parsers::{PeriodRange, SizeRange},
};

impl FilterKind {
    const NEGATE_STRING: &str = "-|";

    pub fn get_filter<C: ClientState>(&self) -> FilterClosure<C> {
        match self {
            FilterKind::NoFilter => Box::new(|_entry| false),
            FilterKind::AllItems {
                i_agree_it_is_dangerous,
            } => Box::new(|_entry| i_agree_it_is_dangerous.to_owned()),
            FilterKind::IgnorePath { in_path } => self.filter_ignore_str_is_not_in_path(in_path),
            FilterKind::IgnoreName { in_name } => {
                self.filter_ignore_str_is_not_in_file_name(in_name)
            }
            FilterKind::Extension { exts } => self.filter_by_extension(exts),
            FilterKind::Name {
                arguments,
                case_insensitive,
            } => self.filter_by_name(arguments, case_insensitive),
            FilterKind::Empty => self.filter_by_empty(),
            FilterKind::Created { range } => self.filter_by_created(range),
            FilterKind::LastModified { range } => self.filter_by_last_modified(range),
            FilterKind::LastAccessed { range } => self.filter_by_last_accessed(range),
            FilterKind::Mimetype { mime: mimetype } => self.filter_by_mimetype(mimetype),
            FilterKind::Size { range } => self.filter_by_size(range),
            FilterKind::Regex { expr: _ } => todo!("not implemented (yet)!"),
            FilterKind::Exif { contains: _ } => todo!("not implemented (yet)!"),
            FilterKind::FileContent { expr: _ } => todo!("not implemented (yet)!"),
            FilterKind::Duplicate {
                detect_original_by: _,
                reverse: _,
            } => todo!("not implemented (yet)!"),
            #[cfg(target_os = "osx")]
            FilterKind::Added { date, mode } => todo!("not implemented (yet)!"),
            #[cfg(target_os = "osx")]
            FilterKind::LastUsed { date, mode } => todo!("not implemented (yet)!"),
        }
    }

    fn filter_by_extension<'a, 'args, C: ClientState>(
        &'a self,
        exts: &'args [String],
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map_or(false, |extension_str| {
                    exts.iter().any(|f| {
                        if f.starts_with('.') {
                            f.strip_prefix('.')
                                .map_or_else(|| false, |f| extension_str == f)
                        } else {
                            f == extension_str
                        }
                    })
                })
        })
    }

    fn filter_by_name<'a, 'args, C: ClientState>(
        &'a self,
        arguments: &'args NameFilterArgs,
        case_insensitive: &'args bool,
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            let NameFilterArgs {
                simple_match: _, // TODO:  implement simple match filter
                starts_with,
                contains,
                ends_with,
            } = arguments.clone();

            let make_lowercase_if = |string: String| {
                if *case_insensitive {
                    string.to_lowercase()
                } else {
                    string
                }
            };

            // Extract the file stem, if we can't do that, it's senseless to continue
            let Some(Some(file_stem)) = entry
                .path()
                .file_stem()
                .map(|f|{
                    f.to_str().map(|f| f.to_owned())}) else {
                        return false
                    };

            let file_stem = make_lowercase_if(file_stem);

            let to_filter_applikation_kind = |string: String| -> FilterOperationKind<String> {
                if !string.starts_with(Self::NEGATE_STRING) {
                    FilterOperationKind::Apply(string)
                } else {
                    FilterOperationKind::Invert({
                        string
                            .strip_prefix(Self::NEGATE_STRING)
                            .map_or_else(|| string.clone(), |f| f.to_owned())
                    })
                }
            };

            let contains_filter = |wrapped_string: FilterOperationKind<String>| match wrapped_string
            {
                FilterOperationKind::Invert(invert) => match file_stem.contains(&invert) {
                    true => Err(FilterErrorKind::InvertedItem(invert)),
                    false => Ok(false),
                },
                FilterOperationKind::Apply(apply) => Ok(file_stem.contains(&apply)),
            };

            let starts_with_filter =
                |wrapped_string: FilterOperationKind<String>| match wrapped_string {
                    FilterOperationKind::Invert(invert) => match file_stem.starts_with(&invert) {
                        true => Err(FilterErrorKind::InvertedItem(invert)),
                        false => Ok(false),
                    },
                    FilterOperationKind::Apply(apply) => Ok(file_stem.starts_with(&apply)),
                };

            let ends_with_filter =
                |wrapped_string: FilterOperationKind<String>| match wrapped_string {
                    FilterOperationKind::Invert(invert) => match file_stem.ends_with(&invert) {
                        true => Err(FilterErrorKind::InvertedItem(invert)),
                        false => Ok(false),
                    },
                    FilterOperationKind::Apply(apply) => Ok(file_stem.ends_with(&apply)),
                };

            let (contains_oks, contains_errs): (Vec<_>, Vec<_>) = contains
                .into_iter()
                .map(make_lowercase_if)
                .map(to_filter_applikation_kind)
                .map(contains_filter)
                .partition_result();

            let (starts_with_oks, starts_with_errs): (Vec<_>, Vec<_>) = starts_with
                .into_iter()
                .map(make_lowercase_if)
                .map(to_filter_applikation_kind)
                .map(starts_with_filter)
                .partition_result();

            let (ends_with_oks, ends_with_errs): (Vec<_>, Vec<_>) = ends_with
                .into_iter()
                .map(make_lowercase_if)
                .map(to_filter_applikation_kind)
                .map(ends_with_filter)
                .partition_result();

            // return early if we have an item that should be skipped due to being inverted
            if !(ends_with_errs.is_empty() & starts_with_errs.is_empty() & contains_errs.is_empty())
            {
                return false;
            };

            let mut oks = contains_oks;
            oks.extend(starts_with_oks);
            oks.extend(ends_with_oks);

            oks.into_iter().any(|f| f)
        })
    }

    fn filter_by_empty<C: ClientState>(&self) -> Box<dyn FnMut(&DirEntry<C>) -> bool + '_> {
        Box::new(|entry| {
            entry.metadata().map_or(false, |e| {
                if entry.path().is_file() {
                    e.len() == 0
                } else if entry.path().is_dir() {
                    entry.path().read_dir().map_or(false, |f| f.count() == 0)
                } else {
                    false
                }
            })
        })
    }

    fn filter_by_last_accessed<'a, 'args, C: ClientState>(
        &'a self,
        range: &'args Option<PeriodRange>,
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            entry.metadata().ok().map_or(false, |metadata| {
                let atime = FileTime::from_last_access_time(&metadata);
                Self::matches_date(atime.seconds(), &range.clone())
            })
        })
    }
    fn filter_by_last_modified<'a, 'args, C: ClientState>(
        &'a self,
        range: &'args Option<PeriodRange>,
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            entry.metadata().ok().map_or(false, |metadata| {
                let mtime = FileTime::from_last_modification_time(&metadata);
                Self::matches_date(mtime.seconds(), &range.clone())
            })
        })
    }

    fn filter_by_created<'a, 'args, C: ClientState>(
        &'a self,
        range: &'args Option<PeriodRange>,
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            entry.metadata().ok().map_or(false, |metadata| {
                FileTime::from_creation_time(&metadata).map_or(false, |ctime| {
                    Self::matches_date(ctime.seconds(), &range.clone())
                })
            })
        })
    }

    pub(crate) fn matches_date(item_date_secs: i64, range: &Option<PeriodRange>) -> bool {
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

    // TODO Support also  top level media type e.g. `image`
    // TODO to identify all images, for now the unit test
    // TODO is on `should_fail` for that
    fn filter_by_mimetype<'a, 'args, C: ClientState>(
        &'a self,
        mimetype: &'args [String],
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            let Ok(Some(file_kind)) = infer::get_from_path(entry.path()) else { return false };

            let file_mime_type = match file_kind.mime_type().parse::<mime::Mime>() {
                Ok(it) => it,
                Err(err) => {
                    eprintln!(
                        "couldn't determine mimetype of {}: {err}",
                        entry.path().display()
                    );
                    return false;
                }
            };

            mimetype
                .iter()
                .map(|f| f.parse::<mime::Mime>())
                .filter_map(|r| r.map_err(|err| println!("{err}")).ok())
                .any(|f| f == file_mime_type)
        })
    }

    fn filter_by_size<'a, 'args, C: ClientState>(
        &'a self,
        range: &'args Option<SizeRange>,
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            let Some(range) = range.clone() else {
                return false
            };
            entry
                .metadata()
                .map_or(false, |metadata| range.in_range(metadata.len() as f64))
        })
    }

    /// this filter is negated, meaning, that if a keyword is found
    /// this filter tells us to not include the file
    fn filter_ignore_str_is_not_in_file_name<'a, 'args, C: ClientState>(
        &'a self,
        ignore_name: &'args [String],
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            if entry.file_type().is_file() {
                entry.file_name().to_str().map_or(true, |file_name| {
                    ignore_name
                        .iter()
                        .any(|pat| file_name.to_lowercase().contains(&pat.to_lowercase()))
                        .not()
                })
            } else {
                true
            }
        })
    }

    /// this filter is negated, meaning, that if a keyword is found
    /// this filter tells us to not include the file
    fn filter_ignore_str_is_not_in_path<'a, 'args, C: ClientState>(
        &'a self,
        ignore_path: &'args [String],
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            entry.path().to_str().map_or(true, |path| {
                ignore_path
                    .iter()
                    .any(|pat| path.to_lowercase().contains(&pat.to_lowercase()))
                    .not()
            })
        })
    }

    /// Returns `true` if the organize filter is [`IgnoreName`].
    ///
    /// [`IgnoreName`]: OrganizeFilter::IgnoreName
    #[must_use]
    pub fn is_ignore_name(&self) -> bool {
        matches!(self, Self::IgnoreName { .. })
    }

    /// Returns `true` if the organize filter is [`IgnorePath`].
    ///
    /// [`IgnorePath`]: OrganizeFilter::IgnorePath
    #[must_use]
    pub fn is_ignore_path(&self) -> bool {
        matches!(self, Self::IgnorePath { .. })
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

impl DuplicateKind {
    /// Returns `true` if the detect duplicate by is [`FirstSeen`].
    ///
    /// [`FirstSeen`]: DetectDuplicateBy::FirstSeen
    #[must_use]
    pub fn is_first_seen(&self) -> bool {
        matches!(self, Self::FirstSeen)
    }

    /// Returns `true` if the detect duplicate by is [`Name`].
    ///
    /// [`Name`]: DetectDuplicateBy::Name
    #[must_use]
    pub fn is_name(&self) -> bool {
        matches!(self, Self::Name)
    }

    /// Returns `true` if the detect duplicate by is [`Created`].
    ///
    /// [`Created`]: DetectDuplicateBy::Created
    #[must_use]
    pub fn is_created(&self) -> bool {
        matches!(self, Self::Created)
    }

    /// Returns `true` if the detect duplicate by is [`LastModified`].
    ///
    /// [`LastModified`]: DetectDuplicateBy::LastModified
    #[must_use]
    pub fn is_last_modified(&self) -> bool {
        matches!(self, Self::LastModified)
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
