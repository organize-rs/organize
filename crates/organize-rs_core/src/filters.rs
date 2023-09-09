//! Filters that `organize` operates with
mod impl_;
mod impl_traits;
#[cfg(test)]
mod tests;

use std::fmt::Debug;

#[cfg(feature = "cli")]
use clap::{Args, Subcommand, ValueEnum};

use displaydoc::Display;

use jwalk::DirEntry;
use serde::{Deserialize, Serialize};
use serde_with::formats::CommaSeparator;
use serde_with::StringWithSeparator;

use serde_with::serde_as;

use crate::parsers::{period_range::PeriodRange, size_range::SizeRange};

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
    #[serde(rename = "created")]
    Created,
    /// Whatever file is visited first is the original.
    ///
    /// This depends on the order of your location entries.
    #[serde(rename = "first_seen")]
    FirstSeen,
    // TODO
    #[serde(rename = "hash")]
    Hash,
    /// The first file sorted by date of last modification is the original.
    #[serde(rename = "last_modified")]
    LastModified,
    /// The first entry sorted by name is the original.
    #[serde(rename = "name")]
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
#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Display, Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "cli", group(required = true, multiple = false))]
pub struct NameFilterArgs {
    // TODO: Not implemented, searching for alternatives
    /// A matching string in [simplematch-syntax](https://github.com/tfeldmann/simplematch)
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(default = "Option::default")]
    simple_match: Option<Vec<String>>,
    /// The filename must begin with the given string
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(default = "Option::default")]
    starts_with: Option<Vec<String>>,
    /// The filename must contain the given string
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(default = "Option::default")]
    contains: Option<Vec<String>>,
    /// The filename (without extension) must end with the given string
    #[cfg_attr(feature = "cli", arg(long))]
    #[serde(default = "Option::default")]
    ends_with: Option<Vec<String>>,
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

/// Contains filter variants that organize can
/// use to match [`jwalk::DirEntry`] properties
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
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Show the date the file was added to the folder
    ///      enabled: true
    ///      locations:
    ///         - !non_recursive
    ///           path: ~/Desktop
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///            - !date_added
    ///              range: 3d..14d
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !echo
    ///            msg: "Date added: {{date_added.strftime('%Y-%m-%d')}}"
    ///      tags:
    ///        - !custom Test::Filter::DateAdded
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
        range: Option<PeriodRange>,
    },
    /// Output all items
    ///
    /// # Result
    ///
    /// Careful! All items are returned, meaning in combination with
    /// an action like `Trash` it would move *all* files/folders to
    /// the trash bin.
    ///
    /// # Example
    ///
    /// Show all items in a folder
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Show all items in a folder
    ///      enabled: true
    ///      locations:
    ///         - !non_recursive
    ///           path: ~/Desktop
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///            - !all_items
    ///              i_agree_it_is_dangerous: true
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !echo
    ///            msg: "Item: {{entry.filename}}"
    ///      tags:
    ///        - !custom Test::Filter::AllItems
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
    #[serde(rename = "all_items")]
    AllItems {
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "bool::default")]
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
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Sort pdfs by year of creation
    ///      enabled: true
    ///      locations:
    ///         - !non_recursive
    ///           path: ~/Desktop
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///            - !extension
    ///              exts:
    ///                - pdf
    ///            - !created
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !move
    ///            dst: ~/Documents/PDF/{{entry.created.year}}/
    ///      tags:
    ///        - !custom Test::Filter::Created
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
        range: Option<PeriodRange>,
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
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Show all duplicate locations in your desktop and download folder (and their subfolders)
    ///      enabled: true
    ///      locations:
    ///         - !recursive
    ///           path: ~/Desktop
    ///           max_depth: 4
    ///           target: files
    ///         - !recursive
    ///           path: ~/Downloads
    ///           max_depth: 4
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///            - !duplicate
    ///              detect_original_by: name
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !echo
    ///            msg: "{{entry.duplicate}} is a duplicate of {{entry.original}}"
    ///      tags:
    ///        - !custom Test::Filter::Duplicate
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
    #[serde(rename = "duplicate")]
    Duplicate {
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "DuplicateKind::default")]
        detect_original_by: DuplicateKind,
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "bool::default")]
        reverse: bool,
    },
    /// Find empty locations
    ///
    /// # Example
    ///
    /// Recursively delete empty folders
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Recursively delete empty folders
    ///      enabled: true
    ///      locations:
    ///         - !recursive
    ///           path: ~/Desktop
    ///           max_depth: 10
    ///           target: folders
    ///      filter_groups:
    ///        - filters:
    ///            - !empty
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !delete
    ///      tags:
    ///        - !custom Test::Filter::EmptyFolders
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
    /// subfolder structure
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Copy all images which contain GPS information
    ///      enabled: true
    ///      locations:
    ///         - !recursive
    ///           path: ~/Pictures
    ///           max_depth: 10
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///            - !exif
    ///              contains:
    ///                - gps.gpsdate
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !copy
    ///            dst: ~/Pictures/with_gps/{{relative_path}}/
    ///      tags:
    ///        - !custom Test::Filter::ExifGps
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
    #[serde(rename = "exif")]
    Exif { contains: Vec<String> },
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
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Match multiple file extensions
    ///      enabled: true
    ///      locations:
    ///         - !recursive
    ///           path: ~/Desktop
    ///           max_depth: 10
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///            - !extension
    ///              exts:
    ///                - .jpg
    ///                - jpeg
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !echo
    ///            msg: "Found JPG file: {{entry.path}}"
    ///      tags:
    ///        - !custom Test::Filter::Extension
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Match an invoice with a regular expression and sort by customer
    ///      enabled: true
    ///      locations:
    ///         - !recursive
    ///           path: ~/Desktop
    ///           max_depth: 10
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///            - !file_content
    ///              expr: 'Invoice.*Customer (?P<customer>\w+)'
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !move
    ///            dst: ~/Documents/Invoices/{{file_content.customer}}/
    ///      tags:
    ///        - !custom Test::Filter::FileContent
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
    #[serde(rename = "file_content")]
    FileContent {
        #[cfg_attr(feature = "cli", arg(long))]
        expr: String,
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
    /// Show the hashes and size of your files
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Show the hashes and size of your files
    ///      enabled: true
    ///      locations:
    ///         - !recursive
    ///           path: ~/Desktop
    ///           max_depth: 10
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///            - !hash
    ///            - !size
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !echo
    ///            msg: "{{hash}} {{size.decimal}}"
    ///      tags:
    ///        - !custom Test::Filter::Hash
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
    #[cfg(feature = "research_organize")]
    #[serde(rename = "hash")]
    Hash,
    /// Defines a string that makes organize skip a location when found in the file name
    ///
    /// # Example
    ///
    /// Ignore file name
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Ignore file name
    ///      enabled: true
    ///      locations:
    ///         - !recursive
    ///           path: ~/Development
    ///           max_depth: 10
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///            - !extension
    ///              exts:
    ///                - toml
    ///            - !ignore_filename
    ///              in_name: "Cargo"
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !echo
    ///            msg: "Files discovered: {{entry}}"
    ///      tags:
    ///        - !custom Test::Filter::IgnoreName
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
    #[serde(rename = "ignore_filename")]
    IgnoreName {
        /// Matches for these Strings in the Filename
        // #[cfg_attr(feature = "cli", arg(long))]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
        in_name: Vec<String>,
    },
    /// Defines a string that makes organize skip a location when found in the full path
    ///
    /// # Example
    ///
    /// Ignore in path
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Ignore in path
    ///      enabled: true
    ///      locations:
    ///         - !recursive
    ///           path: ~/Development
    ///           max_depth: 10
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///            - !extension
    ///              exts:
    ///                - toml
    ///            - !ignore_path
    ///              in_path: ".git"
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !echo
    ///            msg: "Files discovered: {{entry}}"
    ///      tags:
    ///        - !custom Test::Filter::IgnorePath
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
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
    /// Show the date the location was last accessed
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Show the date the location was last accessed
    ///      enabled: true
    ///      locations:
    ///         - !recursive
    ///           path: ~/Desktop
    ///           max_depth: 10
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///            - !last_accessed
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !echo
    ///            msg: "Date last used: {{entry.metadata.last_accessed}}"
    ///      tags:
    ///        - !custom Test::Filter::LastAccessed
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
        range: Option<PeriodRange>,
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
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Sort pdfs by year of last modification
    ///      enabled: true
    ///      locations:
    ///         - !recursive
    ///           path: ~/Desktop
    ///           max_depth: 10
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///            - !last_modified
    ///            - !extension
    ///              exts:
    ///                - pdf
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !move
    ///            dst: ~/Documents/PDF/{{entry.metadata.lastmodified.year}}/
    ///      tags:
    ///        - !custom Test::Filter::LastModified
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
        range: Option<PeriodRange>,
    },
    /// Filter by macOS tags
    ///
    /// # Example
    ///
    /// All locations with a tag 'Invoice' (any color) or with a green tag
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: All locations with a tag 'Invoice' (any color) or with a green tag
    ///      enabled: true
    ///      locations:
    ///         - !non_recursive
    ///           path: ~/Downloads
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///           - !macos_tags
    ///               - "Invoice (*)"
    ///               - "* (green)"
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !echo
    ///            msg: "Match found!"
    ///      tags:
    ///        - !custom Test::Filter::MacOsTags
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Filter by 'image' mimetype
    ///      enabled: true
    ///      locations:
    ///         - !non_recursive
    ///           path: ~/Downloads
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///           - !mimetype
    ///             mime: image/*
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !echo
    ///            msg: "This file is an image: {{entry.mimetype}}"
    ///      tags:
    ///        - !custom Test::Filter::Mimetype
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
    #[serde(rename = "mimetype")]
    Mimetype {
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
        mime: Vec<String>,
    },
    /// Match locations by their name
    ///
    /// # Example
    ///
    /// Match all locations starting with 'A' or 'B' containing '5' or
    /// '6' and ending with '_end'.
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Match locations by name
    ///      enabled: true
    ///      locations:
    ///         - !non_recursive
    ///           path: ~/Desktop
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///           - !name
    ///             starts_with:
    ///               - A
    ///               - B
    ///             ends_with:
    ///               - _end
    ///             contains:
    ///               - "5"
    ///               - "6"
    ///             case_insensitive: true
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !echo
    ///            msg: "Found a match {{entry}}."
    ///      tags:
    ///        - !custom Test::Filter::Name
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
        #[serde(rename = "case_insensitive")]
        #[serde(default = "bool::default")]
        case_insensitive: bool,
    },
    /// Don't use any filter (Default)
    ///
    /// # Result
    ///
    /// Empty / no items due to the risk otherwise if it's used in
    /// combination with an action, that the action will be applied
    /// to all results.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: No locations
    ///      enabled: true
    ///      locations:
    ///         - !non_recursive
    ///           path: ~/Desktop
    ///           target: both
    ///      filter_groups:
    ///        - filters:
    ///           - !no_items
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !echo
    ///            msg: "Running on NoFilter."
    ///      tags:
    ///        - !custom Test::Filter::NoFilter
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
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
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Match an invoice with a regular expression
    ///      enabled: true
    ///      locations:
    ///         - !non_recursive
    ///           path: ~/Desktop
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///           - !regex
    ///             expr: '^RG(?P<the_number>\d{12})-sig\.pdf$'
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !move
    ///            dst: ~/Documents/Invoices/1und1/{{regex.the_number}}.pdf
    ///      tags:
    ///        - !custom Test::Filter::Regex
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Trash big downloads
    ///      enabled: true
    ///      locations:
    ///         - !non_recursive
    ///           path: ~/Downloads
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///           - !size
    ///             range: 0.5GB..
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !trash
    ///      tags:
    ///        - !custom Test::Filter::TrashDownloads
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
    ///
    /// # Example
    ///
    /// Move all JPEGs bigger > 1MB and <10 MB. Search all subfolders and
    /// keep the original relative path.
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Move all JPEGs bigger > 1MB and <10 MB
    ///      enabled: true
    ///      locations:
    ///         - !recursive
    ///           path: ~/Pictures
    ///           max_depth: 10
    ///           target: both
    ///      filter_groups:
    ///        - filters:
    ///           - !size
    ///             range: 1MB..10MB
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !move
    ///            dst: ~/Pictures/sorted/{{relative_path}}/
    ///      tags:
    ///        - !custom Test::Filter::SortPicturesBySize
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
        range: Option<SizeRange>,
    },
}
