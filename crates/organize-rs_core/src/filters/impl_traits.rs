use std::fmt::Display;

#[cfg(feature = "cli")]
use clap::{Args, Subcommand, ValueEnum};

use itertools::{Either, Itertools};

use crate::filters::{
    DateUnitKind, DuplicateKind, FilterApplicationKind, FilterCollection, FilterGroup,
    FilterGroupCollection, FilterGroupOperationKind, FilterKind, FilterOperationKind,
};

impl Default for FilterKind {
    fn default() -> Self {
        Self::NoFilter
    }
}

impl From<(f64, &str)> for DateUnitKind {
    fn from(value: (f64, &str)) -> Self {
        let (value, unit) = value;

        match unit {
            "y" => Self::Years(value),
            "mo" => Self::Months(value),
            "w" => Self::Weeks(value),
            "d" => Self::Days(value),
            "h" => Self::Hours(value),
            "m" => Self::Minutes(value),
            "s" => Self::Seconds(value),
            &_ => panic!("use of a non-standard unit"),
        }
    }
}

impl Display for FilterCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
    FilterCollection

    Filters:
        "
        )?;

        let (all, other): (Vec<_>, Vec<_>) =
            self.0.iter().partition_map(|(mode, filter)| match mode {
                FilterApplicationKind::All => Either::Left((mode, filter)),
                FilterApplicationKind::Any => Either::Right((mode, filter)),
                FilterApplicationKind::None => Either::Right((mode, filter)),
            });

        let (any, none): (Vec<_>, Vec<_>) =
            other
                .into_iter()
                .partition_map(|(mode, filter)| match mode {
                    FilterApplicationKind::Any => Either::Left((mode, filter)),
                    FilterApplicationKind::None => Either::Right((mode, filter)),
                    FilterApplicationKind::All => {
                        unreachable!(
                            "We took already care of that variant, shouldn't exist in here."
                        )
                    }
                });

        let variants_partitioned = vec![all, any, none];

        for variant in &variants_partitioned {
            for (idx, (mode, filter)) in variant.iter().enumerate() {
                if idx == 0 {
                    write!(
                        f,
                        "
    Mode: {mode}
    ^^^^"
                    )?;
                }

                write!(
                    f,
                    "
    Application Kind: {filter}"
                )?;
            }
        }
        Ok(())
    }
}

impl Display for FilterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterKind::NoFilter => write!(
                f,
                "
    -> NoFilter
            "
            ),
            FilterKind::AllItems {
                i_agree_it_is_dangerous,
            } => write!(
                f,
                "
    -> All items
        Arguments:
            consent: {i_agree_it_is_dangerous}
            "
            ),
            FilterKind::Created { range } => {
                write!(
                    f,
                    "
    -> Created
        Arguments:
            range: {range:?}
                "
                )
            }
            FilterKind::LastAccessed { range } => {
                write!(
                    f,
                    "
    -> LastAccessed
        Arguments:
            range: {range:?}
                "
                )
            }
            FilterKind::LastModified { range } => {
                write!(
                    f,
                    "
    -> LastModified
        Arguments:
            range: {range:?}
    "
                )
            }
            FilterKind::Duplicate {
                detect_original_by,
                reverse,
            } => write!(
                f,
                "
    -> Duplicate
        Arguments:
            detection: {detect_original_by:?},
            reverse: {reverse}
    "
            ),
            FilterKind::Empty => write!(f, "Filter: Empty."),
            FilterKind::Exif { contains } => {
                write!(
                    f,
                    "
    -> Exif
        Arguments:
            contains: {contains:?}
                "
                )
            }
            FilterKind::Extension { exts } => {
                write!(
                    f,
                    "
    -> Extension
        Arguments:
            exts: {exts:?}
                "
                )
            }
            FilterKind::FileContent { expr: regex } => {
                write!(
                    f,
                    "
    -> Filecontent
        Arguments:
            regex: {regex}
    "
                )
            }
            FilterKind::Mimetype { mime: mimetype } => {
                write!(
                    f,
                    "
    -> Mimetype
        Arguments:
            mimetypes: {mimetype:?}
                "
                )
            }
            FilterKind::Name {
                arguments,
                case_insensitive,
            } => write!(
                f,
                "
    -> Name
        Arguments:
            args: {arguments:?},
            case_insensitive: {case_insensitive}
                "
            ),
            FilterKind::Regex { expr } => write!(
                f,
                "
    -> Regex
        Arguments:
        expr: {expr}
            "
            ),
            FilterKind::Size { range } => {
                write!(
                    f,
                    "
    -> Size
        Arguments:
            range: {range:?}
                "
                )
            }
            FilterKind::IgnorePath { in_path } => {
                write!(
                    f,
                    "
    -> IgnorePath
        Arguments:
            paths: {in_path:?}
                "
                )
            }
            FilterKind::IgnoreName { in_name } => {
                write!(
                    f,
                    "
    -> IgnoreName
        Arguments:
            names: {in_name:?}
                "
                )
            }
        }
    }
}

impl Default for DuplicateKind {
    fn default() -> Self {
        Self::Name
    }
}

impl Default for FilterApplicationKind {
    fn default() -> Self {
        Self::Any
    }
}

impl std::ops::Deref for FilterCollection {
    type Target = Vec<(FilterApplicationKind, FilterOperationKind<FilterKind>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Default> Default for FilterGroup<T> {
    fn default() -> Self {
        Self {
            operation: FilterGroupOperationKind::default(),
            mode: FilterApplicationKind::default(),
            filters: T::default(),
        }
    }
}

impl Default for FilterGroupOperationKind {
    fn default() -> Self {
        Self::Include
    }
}

impl<T> Default for FilterOperationKind<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::Apply(T::default())
    }
}

impl std::ops::DerefMut for FilterGroupCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for FilterGroupCollection {
    type Target = Vec<FilterGroup<Vec<FilterKind>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
