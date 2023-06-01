//! Filters that `organize` operates with
mod impl_;
mod impl_traits;
#[cfg(test)]
mod tests;

use std::fmt::Debug;

#[cfg(feature = "cli")]
use clap::{Args, Subcommand, ValueEnum};

use displaydoc::Display;

use itertools::Itertools;
use jwalk::DirEntry;
use serde::{Deserialize, Serialize};
use serde_with::formats::CommaSeparator;
use serde_with::StringWithSeparator;

use serde_with::serde_as;

use crate::parsers::{PeriodRange, SizeRange};

pub type FilterClosure<'a, C> = Box<dyn FnMut(&DirEntry<C>) -> bool + 'a>;
pub type FilterClosureCollection<'a, C> = Vec<FilterClosure<'a, C>>;
pub type FilterFilterClosureSliceMut<'a, C> = &'a mut [Box<dyn FnMut(&DirEntry<C>) -> bool>];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CullKind {
    Retain,
    Bump,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display, Default)]
#[serde(transparent)]
pub struct FilterGroupCollection(Vec<FilterGroup<Vec<FilterKind>>>);

impl FilterGroupCollection {
    pub fn from_vec(vec: Vec<FilterGroup<Vec<FilterKind>>>) -> Self {
        Self(vec)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FilterCollection(Vec<(FilterApplicationKind, FilterOperationKind<FilterKind>)>);

/// Should filters be negated
#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum FilterOperationKind<T> {
    /// Invert {0}
    Invert(T),
    /// Apply {0}
    Apply(T),
}

/// Should filter results be included or excluded
#[derive(Debug, Clone, Deserialize, Serialize, Display, Copy)]
pub enum FilterGroupOperationKind {
    /// Exclude
    #[serde(rename = "include")]
    Exclude,
    /// Include
    #[serde(rename = "exclude")]
    Include,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub struct FilterGroup<T> {
    #[serde(rename = "results")]
    pub operation: FilterGroupOperationKind,
    #[serde(rename = "match")]
    pub mode: FilterApplicationKind,
    pub filters: T,
}

/// Application of filters, so whether "all", "any" or "none"
/// of the filters must apply
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(
    Debug, Clone, Deserialize, Serialize, Display, PartialEq, Eq, PartialOrd, Ord, Copy, Hash,
)]
#[non_exhaustive]
pub enum FilterApplicationKind {
    /// All of the filters need to apply
    #[serde(rename = "all")]
    All,
    /// Any of the filters need to apply
    #[serde(rename = "any")]
    Any,
    /// None of the filters need to apply
    #[serde(rename = "none")]
    None,
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

/// Arguments for `name` filter
#[serde_as]
#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Display, Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "cli", group(required = true, multiple = false))]
pub struct NameFilterArgs {
    // TODO: Not implemented, searching for alternatives
    /// A matching string in [simplematch-syntax](https://github.com/tfeldmann/simplematch)
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    simple_match: Vec<String>,
    /// The filename must begin with the given string
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    starts_with: Vec<String>,
    /// The filename must contain the given string
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    contains: Vec<String>,
    /// The filename (without extension) must end with the given string
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
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

/// [`OrganizeFilter`] contains filter variants that organize can
/// use to apply to locations.
#[serde_as]
#[cfg_attr(feature = "cli", derive(Subcommand))]
#[derive(Debug, Clone, Deserialize, Serialize)]
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
    #[serde(rename = "added")]
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
    #[serde(rename = "all_items")]
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
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
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
    #[serde(rename = "file_content")]
    FileContent {
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
    #[serde(rename = "ignore_filename")]
    IgnoreName {
        /// Matches for these Strings in the Filename
        // #[cfg_attr(feature = "cli", arg(long))]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
        in_name: Vec<String>,
    },
    /// Defines a string that makes organize skip a location when found in the full path
    #[serde(rename = "ignore_path")]
    IgnorePath {
        /// Matches for these Strings in the whole Path
        // #[cfg_attr(feature = "cli", arg(long))]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
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
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
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
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
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
    #[serde(rename = "no_items")]
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
    /// Match files by their size
    ///
    /// Accepts file size conditions, e.g: "500MB..", "..20kb", "0KB..", "10KiB..".
    ///
    /// It is possible to define both lower and upper conditions like this:
    /// "20kb..1TB", "20Mb..25Mb".
    ///
    /// The filter will match if all given conditions are satisfied.
    ///
    /// - Accepts all units from KB to TB.
    /// - Maximum size is 4 TB.
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
    ///     target: files
    ///     filters:
    ///       - size: "0.5GB.."
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
