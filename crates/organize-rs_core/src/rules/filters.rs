//! Filters that `organize` operates with

use chrono::{DateTime, Utc};
#[cfg(feature = "cli")]
use clap::{Args, Subcommand, ValueEnum};

use displaydoc::Display;

use serde::{Deserialize, Serialize};
use walkdir::DirEntry;

use crate::parsers::{PeriodRange, SizeRange};

type FilterClosure = Box<dyn FnMut(&DirEntry) -> bool>;

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

/// Arguments for `name` filter
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
    Years(f64),
    /// specify number of months
    #[serde(rename = "months")]
    Months(f64),
    /// specify number of weeks
    #[serde(rename = "weeks")]
    Weeks(f64),
    /// specify number of days
    #[serde(rename = "days")]
    Days(f64),
    /// specify number of hours
    #[serde(rename = "hours")]
    Hours(f64),
    /// specify number of minutes
    #[serde(rename = "minutes")]
    Minutes(f64),
    /// specify number of seconds
    #[serde(rename = "seconds")]
    Seconds(f64),
}

impl DateUnit {
    pub fn into_seconds(&self) -> f64 {
        match self {
            DateUnit::Years(y) => *y * 52f64 * 7f64 * 24f64 * 60f64 * 60f64,
            DateUnit::Months(mo) => *mo * 4f64 * 7f64 * 24f64 * 60f64 * 60f64,
            DateUnit::Weeks(w) => *w * 7f64 * 24f64 * 60f64 * 60f64,
            DateUnit::Days(d) => *d * 24f64 * 60f64 * 60f64,
            DateUnit::Hours(h) => *h * 60f64 * 60f64,
            DateUnit::Minutes(m) => *m * 60f64,
            DateUnit::Seconds(s) => *s,
        }
    }
}

impl From<(f64, &str)> for DateUnit {
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
pub enum OrganizeFilter {
    /// Don't use any filter.
    ///
    /// # Result
    ///
    /// Empty / no items due to the risk otherwise if it's used in
    /// combination with an action, that the action will be applied
    /// to all results.
    NoFilter,
    /// Output all items.
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
    /// Ignore expression in path
    IgnorePath {
        /// Matches for these Strings in the whole Path
        // #[cfg_attr(feature = "cli", arg(long))]
        in_path: Vec<String>,
    },
    /// Ignore expression in file name
    IgnoreName {
        /// Matches for these Strings in the Filename
        // #[cfg_attr(feature = "cli", arg(long))]
        in_name: Vec<String>,
    },
}

impl Default for OrganizeFilter {
    fn default() -> Self {
        Self::NoFilter
    }
}

impl OrganizeFilter {
    pub fn get_filter(&self) -> FilterClosure {
        match self {
            OrganizeFilter::NoFilter => Box::new(move |_entry| false),
            OrganizeFilter::AllItems {
                i_agree_it_is_dangerous,
            } => Box::new({
                let i_agree_it_is_dangerous = i_agree_it_is_dangerous.to_owned();
                move |_entry: &DirEntry| i_agree_it_is_dangerous
            }),
            OrganizeFilter::IgnorePath { in_path } => {
                Box::new(self.filter_ignore_by_str_in_path(in_path))
            }
            OrganizeFilter::IgnoreName { in_name } => {
                Box::new(self.filter_ignore_by_str_in_name(in_name))
            }
            OrganizeFilter::Extension { exts } => Box::new(self.filter_by_extension(exts)),
            OrganizeFilter::Name {
                arguments,
                case_insensitive,
            } => Box::new(self.filter_by_name(arguments, *case_insensitive)),
            OrganizeFilter::Empty => Box::new(self.filter_by_empty()),
            OrganizeFilter::Created { range } => Box::new(self.filter_by_created(range)),
            OrganizeFilter::LastModified { range } => Box::new(self.filter_by_last_modified(range)),
            OrganizeFilter::LastAccessed { range } => Box::new(self.filter_by_last_accessed(range)),
            OrganizeFilter::Mimetype { mimetype } => Box::new(self.filter_by_mimetype(mimetype)),
            OrganizeFilter::Size { range } => Box::new(self.filter_by_size(range)),
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

    /// Returns `true` if the organize filter is [`Created`].
    ///
    /// [`Created`]: OrganizeFilter::Created
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
    pub fn is_last_modified(&self) -> bool {
        matches!(self, Self::LastModified { .. })
    }

    /// Returns `true` if the organize filter is [`LastAccessed`].
    ///
    /// [`LastAccessed`]: OrganizeFilter::LastAccessed
    #[must_use]
    pub fn is_last_accessed(&self) -> bool {
        matches!(self, Self::LastAccessed { .. })
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

    fn filter_by_extension(&self, exts: &[String]) -> impl FnMut(&DirEntry) -> bool {
        let exts = exts.to_owned();
        move |entry: &DirEntry| -> bool {
            let entry = entry.clone();
            let file_path = entry.into_path();
            let Some(extension) = file_path.extension() else {
                return false
            };
            let Some(extension_str) = extension.to_str() else {
                    return false
                };
            exts.iter().any(|f| f == extension_str)
        }
    }

    fn filter_by_name(
        &self,
        arguments: &NameFilterArgs,
        case_insensitive: bool,
    ) -> impl FnMut(&DirEntry) -> bool {
        let arguments = arguments.clone();

        move |entry: &DirEntry| -> bool {
            let entry = entry.clone();
            let file_path = entry.into_path();
            let file_stem = file_path.file_stem().and_then(|f| f.to_str()).map(|f| {
                if case_insensitive {
                    f.to_lowercase()
                } else {
                    f.to_owned()
                }
            });

            let file_name_str = file_path
                .file_name()
                .and_then(|f| f.to_str())
                .map(|f| {
                    if case_insensitive {
                        f.to_lowercase()
                    } else {
                        f.to_owned()
                    }
                })
                .expect("should be able to unpack file name.");

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
                        stem.ends_with(&ew)
                    } else {
                        file_name_str.ends_with(&ew)
                    }
                }
                NameFilterArgs { .. } => false,
            }
        }
    }

    fn filter_by_empty(&self) -> impl FnMut(&DirEntry) -> bool {
        move |entry: &DirEntry| -> bool {
            entry
                .metadata()
                .map(|e| {
                    if entry.path().is_file() {
                        e.len() == 0
                    } else if entry.path().is_dir() {
                        entry.path().read_dir().map_or(false, |f| f.count() == 0)
                    } else {
                        false
                    }
                })
                .unwrap_or(false)
        }
    }

    fn filter_by_last_accessed(&self, range: &PeriodRange) -> impl FnMut(&DirEntry) -> bool {
        let range = range.clone();

        move |entry: &DirEntry| {
            entry.metadata().ok().map_or(false, |f| {
                let Ok(sys_time) = f.accessed() else {
                        return false
                    };
                Self::matches_date(&sys_time, &range)
            })
        }
    }
    fn filter_by_last_modified(&self, range: &PeriodRange) -> impl FnMut(&DirEntry) -> bool {
        let range = range.clone();

        move |entry: &DirEntry| {
            entry.metadata().ok().map_or(false, |f| {
                let Ok(sys_time) = f.modified() else {
                        return false
                    };
                Self::matches_date(&sys_time, &range)
            })
        }
    }
    fn filter_by_created(&self, range: &PeriodRange) -> impl FnMut(&DirEntry) -> bool {
        let range = range.clone();

        move |entry: &DirEntry| {
            entry.metadata().ok().map_or(false, |f| {
                let Ok(sys_time) = f.created() else {
                        return false
                    };
                Self::matches_date(&sys_time, &range)
            })
        }
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

    fn filter_by_mimetype(&self, mimetype: &[String]) -> impl FnMut(&DirEntry) -> bool {
        let mimetype = mimetype.to_owned();

        move |entry| {
            let Ok(Some(file_kind)) = infer::get_from_path(entry.path()) else {
                return false
            };

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
        }
    }

    fn filter_by_size(&self, range: &SizeRange) -> impl FnMut(&DirEntry) -> bool {
        let range = range.clone();

        move |entry| {
            if let Ok(metadata) = entry.metadata() {
                range.in_range(metadata.len() as f64)
            } else {
                false
            }
        }
    }

    fn filter_ignore_by_str_in_name(
        &self,
        ignore_name: &[String],
    ) -> impl FnMut(&DirEntry) -> bool {
        let ignore_name = ignore_name.to_owned();

        move |f| {
            let Some(file_name) = f.file_name().to_str() else { return true };
            let file_name_str = file_name.to_string();
            !ignore_name.iter().any(|pat| file_name_str.contains(pat))
        }
    }

    fn filter_ignore_by_str_in_path(
        &self,
        ignore_path: &[String],
    ) -> impl FnMut(&DirEntry) -> bool {
        let ignore_path = ignore_path.to_owned();

        move |f| {
            let Some(path) = f.path().to_str() else { return true };
            let path_str = path.to_string();
            !ignore_path.iter().any(|pat| path_str.contains(pat))
        }
    }
}
