//! FilterSelf::Extension()he config file(s)
//! and `organize` operates with

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

pub struct FilterFn(Box<dyn FnMut(walkdir::DirEntry) -> bool>);

impl std::ops::Deref for FilterFn {
    type Target = Box<dyn FnMut(walkdir::DirEntry) -> bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Comparison conditions for dates
#[derive(Debug, Clone, Copy, Deserialize, Serialize, ValueEnum)]
pub enum OlderNewer {
    Older,
    Newer,
}

impl OlderNewer {
    /// Returns `true` if the older newer is [`Older`].
    ///
    /// [`Older`]: OlderNewer::Older
    #[must_use]
    pub fn is_older(&self) -> bool {
        matches!(self, Self::Older)
    }

    /// Returns `true` if the older newer is [`Newer`].
    ///
    /// [`Newer`]: OlderNewer::Newer
    #[must_use]
    pub fn is_newer(&self) -> bool {
        matches!(self, Self::Newer)
    }
}

impl Default for OlderNewer {
    fn default() -> Self {
        Self::Older
    }
}

/// Duplication detection
#[derive(Debug, Clone, Copy, Deserialize, Serialize, ValueEnum)]
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

/// Comparison conditions for the size of files
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum SizeConditions {
    GreaterThan(u64),
    GreaterOrEqual(u64),
    SmallerThan(u64),
    SmallerOrEqual(u64),
    EqualTo(u64),
}

/// Comparison conditions for the size of files
#[derive(Debug, Clone, Copy, Deserialize, Serialize, ValueEnum)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FilterDate {
    /// specify number of years
    #[serde(rename = "years")]
    Years(u16),
    /// specify number of months
    #[serde(rename = "months")]
    Months(u64),
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
#[derive(Debug, Clone, Deserialize, Serialize, ValueEnum)]
pub enum FilterDateRaw {
    /// specify number of years
    #[serde(rename = "years")]
    Years,
    /// specify number of months
    #[serde(rename = "months")]
    Months,
    /// specify number of weeks
    #[serde(rename = "weeks")]
    Weeks,
    /// specify number of days
    #[serde(rename = "days")]
    Days,
    /// specify number of hours
    #[serde(rename = "hours")]
    Hours,
    /// specify number of minutes
    #[serde(rename = "minutes")]
    Minutes,
    /// specify number of seconds
    #[serde(rename = "seconds")]
    Seconds,
}

/// [`OrganizeFilter`] contains filter variants that organize can
/// use to apply to files/folders.
#[derive(Debug, Clone, Deserialize, Serialize, Parser)]
pub enum OrganizeFilter {
    /// Matches files / folders by created date
    ///
    /// # Result
    ///
    /// The datetime the file / folder was created.
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
    DateCreated {
        #[clap(long)]
        value: u64,
        #[clap(long)]
        date: FilterDateRaw,
        #[clap(long)]
        mode: OlderNewer,
    },
    /// Matches files by the time the file was added to a folder
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
    DateAdded {
        #[clap(long)]
        value: u64,
        #[clap(long)]
        date: FilterDateRaw,
        #[clap(long)]
        mode: OlderNewer,
    },
    /// Matches files by the time the file was last used
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
    ///        - date_lastused
    ///      actions:
    ///        - echo: "Date last used: {date_lastused.strftime('%Y-%m-%d')}"
    /// ```
    #[cfg(target_os = "osx")]
    #[serde(rename = "date_lastused")]
    DateLastUsed {
        #[clap(long)]
        value: u64,
        #[clap(long)]
        date: FilterDateRaw,
        #[clap(long)]
        mode: OlderNewer,
    },
    /// Matches files by last modified date
    ///
    /// # Result
    ///
    /// The datetime the files / folders was last modified.
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
    DateLastModified {
        #[clap(long)]
        value: u64,
        #[clap(long)]
        date: FilterDateRaw,
        #[clap(long)]
        mode: OlderNewer,
    },
    /// A fast duplicate file finder
    ///
    /// This filter compares files byte by byte and finds identical
    /// files with potentially different filenames.
    ///
    /// You can reverse the sorting method by prefixing a `-`.
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
    /// Show all duplicate files in your desktop and download folder
    /// (and their subfolders)
    ///
    /// ```yaml
    /// rules:
    ///   - name: Show all duplicate files in your desktop and download folder (and their subfolders)
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
        #[clap(long)]
        detect_original_by: Option<DetectDuplicateBy>,
    },
    /// Finds empty dirs and files
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
        /// The file extensions to match (does not need to start
        /// with a colon)
        #[clap(long)]
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
        #[clap(long)]
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
    /// All files with a tag 'Invoice' (any color) or with a green tag
    ///
    /// ```yaml
    /// rules:
    ///   - name: "All files with a tag 'Invoice' (any color) or with a green tag"
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
        #[clap(long)]
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
        #[clap(long)]
        mimetype: Vec<String>,
    },
    /// Match files and folders by name
    ///
    /// # Example
    ///
    /// Match all files starting with 'A' or 'B' containing '5' or
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
        // TODO: alternative?
        /// A matching string in [simplematch-syntax](https://github.com/tfeldmann/simplematch)
        #[clap(long)]
        simple_match: String,
        /// The filename must begin with the given string
        #[clap(long)]
        starts_with: String,
        /// The filename must contain the given string
        #[clap(long)]
        contains: String,
        /// The filename (without extension) must end with the given string
        #[clap(long)]
        ends_with: String,
        /// By default, the matching is case sensitive.
        ///
        /// Change this to False to use case insensitive matching.
        #[clap(long)]
        case_sensitive: bool,
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
        #[clap(long)]
        expr: String,
    },
    /// Matches files and folders by size
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
        #[clap(long)]
        upper_value: Option<u64>,
        #[clap(long)]
        upper: Option<SizeConditionsRaw>,
        #[clap(long)]
        lower_value: Option<u64>,
        #[clap(long)]
        lower: Option<SizeConditionsRaw>,
    },
}

impl OrganizeFilter {
    pub fn get_filter(&self) -> FilterFn {
        match self {
            OrganizeFilter::DateCreated { value, date, mode } => todo!(),
            OrganizeFilter::DateLastModified { value, date, mode } => todo!(),
            OrganizeFilter::Duplicate { detect_original_by } => todo!(),
            OrganizeFilter::Empty => todo!(),
            OrganizeFilter::Exif => todo!(),
            OrganizeFilter::Extension { exts } => todo!(),
            OrganizeFilter::Filecontent { regex } => todo!(),
            OrganizeFilter::Mimetype { mimetype } => todo!(),
            OrganizeFilter::Name {
                simple_match,
                starts_with,
                contains,
                ends_with,
                case_sensitive,
            } => todo!(),
            OrganizeFilter::Regex { expr } => todo!(),
            OrganizeFilter::Size {
                upper_value,
                upper,
                lower_value,
                lower,
            } => todo!(),
        }
    }

    /// Returns `true` if the organize filter is [`DateCreated`].
    ///
    /// [`DateCreated`]: OrganizeFilter::DateCreated
    #[must_use]
    pub fn is_date_created(&self) -> bool {
        matches!(self, Self::DateCreated { .. })
    }

    /// Returns `true` if the organize filter is [`DateAdded`].
    ///
    /// [`DateAdded`]: OrganizeFilter::DateAdded
    #[must_use]
    #[cfg(target_os = "osx")]
    pub fn is_date_added(&self) -> bool {
        matches!(self, Self::DateAdded { .. })
    }

    /// Returns `true` if the organize filter is [`DateLastUsed`].
    ///
    /// [`DateLastUsed`]: OrganizeFilter::DateLastUsed
    #[must_use]
    #[cfg(target_os = "osx")]
    pub fn is_date_last_used(&self) -> bool {
        matches!(self, Self::DateLastUsed { .. })
    }

    /// Returns `true` if the organize filter is [`DateLastModified`].
    ///
    /// [`DateLastModified`]: OrganizeFilter::DateLastModified
    #[must_use]
    pub fn is_date_last_modified(&self) -> bool {
        matches!(self, Self::DateLastModified { .. })
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
        matches!(self, Self::Empty)
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

    pub fn as_duplicate(&self) -> Option<&Option<DetectDuplicateBy>> {
        if let Self::Duplicate { detect_original_by } = self {
            Some(detect_original_by)
        } else {
            None
        }
    }

    pub fn try_into_duplicate(self) -> Result<Option<DetectDuplicateBy>, Self> {
        if let Self::Duplicate { detect_original_by } = self {
            Ok(detect_original_by)
        } else {
            Err(self)
        }
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
}
