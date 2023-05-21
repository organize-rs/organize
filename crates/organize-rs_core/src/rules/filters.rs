//! FilterSelf::Extension()he config file(s)
//! and `organize` operates with
//!

use chrono::{DateTime, Utc};
#[cfg(feature = "cli")]
use clap::{Args, Subcommand, ValueEnum};

use displaydoc::Display;

use serde::{Deserialize, Serialize};
use walkdir::DirEntry;

/// Comparison conditions for dates
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Display)]
pub enum Interval {
    /// older
    OlderThan,
    /// newer
    NewerThan,
}

impl Interval {
    /// Returns `true` if the older newer is [`Older`].
    ///
    /// [`Older`]: OlderNewer::Older
    #[must_use]
    pub fn is_older(&self) -> bool {
        matches!(self, Self::OlderThan)
    }

    /// Returns `true` if the older newer is [`Newer`].
    ///
    /// [`Newer`]: OlderNewer::Newer
    #[must_use]
    pub fn is_newer(&self) -> bool {
        matches!(self, Self::NewerThan)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::OlderThan
    }
}

/// Duplication detection
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DetectDuplicateBy {
    /// Whatever file is visited first is the original.
    ///
    /// This depends on the order of your location entries.
    FirstSeen,
    /// The first entry sorted by name is the original.
    Name,
    /// The first entry sorted by creation date is the original.
    Created,
    /// The first file sorted by date of last modification is the original.
    LastModified,
    // TODO
    // Hash,
}

impl Default for DetectDuplicateBy {
    fn default() -> Self {
        Self::Name
    }
}

impl DetectDuplicateBy {
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

/// Comparison conditions for the size of a location
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum SizeConditions {
    GreaterThan(u64),
    GreaterOrEqual(u64),
    SmallerThan(u64),
    SmallerOrEqual(u64),
    EqualTo(u64),
}

/// Comparison conditions for the size of a location
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum SizeConditionsRaw {
    GreaterThan,
    GreaterOrEqual,
    SmallerThan,
    SmallerOrEqual,
    EqualTo,
}

impl SizeConditions {
    /// Returns `true` if the size conditions is [`GreaterThan`].
    ///
    /// [`GreaterThan`]: SizeConditions::GreaterThan
    #[must_use]
    pub fn is_greater_than(&self) -> bool {
        matches!(self, Self::GreaterThan(..))
    }

    pub fn as_greater_than(&self) -> Option<&u64> {
        if let Self::GreaterThan(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_greater_than(self) -> Result<u64, Self> {
        if let Self::GreaterThan(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the size conditions is [`GreaterOrEqual`].
    ///
    /// [`GreaterOrEqual`]: SizeConditions::GreaterOrEqual
    #[must_use]
    pub fn is_greater_or_equal(&self) -> bool {
        matches!(self, Self::GreaterOrEqual(..))
    }

    pub fn as_greater_or_equal(&self) -> Option<&u64> {
        if let Self::GreaterOrEqual(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_greater_or_equal(self) -> Result<u64, Self> {
        if let Self::GreaterOrEqual(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the size conditions is [`SmallerThan`].
    ///
    /// [`SmallerThan`]: SizeConditions::SmallerThan
    #[must_use]
    pub fn is_smaller_than(&self) -> bool {
        matches!(self, Self::SmallerThan(..))
    }

    pub fn as_smaller_than(&self) -> Option<&u64> {
        if let Self::SmallerThan(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_smaller_than(self) -> Result<u64, Self> {
        if let Self::SmallerThan(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the size conditions is [`SmallerOrEqual`].
    ///
    /// [`SmallerOrEqual`]: SizeConditions::SmallerOrEqual
    #[must_use]
    pub fn is_smaller_or_equal(&self) -> bool {
        matches!(self, Self::SmallerOrEqual(..))
    }

    pub fn as_smaller_or_equal(&self) -> Option<&u64> {
        if let Self::SmallerOrEqual(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_smaller_or_equal(self) -> Result<u64, Self> {
        if let Self::SmallerOrEqual(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the size conditions is [`EqualTo`].
    ///
    /// [`EqualTo`]: SizeConditions::EqualTo
    #[must_use]
    pub fn is_equal_to(&self) -> bool {
        matches!(self, Self::EqualTo(..))
    }

    pub fn as_equal_to(&self) -> Option<&u64> {
        if let Self::EqualTo(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_equal_to(self) -> Result<u64, Self> {
        if let Self::EqualTo(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Display, Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "cli", group(required = false, multiple = true))]
pub struct FilterRecursive {
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

impl FilterRecursive {
    pub fn recursive(&self) -> bool {
        self.recursive
    }

    pub fn max_depth(&self) -> u64 {
        self.max_depth
    }
}

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Display, Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "cli", group(required = true, multiple = false))]
pub struct NameFilterArgs {
    // TODO: Not implemented, searching for alternatives
    /// A matching string in [simplematch-syntax](https://github.com/tfeldmann/simplematch)
    #[cfg_attr(feature = "cli", arg(long))]
    simple_match: Option<String>,
    /// The filename must begin with the given string
    #[cfg_attr(feature = "cli", arg(long))]
    starts_with: Option<String>,
    /// The filename must contain the given string
    #[cfg_attr(feature = "cli", arg(long))]
    contains: Option<String>,
    /// The filename (without extension) must end with the given string
    #[cfg_attr(feature = "cli", arg(long))]
    ends_with: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum DateUnit {
    /// specify number of years
    #[serde(rename = "years")]
    Years(u64),
    /// specify number of months
    #[serde(rename = "months")]
    Months(u64),
    /// specify number of weeks
    #[serde(rename = "weeks")]
    Weeks(u64),
    /// specify number of days
    #[serde(rename = "days")]
    Days(u64),
    /// specify number of hours
    #[serde(rename = "hours")]
    Hours(u64),
    /// specify number of minutes
    #[serde(rename = "minutes")]
    Minutes(u64),
    /// specify number of seconds
    #[serde(rename = "seconds")]
    Seconds(u64),
}

impl From<FilterDate> for DateUnit {
    fn from(value: FilterDate) -> Self {
        match value {
            FilterDate {
                years: Some(years), ..
            } => DateUnit::Years(years),
            FilterDate {
                months: Some(months),
                ..
            } => DateUnit::Months(months),
            FilterDate {
                weeks: Some(weeks), ..
            } => DateUnit::Weeks(weeks),
            FilterDate {
                days: Some(days), ..
            } => DateUnit::Days(days),
            FilterDate {
                hours: Some(hours), ..
            } => DateUnit::Hours(hours),
            FilterDate {
                minutes: Some(minutes),
                ..
            } => DateUnit::Minutes(minutes),
            FilterDate {
                seconds: Some(seconds),
                ..
            } => DateUnit::Seconds(seconds),
            FilterDate { .. } => unreachable!(),
        }
    }
}

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[cfg_attr(feature = "cli", group(required = true, multiple = false))]
pub struct FilterDate {
    /// specify number of years
    #[serde(rename = "years")]
    #[cfg_attr(feature = "cli", arg(long))]
    years: Option<u64>,
    /// specify number of months
    #[serde(rename = "months")]
    #[cfg_attr(feature = "cli", arg(long))]
    months: Option<u64>,
    /// specify number of weeks
    #[serde(rename = "weeks")]
    #[cfg_attr(feature = "cli", arg(long))]
    weeks: Option<u64>,
    /// specify number of days
    #[serde(rename = "days")]
    #[cfg_attr(feature = "cli", arg(long))]
    days: Option<u64>,
    /// specify number of hours
    #[serde(rename = "hours")]
    #[cfg_attr(feature = "cli", arg(long))]
    hours: Option<u64>,
    /// specify number of minutes
    #[serde(rename = "minutes")]
    #[cfg_attr(feature = "cli", arg(long))]
    minutes: Option<u64>,
    /// specify number of seconds
    #[serde(rename = "seconds")]
    #[cfg_attr(feature = "cli", arg(long))]
    seconds: Option<u64>,
}

/// [`OrganizeFilter`] contains filter variants that organize can
/// use to apply to locations.
#[cfg_attr(feature = "cli", derive(Subcommand))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OrganizeFilter {
    /// Matches locations by created date
    ///
    /// # Result
    ///
    /// The datetime the location was created.
    ///
    /// # Example
    ///
    /// Sort pdfs by year of creation
    ///
    /// ```yaml
    /// rules:
    ///    - name: Sort pdfs by year of creation
    ///      locations: "~/Documents"
    ///      filters:
    ///        - extension: pdf
    ///        - created
    ///      actions:
    ///        - move: "~/Documents/PDF/{created.year}/"
    /// ```
    #[serde(rename = "created")]
    Created {
        #[cfg_attr(feature = "cli", command(flatten))]
        date: FilterDate,
        #[cfg_attr(feature = "cli", arg(long))]
        mode: Interval,
    },
    /// Matches locations by the time the file was added to a folder
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
        #[cfg_attr(feature = "cli", command(flatten))]
        date: FilterDate,
        #[cfg_attr(feature = "cli", arg(long))]
        mode: Interval,
    },
    /// Matches locations by the time the file was last accessed
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
        #[cfg_attr(feature = "cli", command(flatten))]
        date: FilterDate,
        #[cfg_attr(feature = "cli", arg(long))]
        mode: Interval,
    },
    /// Matches locations by last modified date
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
        #[cfg_attr(feature = "cli", command(flatten))]
        date: FilterDate,
        #[cfg_attr(feature = "cli", arg(long))]
        mode: Interval,
    },
    /// A fast duplicate file finder
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
        detect_original_by: Option<DetectDuplicateBy>,
        #[cfg_attr(feature = "cli", arg(long))]
        reverse: bool,
    },
    /// Finds empty locations
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
    /// Filter by image EXIF data
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
    /// Filter by file extension
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
    /// Matches file content with the given regular expression
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
    /// Calculates the hash of a file
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
    /// Match locations by name
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
        arguments: NameFilterArgs,
        /// By default, the matching is case sensitive.
        ///
        /// Change this to `False` to use case insensitive matching.
        #[cfg_attr(feature = "cli", arg(long))]
        case_insensitive: bool,
    },
    /// Matches filenames with the given regular expression
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
    /// Matches locations by size
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
        // #[cfg_attr(feature = "cli", command(flatten))]
        // upper: SizeFilterUpperArgs,
        // #[cfg_attr(feature = "cli", command(flatten))]
        // lower: SizeFilterLowerArgs,
        #[cfg_attr(feature = "cli", arg(long))]
        condition: String,
    },
}

// TODO: Grouping CLI or better option?
// Enum variants would be better as it was intended
#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Display, Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "cli", group(required = false, multiple = true))]
pub struct SizeFilterLowerArgs {
    #[cfg_attr(feature = "cli", arg(long))]
    lower_value: Option<u64>,
    #[cfg_attr(feature = "cli", arg(long))]
    lower: Option<SizeConditionsRaw>,
    #[cfg_attr(feature = "cli", arg(long))]
    unit: Option<String>,
}
#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Display, Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "cli", group(required = false, multiple = true))]
pub struct SizeFilterUpperArgs {
    #[cfg_attr(feature = "cli", arg(long))]
    upper_value: Option<u64>,
    #[cfg_attr(feature = "cli", arg(long))]
    upper: Option<SizeConditionsRaw>,
    #[cfg_attr(feature = "cli", arg(long))]
    unit: Option<String>,
}

impl OrganizeFilter {
    pub fn get_filter(&self) -> Box<dyn FnMut(&DirEntry) -> bool> {
        match self {
            OrganizeFilter::Extension { exts } => Box::new(self.filter_by_extension(exts.clone())),
            OrganizeFilter::Name {
                arguments,
                case_insensitive,
            } => Box::new(self.filter_by_name(arguments.clone(), *case_insensitive)),
            OrganizeFilter::Empty => Box::new(self.filter_by_empty()),
            OrganizeFilter::Created { date, mode } => {
                Box::new(self.filter_by_created(*date, *mode))
            }
            OrganizeFilter::LastModified { date, mode } => {
                Box::new(self.filter_by_last_modified(*date, *mode))
            }
            OrganizeFilter::LastAccessed { date, mode } => {
                Box::new(self.filter_by_last_accessed(*date, *mode))
            }
            OrganizeFilter::Mimetype { mimetype } => {
                Box::new(self.filter_by_mimetype(mimetype.clone()))
            }
            OrganizeFilter::Size { condition } => todo!("not implemented (yet)!"),
            OrganizeFilter::Regex { expr: _ } => todo!("not implemented (yet)!"),
            OrganizeFilter::Exif => todo!("not implemented (yet)!"),
            OrganizeFilter::Filecontent { regex: _ } => todo!("not implemented (yet)!"),
            OrganizeFilter::Duplicate {
                detect_original_by: _,
                reverse: _,
            } => todo!("not implemented (yet)!"),
            #[cfg(target_os = "osx")]
            OrganizeFilter::Added { date, mode } => todo!("not implemented (yet)!"),
            #[cfg(target_os = "osx")]
            OrganizeFilter::LastUsed { date, mode } => todo!("not implemented (yet)!"),
        }
    }

    /// Returns `true` if the organize filter is [`DateCreated`].
    ///
    /// [`DateCreated`]: OrganizeFilter::DateCreated
    #[must_use]
    pub fn is_date_created(&self) -> bool {
        matches!(self, Self::Created { .. })
    }

    /// Returns `true` if the organize filter is [`Added`].
    ///
    /// [`Added`]: OrganizeFilter::Added
    #[must_use]
    #[cfg(target_os = "osx")]
    pub fn is_date_added(&self) -> bool {
        matches!(self, Self::Added { .. })
    }

    /// Returns `true` if the organize filter is [`LastUsed`].
    ///
    /// [`LastUsed`]: OrganizeFilter::LastUsed
    #[must_use]
    #[cfg(target_os = "osx")]
    pub fn is_date_last_used(&self) -> bool {
        matches!(self, Self::LastUsed { .. })
    }

    /// Returns `true` if the organize filter is [`LastModified`].
    ///
    /// [`LastModified`]: OrganizeFilter::LastModified
    #[must_use]
    pub fn is_date_last_modified(&self) -> bool {
        matches!(self, Self::LastModified { .. })
    }

    /// Returns `true` if the organize filter is [`Duplicate`].
    ///
    /// [`Duplicate`]: OrganizeFilter::Duplicate
    #[must_use]
    pub fn is_duplicate(&self) -> bool {
        matches!(self, Self::Duplicate { .. })
    }

    /// Returns `true` if the organize filter is [`Empty`].
    ///
    /// [`Empty`]: OrganizeFilter::Empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty { .. })
    }

    /// Returns `true` if the organize filter is [`Exif`].
    ///
    /// [`Exif`]: OrganizeFilter::Exif
    #[must_use]
    pub fn is_exif(&self) -> bool {
        matches!(self, Self::Exif)
    }

    /// Returns `true` if the organize filter is [`Extension`].
    ///
    /// [`Extension`]: OrganizeFilter::Extension
    #[must_use]
    pub fn is_extension(&self) -> bool {
        matches!(self, Self::Extension { .. })
    }

    /// Returns `true` if the organize filter is [`Filecontent`].
    ///
    /// [`Filecontent`]: OrganizeFilter::Filecontent
    #[must_use]
    pub fn is_filecontent(&self) -> bool {
        matches!(self, Self::Filecontent { .. })
    }

    /// Returns `true` if the organize filter is [`Hash`].
    ///
    /// [`Hash`]: OrganizeFilter::Hash
    #[must_use]
    #[cfg(feature = "research_organize")]
    pub fn is_hash(&self) -> bool {
        matches!(self, Self::Hash)
    }

    /// Returns `true` if the organize filter is [`MacOsTags`].
    ///
    /// [`MacOsTags`]: OrganizeFilter::MacOsTags
    #[must_use]
    #[cfg(target_os = "osx")]
    pub fn is_mac_os_tags(&self) -> bool {
        matches!(self, Self::MacOsTags { .. })
    }

    #[cfg(target_os = "osx")]
    pub fn as_mac_os_tags(&self) -> Option<&Vec<String>> {
        if let Self::MacOsTags { tags } = self {
            Some(tags)
        } else {
            None
        }
    }

    #[cfg(target_os = "osx")]
    pub fn try_into_mac_os_tags(self) -> Result<Vec<String>, Self> {
        if let Self::MacOsTags { tags } = self {
            Ok(tags)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the organize filter is [`Mimetype`].
    ///
    /// [`Mimetype`]: OrganizeFilter::Mimetype
    #[must_use]
    pub fn is_mimetype(&self) -> bool {
        matches!(self, Self::Mimetype { .. })
    }

    pub fn as_mimetype(&self) -> Option<&Vec<String>> {
        if let Self::Mimetype { mimetype } = self {
            Some(mimetype)
        } else {
            None
        }
    }

    pub fn try_into_mimetype(self) -> Result<Vec<String>, Self> {
        if let Self::Mimetype { mimetype } = self {
            Ok(mimetype)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the organize filter is [`Name`].
    ///
    /// [`Name`]: OrganizeFilter::Name
    #[must_use]
    pub fn is_name(&self) -> bool {
        matches!(self, Self::Name { .. })
    }

    /// Returns `true` if the organize filter is [`Regex`].
    ///
    /// [`Regex`]: OrganizeFilter::Regex
    #[must_use]
    pub fn is_regex(&self) -> bool {
        matches!(self, Self::Regex { .. })
    }

    pub fn as_regex(&self) -> Option<&String> {
        if let Self::Regex { expr: expression } = self {
            Some(expression)
        } else {
            None
        }
    }

    pub fn try_into_regex(self) -> Result<String, Self> {
        if let Self::Regex { expr: expression } = self {
            Ok(expression)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the organize filter is [`Size`].
    ///
    /// [`Size`]: OrganizeFilter::Size
    #[must_use]
    pub fn is_size(&self) -> bool {
        matches!(self, Self::Size { .. })
    }

    pub fn as_extension(&self) -> Option<&Vec<String>> {
        if let Self::Extension { exts: extensions } = self {
            Some(extensions)
        } else {
            None
        }
    }

    pub fn try_into_extension(self) -> Result<Vec<String>, Self> {
        if let Self::Extension { exts: extensions } = self {
            Ok(extensions)
        } else {
            Err(self)
        }
    }

    pub fn as_filecontent(&self) -> Option<&String> {
        if let Self::Filecontent { regex: expression } = self {
            Some(expression)
        } else {
            None
        }
    }

    pub fn try_into_filecontent(self) -> Result<String, Self> {
        if let Self::Filecontent { regex: expression } = self {
            Ok(expression)
        } else {
            Err(self)
        }
    }

    fn filter_by_extension(&self, exts: Vec<String>) -> impl FnMut(&DirEntry) -> bool {
        move |entry: &DirEntry| -> bool {
            let mut exts = exts.clone().into_iter();
            let entry = entry.clone();
            let file_path = entry.into_path();
            if let Some(extension) = file_path.extension() {
                let extension_str = extension.to_string_lossy();
                exts.any(|f| f == extension_str)
            } else {
                false
            }
        }
    }

    fn filter_by_name(
        &self,
        arguments: NameFilterArgs,
        case_insensitive: bool,
    ) -> impl FnMut(&DirEntry) -> bool {
        move |entry: &DirEntry| -> bool {
            let entry = entry.clone();
            let file_path = entry.into_path();
            let file_stem = file_path.file_stem();
            if let Some(file_name) = file_path.file_name() {
                let mut file_name_str = file_name.to_string_lossy().into_owned();

                if case_insensitive {
                    file_name_str = file_name_str.to_lowercase();
                }

                match &arguments {
                    NameFilterArgs {
                        starts_with: Some(sw),
                        ..
                    } => {
                        let mut sw = sw.clone();
                        if case_insensitive {
                            sw = sw.to_lowercase();
                        }

                        file_name_str.starts_with(&sw)
                    }
                    NameFilterArgs {
                        contains: Some(c), ..
                    } => {
                        let mut c = c.clone();
                        if case_insensitive {
                            c = c.to_lowercase();
                        }
                        file_name_str.contains(&c)
                    }
                    NameFilterArgs {
                        ends_with: Some(ew),
                        ..
                    } => {
                        let mut ew = ew.clone();
                        if case_insensitive {
                            ew = ew.to_lowercase();
                        }
                        if let Some(stem) = file_stem {
                            let mut new_stem = stem.to_string_lossy().into_owned();

                            if case_insensitive {
                                new_stem = new_stem.to_lowercase();
                            }

                            new_stem.ends_with(&ew)
                        } else {
                            file_name_str.ends_with(&ew)
                        }
                    }
                    NameFilterArgs { .. } => false,
                }
            } else {
                false
            }
        }
    }

    fn filter_by_empty(&self) -> impl FnMut(&DirEntry) -> bool {
        move |entry: &DirEntry| -> bool {
            if let Ok(metadata) = entry.metadata() {
                if entry.path().is_file() {
                    metadata.len() == 0
                } else if entry.path().is_dir() {
                    if let Ok(iter) = entry.path().read_dir() {
                        iter.count() == 0
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        }
    }

    fn filter_by_last_accessed(
        &self,
        date: FilterDate,
        mode: Interval,
    ) -> impl FnMut(&DirEntry) -> bool {
        move |entry: &DirEntry| {
            let metadata = entry.metadata().expect("getting metadata should not fail");
            let date_accessed = metadata
                .accessed()
                .expect("getting created date should not fail");
            Self::matches_date(date_accessed, date, mode)
        }
    }
    fn filter_by_last_modified(
        &self,
        date: FilterDate,
        mode: Interval,
    ) -> impl FnMut(&DirEntry) -> bool {
        move |entry: &DirEntry| {
            let metadata = entry.metadata().expect("getting metadata should not fail");
            let date_modified = metadata
                .modified()
                .expect("getting created date should not fail");
            Self::matches_date(date_modified, date, mode)
        }
    }
    fn filter_by_created(&self, date: FilterDate, mode: Interval) -> impl FnMut(&DirEntry) -> bool {
        move |entry: &DirEntry| {
            let metadata = entry.metadata().expect("getting metadata should not fail");
            let date_created = metadata
                .created()
                .expect("getting created date should not fail");
            Self::matches_date(date_created, date, mode)
        }
    }

    fn matches_date(item_date: std::time::SystemTime, date: FilterDate, mode: Interval) -> bool {
        let datetime_file: DateTime<Utc> = chrono::DateTime::from(item_date);
        let now = chrono::offset::Utc::now();

        let seconds_since_created = u64::try_from((now - datetime_file).num_seconds())
            .expect("subtraction of two datetimes can't be negative");

        let unit = DateUnit::from(date);
        match (unit, mode) {
            (DateUnit::Years(y), Interval::OlderThan)
                if y * 52 * 7 * 24 * 60 * 60 < seconds_since_created =>
            {
                true
            }
            (DateUnit::Years(y), Interval::NewerThan)
                if y * 52 * 7 * 24 * 60 * 60 >= seconds_since_created =>
            {
                true
            }
            (DateUnit::Months(m), Interval::OlderThan)
                if m * 4 * 7 * 24 * 60 * 60 < seconds_since_created =>
            {
                true
            }
            (DateUnit::Months(m), Interval::NewerThan)
                if m * 4 * 7 * 24 * 60 * 60 >= seconds_since_created =>
            {
                true
            }
            (DateUnit::Weeks(w), Interval::OlderThan)
                if w * 7 * 24 * 60 * 60 < seconds_since_created =>
            {
                true
            }
            (DateUnit::Weeks(w), Interval::NewerThan)
                if w * 7 * 24 * 60 * 60 >= seconds_since_created =>
            {
                true
            }
            (DateUnit::Days(d), Interval::OlderThan)
                if d * 24 * 60 * 60 < seconds_since_created =>
            {
                true
            }
            (DateUnit::Days(d), Interval::NewerThan)
                if d * 24 * 60 * 60 >= seconds_since_created =>
            {
                true
            }
            (DateUnit::Hours(h), Interval::OlderThan) if h * 60 * 60 < seconds_since_created => {
                true
            }
            (DateUnit::Hours(h), Interval::NewerThan) if h * 60 * 60 >= seconds_since_created => {
                true
            }
            (DateUnit::Minutes(m), Interval::OlderThan) if m * 60 < seconds_since_created => true,
            (DateUnit::Minutes(m), Interval::NewerThan) if m * 60 >= seconds_since_created => true,
            (DateUnit::Seconds(s), Interval::OlderThan) if s < seconds_since_created => true,
            (DateUnit::Seconds(s), Interval::NewerThan) if s >= seconds_since_created => true,
            (_, _) => false,
        }
    }

    fn filter_by_mimetype(&self, mimetype: Vec<String>) -> impl FnMut(&DirEntry) -> bool {
        move |entry| {
            let file_path = entry.clone().into_path();
            let mimetype = mimetype.clone();
            let Ok(Some(file_kind)) = infer::get_from_path(&file_path) else {
                return false
            };

            let file_mime_type = match file_kind.mime_type().parse::<mime::Mime>() {
                Ok(it) => it,
                Err(err) => {
                    eprintln!(
                        "couldn't determine mimetype of {}: {err}",
                        file_path.display()
                    );
                    return false;
                }
            };

            mimetype
                .into_iter()
                .map(|f| f.parse::<mime::Mime>())
                .filter_map(|r| r.map_err(|err| println!("{err}")).ok())
                .any(|f| f == file_mime_type)
        }
    }
}
