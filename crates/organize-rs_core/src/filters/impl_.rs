use std::ops::Not;

use chrono::{DateTime, Utc};
#[cfg(feature = "cli")]
use clap::{Args, Subcommand, ValueEnum};

use itertools::Itertools;
use jwalk::{ClientState, DirEntry};

use crate::{
    filters::{
        CullKind, DateUnitKind, DuplicateKind, FilterApplicationKind, FilterClosure,
        FilterCollection, FilterGroup, FilterKind, FilterModeKind, NameFilterArgs,
        RawFilterApplicationKind, RecursiveFilterArgs,
    },
    parsers::{PeriodRange, SizeRange},
};

impl FilterKind {
    const NEGATE_STRING: &str = "#!";

    pub fn get_filter<C: ClientState>(&self) -> FilterClosure<C> {
        match self {
            FilterKind::NoFilter => Box::new(|_entry| false),
            FilterKind::AllItems {
                i_agree_it_is_dangerous,
            } => Box::new(|_entry| i_agree_it_is_dangerous.to_owned()),
            FilterKind::IgnorePath { in_path } => self.filter_ignore_str_is_in_path(in_path),
            FilterKind::IgnoreName { in_name } => self.filter_ignore_str_is_in_name(in_name),
            FilterKind::Extension { exts } => self.filter_by_extension(exts),
            FilterKind::Name {
                arguments,
                case_insensitive,
            } => self.filter_by_name(arguments, case_insensitive),
            FilterKind::Empty => self.filter_by_empty(),
            FilterKind::Created { range } => self.filter_by_created(range),
            FilterKind::LastModified { range } => self.filter_by_last_modified(range),
            FilterKind::LastAccessed { range } => self.filter_by_last_accessed(range),
            FilterKind::Mimetype { mimetype } => self.filter_by_mimetype(mimetype),
            FilterKind::Size { range } => self.filter_by_size(range),
            FilterKind::Regex { expr: _ } => todo!("not implemented (yet)!"),
            FilterKind::Exif => todo!("not implemented (yet)!"),
            FilterKind::Filecontent { regex: _ } => todo!("not implemented (yet)!"),
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
                    exts.iter().any(|f| f == extension_str)
                })
        })
    }

    fn filter_by_name<'a, 'args, C: ClientState>(
        &'a self,
        arguments: &'args NameFilterArgs,
        case_insensitive: &'args bool,
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            let arguments = arguments.clone();
            let file_path = entry.path();

            let file_name_str = file_path
                .file_name()
                .and_then(|f| f.to_str())
                .map(|f| {
                    if *case_insensitive {
                        f.to_lowercase()
                    } else {
                        f.to_owned()
                    }
                })
                .expect("should be able to unpack file name.");

            match arguments {
                NameFilterArgs {
                    starts_with: sw, ..
                } if !sw.is_empty() => sw
                    .into_iter()
                    .unique()
                    .map(|string| {
                        let mut str = string;
                        if *case_insensitive {
                            str = str.to_lowercase();
                        }
                        str
                    })
                    .map(|string| {
                        if string.starts_with(Self::NEGATE_STRING) {
                            FilterApplicationKind::Invert({
                                string
                                    .strip_prefix(Self::NEGATE_STRING)
                                    .map(|f| f.to_owned())
                            })
                        } else {
                            FilterApplicationKind::Apply(Some(string))
                        }
                    })
                    .map(|unique_string| match unique_string {
                        FilterApplicationKind::Invert(Some(invert)) => {
                            file_name_str.starts_with(&invert).then_some(CullKind::Bump)
                        }
                        FilterApplicationKind::Apply(Some(apply)) => file_name_str
                            .starts_with(&apply)
                            .then_some(CullKind::Retain),
                        _ => Some(CullKind::Retain),
                    })
                    .any(Self::is_bump)
                    .not(),
                NameFilterArgs { contains: c, .. } if !c.is_empty() => c
                    .into_iter()
                    .unique()
                    .map(|string| {
                        let mut str = string;
                        if *case_insensitive {
                            str = str.to_lowercase();
                        }
                        str
                    })
                    .map(|string| {
                        if string.starts_with(Self::NEGATE_STRING) {
                            FilterApplicationKind::Invert({
                                string
                                    .strip_prefix(Self::NEGATE_STRING)
                                    .map(|f| f.to_owned())
                            })
                        } else {
                            FilterApplicationKind::Apply(Some(string))
                        }
                    })
                    .map(|unique_string| match unique_string {
                        FilterApplicationKind::Invert(Some(invert)) => {
                            file_name_str.contains(&invert).then_some(CullKind::Bump)
                        }
                        FilterApplicationKind::Apply(Some(apply)) => {
                            file_name_str.contains(&apply).then_some(CullKind::Retain)
                        }
                        _ => Some(CullKind::Retain),
                    })
                    .any(Self::is_bump)
                    .not(),
                NameFilterArgs { ends_with: ew, .. } if !ew.is_empty() => ew
                    .into_iter()
                    .unique()
                    .map(|string| {
                        let mut str = string;
                        if *case_insensitive {
                            str = str.to_lowercase();
                        }
                        str
                    })
                    .map(|string| {
                        if string.starts_with(Self::NEGATE_STRING) {
                            FilterApplicationKind::Invert({
                                string
                                    .strip_prefix(Self::NEGATE_STRING)
                                    .map(|f| f.to_owned())
                            })
                        } else {
                            FilterApplicationKind::Apply(Some(string))
                        }
                    })
                    .map(|unique_string| {
                        let file_stem = file_path.file_stem().and_then(|f| f.to_str()).map(|f| {
                            if *case_insensitive {
                                f.to_lowercase()
                            } else {
                                f.to_owned()
                            }
                        });

                        match (unique_string, file_stem) {
                            (FilterApplicationKind::Invert(Some(invert)), None) => {
                                file_name_str.ends_with(&invert).then_some(CullKind::Bump)
                            }
                            (FilterApplicationKind::Apply(Some(apply)), None) => {
                                file_name_str.ends_with(&apply).then_some(CullKind::Retain)
                            }
                            (FilterApplicationKind::Invert(Some(invert)), Some(stem)) => {
                                stem.ends_with(&invert).then_some(CullKind::Bump)
                            }
                            (FilterApplicationKind::Apply(Some(apply)), Some(stem)) => {
                                stem.ends_with(&apply).then_some(CullKind::Retain)
                            }
                            _ => Some(CullKind::Retain),
                        }
                    })
                    .any(Self::is_bump)
                    .not(),
                NameFilterArgs { .. } => false,
            }
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
        range: &'args PeriodRange,
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            entry.metadata().ok().map_or(false, |metadata| {
                metadata.accessed().map_or(false, |sys_time| {
                    Self::matches_date(&sys_time, &range.clone())
                })
            })
        })
    }
    fn filter_by_last_modified<'a, 'args, C: ClientState>(
        &'a self,
        range: &'args PeriodRange,
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            entry.metadata().ok().map_or(false, |metadata| {
                metadata.modified().map_or(false, |sys_time| {
                    Self::matches_date(&sys_time, &range.clone())
                })
            })
        })
    }
    fn filter_by_created<'a, 'args, C: ClientState>(
        &'a self,
        range: &'args PeriodRange,
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            entry.metadata().ok().map_or(false, |metadata| {
                metadata.created().map_or(false, |sys_time| {
                    Self::matches_date(&sys_time, &range.clone())
                })
            })
        })
    }

    fn matches_date(item_date: &std::time::SystemTime, range: &PeriodRange) -> bool {
        let datetime_file: DateTime<Utc> = chrono::DateTime::from(*item_date);
        let now = chrono::offset::Utc::now();

        let seconds_since_created = match u64::try_from((now - datetime_file).num_seconds()) {
            Ok(it) => it,
            Err(err) => {
                eprintln!("subtraction of two datetimes can't be negative: {err}");
                return false;
            }
        } as f64;

        range.in_range(seconds_since_created)
    }

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
        range: &'args SizeRange,
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            entry
                .metadata()
                .map_or(false, |metadata| range.in_range(metadata.len() as f64))
        })
    }

    fn filter_ignore_str_is_in_name<'a, 'args, C: ClientState>(
        &'a self,
        ignore_name: &'args [String],
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            if entry.file_type().is_file() {
                entry.file_name().to_str().map_or(false, |file_name| {
                    ignore_name
                        .iter()
                        .any(|pat| file_name.to_lowercase().contains(&pat.to_lowercase()))
                })
            } else {
                false
            }
        })
    }

    fn filter_ignore_str_is_in_path<'a, 'args, C: ClientState>(
        &'a self,
        ignore_path: &'args [String],
    ) -> Box<dyn FnMut(&DirEntry<C>) -> bool + 'args> {
        Box::new(|entry| {
            entry.path().to_str().map_or(false, |path| {
                ignore_path
                    .iter()
                    .any(|pat| path.to_lowercase().contains(&pat.to_lowercase()))
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

impl<T> FilterApplicationKind<T> {
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

impl<T> FilterGroup<T> {
    pub fn new(apply: RawFilterApplicationKind, mode: FilterModeKind, filters: T) -> Self {
        Self {
            apply,
            mode,
            filters,
        }
    }
}

impl FilterCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_vec(
        filter_collection: Vec<(FilterModeKind, FilterApplicationKind<FilterKind>)>,
    ) -> Self {
        Self(filter_collection)
    }

    pub fn decompose(self) -> Vec<(FilterModeKind, FilterApplicationKind<FilterKind>)> {
        self.0
    }

    pub fn push(&mut self, filter_collection: (FilterModeKind, FilterApplicationKind<FilterKind>)) {
        self.0.push(filter_collection)
    }
}

impl FilterModeKind {
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
