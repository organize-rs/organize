//! Filters that `organize` operates with

use std::{
    fmt::{Debug, Display},
    ops::Not,
};

use chrono::{DateTime, Utc};
#[cfg(feature = "cli")]
use clap::{Args, Subcommand, ValueEnum};

use displaydoc::Display;

use itertools::{Either, Itertools};
use jwalk::{ClientState, DirEntry};
use serde::{Deserialize, Serialize};

use crate::parsers::{PeriodRange, SizeRange};

pub type FilterClosure<'a, C> = Box<dyn FnMut(&DirEntry<C>) -> bool + 'a>;
pub type FilterClosureCollection<'a, C> = Vec<FilterClosure<'a, C>>;
pub type FilterFilterClosureSliceMut<'a, C> = &'a mut [Box<dyn FnMut(&DirEntry<C>) -> bool>];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CullKind {
    Retain,
    Bump,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display)]
#[serde(transparent)]
pub struct FilterApplicationCollection(Vec<FilterApplicationKind<FilterKind>>);

impl std::ops::DerefMut for FilterApplicationCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for FilterApplicationCollection {
    fn default() -> Self {
        Self(vec![FilterApplicationKind::default()])
    }
}

impl std::ops::Deref for FilterApplicationCollection {
    type Target = Vec<FilterApplicationKind<FilterKind>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct FilterCollection(Vec<(FilterModeGroupKind, FilterApplicationKind<FilterKind>)>);

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
                FilterModeGroupKind::All => Either::Left((mode, filter)),
                FilterModeGroupKind::Any => Either::Right((mode, filter)),
                FilterModeGroupKind::None => Either::Right((mode, filter)),
            });

        let (any, none): (Vec<_>, Vec<_>) =
            other
                .into_iter()
                .partition_map(|(mode, filter)| match mode {
                    FilterModeGroupKind::Any => Either::Left((mode, filter)),
                    FilterModeGroupKind::None => Either::Right((mode, filter)),
                    FilterModeGroupKind::All => {
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
                )?
            }
        }

        Ok(())
    }
}

impl FilterCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_vec(
        filter_collection: Vec<(FilterModeGroupKind, FilterApplicationKind<FilterKind>)>,
    ) -> Self {
        Self(filter_collection)
    }

    pub fn decompose(self) -> Vec<(FilterModeGroupKind, FilterApplicationKind<FilterKind>)> {
        self.0
    }

    pub fn push(
        &mut self,
        filter_collection: (FilterModeGroupKind, FilterApplicationKind<FilterKind>),
    ) {
        self.0.push(filter_collection)
    }
}

impl std::ops::Deref for FilterCollection {
    type Target = Vec<(FilterModeGroupKind, FilterApplicationKind<FilterKind>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Should filters be negated
#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum FilterApplicationKind<T> {
    /// Invert {0}
    Invert(T),
    /// Apply {0}
    Apply(T),
}

impl<T> Default for FilterApplicationKind<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::Apply(T::default())
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

/// Application of filters, so whether "all", "any" or "none"
/// of the filters must apply

#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(
    Debug, Clone, Deserialize, Serialize, Display, PartialEq, Eq, PartialOrd, Ord, Copy, Hash,
)]
#[non_exhaustive]
pub enum FilterModeGroupKind {
    /// All of the filters need to apply
    All,
    /// Any of the filters need to apply
    Any,
    /// None of the filters need to apply
    None,
}

impl Default for FilterModeGroupKind {
    fn default() -> Self {
        Self::Any
    }
}

impl FilterModeGroupKind {
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

/// Duplication detection
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DuplicateKind {
    /// The first entry sorted by creation date is the original.
    Created,
    /// Whatever file is visited first is the original.
    ///
    /// This depends on the order of your location entries.
    FirstSeen,
    // TODO
    Hash,
    /// The first file sorted by date of last modification is the original.
    LastModified,
    /// The first entry sorted by name is the original.
    Name,
}

impl Default for DuplicateKind {
    fn default() -> Self {
        Self::Name
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

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Display, Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "cli", group(required = false, multiple = true))]
pub struct RecursiveFilterArgs {
    /// Recurse into subfolders
    #[cfg_attr(
        feature = "cli",
        arg(short, long, default_value_t = false, global = true, group = "recurse")
    )]
    recursive: bool,

    /// Maximal depth when operating recursively
    #[cfg_attr(
        feature = "cli",
        arg(short, long, global = true, default_value_t = 1, requires = "recurse")
    )]
    max_depth: u64,
}

impl RecursiveFilterArgs {
    pub fn recursive(&self) -> bool {
        self.recursive
    }

    pub fn max_depth(&self) -> u64 {
        self.max_depth
    }
}

/// Arguments for `name` filter
#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Display, Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "cli", group(required = true, multiple = false))]
pub struct NameFilterArgs {
    // TODO: Not implemented, searching for alternatives
    /// A matching string in [simplematch-syntax](https://github.com/tfeldmann/simplematch)
    #[cfg_attr(feature = "cli", arg(long))]
    simple_match: Vec<String>,
    /// The filename must begin with the given string
    #[cfg_attr(feature = "cli", arg(long))]
    starts_with: Vec<String>,
    /// The filename must contain the given string
    #[cfg_attr(feature = "cli", arg(long))]
    contains: Vec<String>,
    /// The filename (without extension) must end with the given string
    #[cfg_attr(feature = "cli", arg(long))]
    ends_with: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum DateUnitKind {
    /// specify number of days
    #[serde(rename = "days")]
    Days(f64),
    /// specify number of hours
    #[serde(rename = "hours")]
    Hours(f64),
    /// specify number of minutes
    #[serde(rename = "minutes")]
    Minutes(f64),
    /// specify number of months
    #[serde(rename = "months")]
    Months(f64),
    /// specify number of seconds
    #[serde(rename = "seconds")]
    Seconds(f64),
    /// specify number of weeks
    #[serde(rename = "weeks")]
    Weeks(f64),
    /// specify number of years
    #[serde(rename = "years")]
    Years(f64),
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

/// [`OrganizeFilter`] contains filter variants that organize can
/// use to apply to locations.
#[cfg_attr(feature = "cli", derive(Subcommand))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FilterKind {
    /// Match locations by the time they were added to a folder
    ///
    /// # Result
    ///
    /// The datetime the files / folders were added.
    ///
    /// # Example
    ///
    /// Show the date the file was added to the folder
    ///
    /// ```yaml
    /// rules:
    ///    - name: Show the date the file was added to the folder
    ///      locations: "~/Desktop"
    ///      filters:
    ///        - date_added
    ///      actions:
    ///        - echo: "Date added: {date_added.strftime('%Y-%m-%d')}"
    /// ```
    #[cfg(target_os = "osx")]
    #[serde(rename = "date_added")]
    Added {
        /// This filter uses the `range` syntax (always inclusive) of Rust.
        /// ..7d => in the last 7 days; 2mo.. => older than 2 months and onwards; 1d..2d =>
        /// between 1 to 2 days old. Left and right boundary need to have the same units.
        /// [possible values: y, mo, w, d, h, m, s]
        ///
        /// **NOTE**: You can one of `['y', 'mo', 'w', 'd', 'h', 'm', 's']`. They
        /// will be **converted** to `seconds` accordingly and are **case-insensitive**.
        #[cfg_attr(feature = "cli", arg(long, value_parser = clap::value_parser!(PeriodRange)))]
        range: PeriodRange,
    },
    /// Output all items
    ///
    /// # Result
    ///
    /// Careful! All items are returned, meaning in combination with
    /// an action like `Trash` it would move *all* files/folders to
    /// the trash bin.
    AllItems {
        #[cfg_attr(feature = "cli", arg(long))]
        i_agree_it_is_dangerous: bool,
    },
    /// Match locations by the time they were created
    ///
    /// # Result
    ///
    /// The datetime the location was created.
    ///
    /// # Example
    ///
    /// Sort pdfs by year of creation
    ///
    /// ```rust
    /// let rule = r#"
    /// rules:
    ///    - name: Sort pdfs by year of creation
    ///      locations: "~/Documents"
    ///      filters:
    ///        - extension: pdf
    ///        - created
    ///      actions:
    ///        - move: "~/Documents/PDF/{created.year}/"
    /// "#;
    /// ```
    #[serde(rename = "created")]
    Created {
        /// This filter uses the `range` syntax (always inclusive) of Rust.
        /// ..7d => in the last 7 days; 2mo.. => older than 2 months and onwards; 1d..2d =>
        /// between 1 to 2 days old. Left and right boundary need to have the same units.
        /// [possible values: y, mo, w, d, h, m, s]
        ///
        /// **NOTE**: You can one of `['y', 'mo', 'w', 'd', 'h', 'm', 's']`. They
        /// will be **converted** to `seconds` accordingly and are **case-insensitive**.
        #[cfg_attr(feature = "cli", arg(long, value_parser = clap::value_parser!(PeriodRange)))]
        range: PeriodRange,
    },
    /// Match locations that have duplicates
    ///
    /// This filter compares locations byte by byte and finds identical
    /// locations with potentially different filenames.
    ///
    /// You can reverse the sorting method by setting `reverse`.
    ///
    /// So with detect_original_by: "-created" the file with the older
    /// creation date is the original and the younger file is the
    /// duplicate. This works on all methods, for example "-first_seen",
    /// "-name", "-created", "-lastmodified".
    ///
    /// # Result
    ///
    /// The path to the original
    ///
    /// # Example
    ///
    /// Show all duplicate locations in your desktop and download folder
    /// (and their subfolders)
    ///
    /// ```yaml
    /// rules:
    ///   - name: Show all duplicate locations in your desktop and download folder (and their subfolders)
    ///     locations:
    ///       - ~/Desktop
    ///       - ~/Downloads
    ///     subfolders: true
    ///     filters:
    ///       - duplicate
    ///     actions:
    ///       - echo: "{path} is a duplicate of {duplicate.original}"
    /// ```
    #[serde(rename = "duplicate")]
    Duplicate {
        #[cfg_attr(feature = "cli", arg(long))]
        detect_original_by: Option<DuplicateKind>,
        #[cfg_attr(feature = "cli", arg(long))]
        reverse: bool,
    },
    /// Find empty locations
    ///
    /// # Example
    ///
    /// Recursively delete empty folders
    ///
    /// ```yaml
    /// rules:
    ///   - targets: dirs
    ///     locations:
    ///       - path: ~/Desktop
    ///         max_depth: null
    ///     filters:
    ///       - empty
    ///     actions:
    ///       - delete
    /// ```
    #[serde(rename = "empty")]
    Empty,
    /// Filter images by their EXIF data
    ///
    /// The exif filter can be used as a filter as well as a way to
    /// get exif information into your actions
    ///
    /// # Result
    ///
    /// An object of all the collected exif inforamtion available in the file.
    /// Typically it consists of the following tags (if present in the file):
    ///
    /// - `{exif.image}` -- information related to the main image
    /// - `{exif.exif}` -- Exif information
    /// - `{exif.gps}` -- GPS information
    /// - `{exif.interoperability}` -- Interoperability information
    ///
    /// # Example
    ///
    /// Copy all images which contain GPS information while keeping
    /// subfolder structure:
    ///
    /// ```yaml
    /// rules:
    ///   - name: "GPS demo"
    ///     locations:
    ///       - path: ~/Pictures
    ///         max_depth: null
    ///     filters:
    ///       - exif: gps.gpsdate
    ///     actions:
    ///       - copy: ~/Pictures/with_gps/{relative_path}/
    /// ```
    #[serde(rename = "exif")]
    Exif,
    /// Match locations by their file extension
    ///
    /// # Result
    ///
    /// the original file extension (without colon)
    ///
    /// # Example
    ///
    /// Match multiple file extensions
    ///
    /// ```yaml
    /// rules:
    ///   - name: "Match multiple file extensions"
    ///     locations: "~/Desktop"
    ///     filters:
    ///       - extension:
    ///           - .jpg
    ///           - jpeg
    ///     actions:
    ///       - echo: "Found JPG file: {path}"
    /// ```
    #[serde(rename = "extension")]
    Extension {
        /// The file extensions to match (without dot)
        #[cfg_attr(feature = "cli", arg(long))]
        exts: Vec<String>,
    },
    /// Match file content with the given regular expression
    ///
    /// Any named groups `((?P<groupname>.*))` in your regular
    /// expression will be returned.
    ///
    /// # Result
    ///  
    /// The text matched with the named group `(?P<groupname>)`
    ///
    /// # Example
    ///
    /// Match an invoice with a regular expression and sort by customer
    ///
    /// ```yaml
    /// rules:
    ///   - name: "Match an invoice with a regular expression and sort by customer"
    ///     locations: "~/Desktop"
    ///     filters:
    ///       - filecontent: 'Invoice.*Customer (?P<customer>\w+)'
    ///     actions:
    ///       - move: "~/Documents/Invoices/{filecontent.customer}/"
    /// ```
    #[serde(rename = "filecontent")]
    Filecontent {
        #[cfg_attr(feature = "cli", arg(long))]
        regex: String,
    },
    // TODO: Check for available hash algorithms from organize-py
    // TODO: shake_256, sha3_256, sha1, sha3_224, sha384, sha512, blake2b,
    // TODO: blake2s, sha256, sha224, shake_128, sha3_512, sha3_384 and md5
    /// Calculat the hash of a file
    ///
    /// # Result
    ///
    /// The hash of the file.
    ///
    /// # Example
    ///
    /// Show the hashes of your files:
    ///
    /// ```yaml
    /// rules:
    ///   - name: "Show the hashes and size of your files"
    ///     locations: "~/Desktop"
    ///     filters:
    ///       - hash
    ///       - size
    ///     actions:
    ///       - echo: "{hash} {size.decimal}"
    /// ```
    #[cfg(feature = "research_organize")]
    #[serde(rename = "hash")]
    Hash,
    /// Defines a string that makes organize skip a location when found in the file name
    IgnoreName {
        /// Matches for these Strings in the Filename
        // #[cfg_attr(feature = "cli", arg(long))]
        in_name: Vec<String>,
    },
    /// Defines a string that makes organize skip a location when found in the full path
    IgnorePath {
        /// Matches for these Strings in the whole Path
        // #[cfg_attr(feature = "cli", arg(long))]
        in_path: Vec<String>,
    },
    /// Match locations by the time they were last accessed
    ///
    /// # Result
    ///
    /// The datetime the location / folders were accessed.
    ///
    /// # Example
    ///
    /// Show the date the location last accessed
    ///
    /// ```yaml
    /// rules:
    ///    - name: Show the date the location was last accessed
    ///      locations: "~/Desktop"
    ///      filters:
    ///        - last_accessed
    ///      actions:
    ///        - echo: "Date last used: {last_accessed.strftime('%Y-%m-%d')}"
    /// ```
    #[serde(rename = "last_accessed")]
    LastAccessed {
        /// This filter uses the `range` syntax (always inclusive) of Rust.
        /// ..7d => in the last 7 days; 2mo.. => older than 2 months and onwards; 1d..2d =>
        /// between 1 to 2 days old. Left and right boundary need to have the same units.
        /// [possible values: y, mo, w, d, h, m, s]
        ///
        /// **NOTE**: You can one of `['y', 'mo', 'w', 'd', 'h', 'm', 's']`. They
        /// will be **converted** to `seconds` accordingly and are **case-insensitive**.
        #[cfg_attr(feature = "cli", arg(long, value_parser = clap::value_parser!(PeriodRange)))]
        range: PeriodRange,
    },
    /// Match locations by the time they were last modified
    ///
    /// # Result
    ///
    /// The datetime the location / folders was last modified.
    ///
    /// # Example
    ///
    /// Sort pdfs by year of last modification
    ///
    /// ```yaml
    /// rules:
    ///   - name: "Sort pdfs by year of last modification"
    ///     locations: "~/Documents"
    ///     filters:
    ///       - extension: pdf
    ///       - lastmodified
    ///     actions:
    ///       - move: "~/Documents/PDF/{lastmodified.year}/"
    /// ```
    #[serde(rename = "last_modified")]
    LastModified {
        /// This filter uses the `range` syntax (always inclusive) of Rust.
        /// ..7d => in the last 7 days; 2mo.. => older than 2 months and onwards; 1d..2d =>
        /// between 1 to 2 days old. Left and right boundary need to have the same units.
        /// [possible values: y, mo, w, d, h, m, s]
        ///
        /// **NOTE**: You can one of `['y', 'mo', 'w', 'd', 'h', 'm', 's']`. They
        /// will be **converted** to `seconds` accordingly and are **case-insensitive**.
        #[cfg_attr(feature = "cli", arg(long, value_parser = clap::value_parser!(PeriodRange)))]
        range: PeriodRange,
    },
    /// Filter by macOS tags
    ///
    /// # Example
    ///
    /// All locations with a tag 'Invoice' (any color) or with a green tag
    ///
    /// ```yaml
    /// rules:
    ///   - name: "All locations with a tag 'Invoice' (any color) or with a green tag"
    ///     locations: "~/Downloads"
    ///     filters:
    ///       - macos_tags:
    ///           - "Invoice (*)"
    ///           - "* (green)"
    ///     actions:
    ///       - echo: "Match found!"
    /// ```
    #[cfg(target_os = "osx")]
    #[serde(rename = "macos_tags")]
    MacOsTags {
        #[cfg_attr(feature = "cli", arg(long))]
        tags: Vec<String>,
    },
    /// Filter by MIME type associated with the file extension
    ///
    /// Supports a single string or list of MIME type strings as argument.
    /// The types don't need to be fully specified, for example "audio"
    /// matches everything from "audio/midi" to "audio/quicktime".
    ///
    /// # Result
    ///
    /// The MIME type of the file.
    ///
    /// # Example
    ///
    /// Filter by 'image' mimetype
    ///
    /// ```yaml
    ///
    /// rules:
    ///   - name: "Filter by 'image' mimetype"
    ///     locations: "~/Downloads"
    ///     filters:
    ///       - mimetype: image
    ///     actions:
    ///       - echo: "This file is an image: {mimetype}"
    /// ```
    #[serde(rename = "mimetype")]
    Mimetype {
        #[cfg_attr(feature = "cli", arg(long))]
        mimetype: Vec<String>,
    },
    /// Match locations by their name
    ///
    /// # Example
    ///
    /// Match all locations starting with 'A' or 'B' containing '5' or
    /// '6' and ending with '_end'.
    ///
    /// ```yaml
    /// rules:
    ///   - locations: "~/Desktop"
    ///     filters:
    ///       - name:
    ///           startswith:
    ///             - "A"
    ///             - "B"
    ///           contains:
    ///             - "5"
    ///             - "6"
    ///           endswith: _end
    ///           case_sensitive: false
    ///     actions:
    ///       - echo: "Found a match."
    /// ```
    #[serde(rename = "name")]
    Name {
        #[cfg_attr(feature = "cli", command(flatten))]
        #[serde(flatten)]
        arguments: NameFilterArgs,
        /// By default, the matching is case sensitive.
        ///
        /// Change this to `False` to use case insensitive matching.
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(rename = "case_sensitive")]
        case_insensitive: bool,
    },
    /// Don't use any filter
    ///
    /// # Result
    ///
    /// Empty / no items due to the risk otherwise if it's used in
    /// combination with an action, that the action will be applied
    /// to all results.
    NoFilter,
    /// Match filenames with the given regular expression
    ///
    /// Any named groups `((?P<groupname>.*))` in your regular
    /// expression will be returned.
    ///
    /// # Result
    ///  
    /// The text matched with the named group `(?P<groupname>)`
    ///
    /// # Example
    ///
    /// Match an invoice with a regular expression and sort by customer
    /// and rename the invoice using the invoice number extracted via the
    /// regular expression
    ///
    /// ```yaml
    /// rules:
    ///   - locations: ~/Desktop
    ///     filters:
    ///       - regex: '^RG(?P<the_number>\d{12})-sig\.pdf$'
    ///     actions:
    ///       - move: ~/Documents/Invoices/1und1/{regex.the_number}.pdf
    /// ```
    #[serde(rename = "regex")]
    Regex {
        #[cfg_attr(feature = "cli", arg(long))]
        expr: String,
    },
    /// Match locations by their size
    ///
    /// Accepts file size conditions, e.g: ">= 500 MB", "< 20k", ">0", "= 10 KiB".
    ///
    /// It is possible to define both lower and upper conditions like this:
    /// ">20k, < 1 TB", ">= 20 Mb, <25 Mb".
    ///
    /// The filter will match if all given conditions are satisfied.
    ///
    /// - Accepts all units from KB to YB.
    /// - If no unit is given, kilobytes are assumend.
    /// - If binary prefix is given (KiB, GiB) the size is calculated using base 1024.
    ///
    /// # Result
    ///
    /// {size.bytes}: (int) Size in bytes
    /// {size.traditional}: (str) Size with unit (powers of 1024, JDEC prefixes)
    /// {size.binary}: (str) Size with unit (powers of 1024, IEC prefixes)
    /// {size.decimal}: (str) Size with unit (powers of 1000, SI prefixes)
    ///
    /// # Example
    ///
    /// Trash big downloads
    ///
    /// ```yaml
    /// rules:
    ///   - locations: "~/Downloads"
    ///     targets: files
    ///     filters:
    ///       - size: "> 0.5 GB"
    ///     actions:
    ///       - trash
    /// ```
    ///
    /// # Example
    ///
    /// Move all JPEGs bigger > 1MB and <10 MB. Search all subfolders and
    /// keep the original relative path.
    ///
    /// ```yaml
    /// rules:
    ///   - locations:
    ///       - path: "~/Pictures"
    ///         max_depth: null
    ///     filters:
    ///       - extension:
    ///           - jpg
    ///           - jpeg
    ///       - size: ">1mb, <10mb"
    ///     actions:
    ///       - move: "~/Pictures/sorted/{relative_path}/"
    /// ```
    #[serde(rename = "size")]
    Size {
        /// This filter uses the `range` syntax (always inclusive) of Rust.
        /// ..11MB => smaller than; 15MB.. => bigger than; 10KB..20MiB =>
        /// bigger than 10 KB, but smaller than 20 MiB
        ///
        /// **NOTE**: You can use `decimal` (metric) and `binary` (IEC)
        /// multiple-byte units. E.g., `KiB` or `KB`, `GB` or `GiB`. They
        /// will be **converted** accordingly and are **case-insensitive**.
        #[cfg_attr(feature = "cli", arg(long, value_parser = clap::value_parser!(SizeRange)))]
        range: SizeRange,
    },
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
            range: {range}
                "
                )
            }
            FilterKind::LastAccessed { range } => {
                write!(
                    f,
                    "
    -> LastAccessed
        Arguments:
            range: {range}
                "
                )
            }
            FilterKind::LastModified { range } => {
                write!(
                    f,
                    "
    -> LastModified
        Arguments:
            range: {range}
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
            FilterKind::Exif => write!(f, "Filter: Exif."),
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
            FilterKind::Filecontent { regex } => {
                write!(
                    f,
                    "
    -> Filecontent
        Arguments:
            regex: {regex}
    "
                )
            }
            FilterKind::Mimetype { mimetype } => {
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
            range: {range}
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

impl Default for FilterKind {
    fn default() -> Self {
        Self::NoFilter
    }
}

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
            entry.file_name().to_str().map_or(false, |file_name| {
                ignore_name
                    .iter()
                    .any(|pat| file_name.to_lowercase().contains(&pat.to_lowercase()))
            })
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
