//! Actions that can be used in the config file and
//! `organize` applieds to matching rules

use serde::{Deserialize, Serialize};

/// Colours for `MacOS` tags
#[cfg(target_os = "osx")]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum MacOsTagColours {
    None,
    Gray,
    Green,
    Purple,
    Blue,
    Yellow,
    Red,
    Orange,
}

#[cfg(target_os = "osx")]
impl MacOsTagColours {
    /// Returns `true` if the mac os tag colours is [`None`].
    ///
    /// [`None`]: MacOsTagColours::None
    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` if the mac os tag colours is [`Gray`].
    ///
    /// [`Gray`]: MacOsTagColours::Gray
    #[must_use]
    pub fn is_gray(&self) -> bool {
        matches!(self, Self::Gray)
    }

    /// Returns `true` if the mac os tag colours is [`Green`].
    ///
    /// [`Green`]: MacOsTagColours::Green
    #[must_use]
    pub fn is_green(&self) -> bool {
        matches!(self, Self::Green)
    }

    /// Returns `true` if the mac os tag colours is [`Purple`].
    ///
    /// [`Purple`]: MacOsTagColours::Purple
    #[must_use]
    pub fn is_purple(&self) -> bool {
        matches!(self, Self::Purple)
    }

    /// Returns `true` if the mac os tag colours is [`Blue`].
    ///
    /// [`Blue`]: MacOsTagColours::Blue
    #[must_use]
    pub fn is_blue(&self) -> bool {
        matches!(self, Self::Blue)
    }

    /// Returns `true` if the mac os tag colours is [`Yellow`].
    ///
    /// [`Yellow`]: MacOsTagColours::Yellow
    #[must_use]
    pub fn is_yellow(&self) -> bool {
        matches!(self, Self::Yellow)
    }

    /// Returns `true` if the mac os tag colours is [`Red`].
    ///
    /// [`Red`]: MacOsTagColours::Red
    #[must_use]
    pub fn is_red(&self) -> bool {
        matches!(self, Self::Red)
    }

    /// Returns `true` if the mac os tag colours is [`Orange`].
    ///
    /// [`Orange`]: MacOsTagColours::Orange
    #[must_use]
    pub fn is_orange(&self) -> bool {
        matches!(self, Self::Orange)
    }
}

#[cfg(target_os = "osx")]
impl Default for MacOsTagColours {
    fn default() -> Self {
        Self::None
    }
}

/// Actions for conflict resolution
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum OnConflict {
    Skip,
    Overwrite,
    Trash,
    RenameNew,
    RenameExisting,
}

impl OnConflict {
    /// Returns `true` if [`OnConflict`] is [`Skip`].
    ///
    /// [`Skip`]: OnConflict::Skip
    #[must_use]
    pub fn is_skip(&self) -> bool {
        matches!(self, Self::Skip)
    }

    /// Returns `true` if [`OnConflict`] is [`Overwrite`].
    ///
    /// [`Overwrite`]: OnConflict::Overwrite
    #[must_use]
    pub fn is_overwrite(&self) -> bool {
        matches!(self, Self::Overwrite)
    }

    /// Returns `true` if [`OnConflict`] is [`Trash`].
    ///
    /// [`Trash`]: OnConflict::Trash
    #[must_use]
    pub fn is_trash(&self) -> bool {
        matches!(self, Self::Trash)
    }

    /// Returns `true` if [`OnConflict`] is [`RenameNew`].
    ///
    /// [`RenameNew`]: OnConflict::RenameNew
    #[must_use]
    pub fn is_rename_new(&self) -> bool {
        matches!(self, Self::RenameNew)
    }

    /// Returns `true` if [`OnConflict`] is [`RenameExisting`].
    ///
    /// [`RenameExisting`]: OnConflict::RenameExisting
    #[must_use]
    pub fn is_rename_existing(&self) -> bool {
        matches!(self, Self::RenameExisting)
    }
}

impl Default for OnConflict {
    fn default() -> Self {
        Self::RenameNew
    }
}

/// Support template strings
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum TemplateStrings {
    Filename,
    Counter,
    Extension,
}

impl TemplateStrings {
    /// Returns `true` if [`TemplateStrings`] is [`Filename`].
    ///
    /// [`Filename`]: TemplateStrings::Filename
    #[must_use]
    pub fn is_filename(&self) -> bool {
        matches!(self, Self::Filename)
    }

    /// Returns `true` if [`TemplateStrings`] is [`Counter`].
    ///
    /// [`Counter`]: TemplateStrings::Counter
    #[must_use]
    pub fn is_counter(&self) -> bool {
        matches!(self, Self::Counter)
    }

    /// Returns `true` if [`TemplateStrings`] is [`Extension`].
    ///
    /// [`Extension`]: TemplateStrings::Extension
    #[must_use]
    pub fn is_extension(&self) -> bool {
        matches!(self, Self::Extension)
    }
}

/// A template for renaming a file
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RenameTemplate(Vec<TemplateStrings>);

impl Default for RenameTemplate {
    fn default() -> Self {
        Self(vec![
            TemplateStrings::Filename,
            TemplateStrings::Counter,
            TemplateStrings::Extension,
        ])
    }
}

/// Mode how should be written to a file
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WriteMode {
    /// append text to the file
    Append,
    /// insert text as first line
    Prepend,
    /// overwrite content with text
    Overwrite,
}

impl WriteMode {
    /// Returns `true` if the write mode is [`Append`].
    ///
    /// [`Append`]: WriteMode::Append
    #[must_use]
    pub fn is_append(&self) -> bool {
        matches!(self, Self::Append)
    }

    /// Returns `true` if the write mode is [`Prepend`].
    ///
    /// [`Prepend`]: WriteMode::Prepend
    #[must_use]
    pub fn is_prepend(&self) -> bool {
        matches!(self, Self::Prepend)
    }

    /// Returns `true` if the write mode is [`Overwrite`].
    ///
    /// [`Overwrite`]: WriteMode::Overwrite
    #[must_use]
    pub fn is_overwrite(&self) -> bool {
        matches!(self, Self::Overwrite)
    }
}

impl Default for WriteMode {
    fn default() -> Self {
        Self::Append
    }
}

/// A filename that should be written to
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WriteFile(String);

impl Default for WriteFile {
    fn default() -> Self {
        Self("organize_out.txt".to_string())
    }
}

// TODO: Shell supported
// adapted from: https://organize.readthedocs.io/en/latest/actions/
//
/// Actions that can be used within the config file
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OrganizeAction {
    /// Do nothing.
    None,
    /// Confirm before deleting a duplicate.
    ///
    /// Supports placeholders
    /// e.g. `confirm: "Delete {name}?"`
    /// `{name}` would be the filename
    ///
    /// # Example
    ///
    /// Confirm before deleting a duplicate
    ///
    /// ```yaml
    /// rules:
    /// - name: "Delete duplicates with confirmation"
    ///     locations:
    ///     - ~/Downloads
    ///     - ~/Documents
    ///     filters:
    ///     - not empty
    ///     - duplicate
    ///     - name
    ///     actions:
    ///     - confirm: "Delete {name}?"
    ///     - trash
    /// ```
    Confirm {
        text: Option<String>,
        vars: Option<Vec<String>>,
    },
    /// Copy a file or dir to a new location.
    ///
    /// If the specified path does not exist it will be created.
    ///
    /// # Example
    ///
    /// Use a placeholder to copy all .pdf files into a "PDF" folder
    /// and all .jpg files into a "JPG" folder. Existing files will
    /// be overwritten.
    ///
    /// ```yaml
    /// rules:
    ///  - locations: ~/Desktop
    ///    filters:
    ///      - extension:
    ///          - pdf
    ///          - jpg
    ///    actions:
    ///      - copy:
    ///          dest: "~/Desktop/{extension.upper()}/"
    ///          on_conflict: overwrite
    /// ```
    Copy {
        /// The destination where the file / dir should be copied
        /// to. If `destination` ends with a slash, it is assumed
        /// to be a target directory and the file / dir will be
        /// copied into `destination`and keep its name.
        destination: String,
        /// What should happen in case dest already exists.
        /// One of skip, overwrite, trash, rename_new and rename_existing.
        ///
        /// Defaults to rename_new.
        on_conflict: OnConflict,
        /// A template for renaming the file / dir in case of a conflict.
        ///
        /// Defaults to `{name}_{counter}{extension}`
        rename_template: RenameTemplate,
        /// An opener url of the filesystem you want to copy to.
        ///
        /// If this is not given, the local filesystem is used.
        filesystem: Option<String>,
    },
    /// Delete a file from disk.
    ///
    /// Deleted files have no recovery option!
    /// Using the `Trash` action is strongly advised for most use-cases!
    ///
    /// # Example
    ///
    /// ```yaml
    /// rules:
    /// - locations: "~/Downloads"
    ///     filters:
    ///     - lastmodified:
    ///         days: 365
    ///     - extension:
    ///         - png
    ///         - jpg
    ///     actions:
    ///     - delete
    /// ```
    Delete,
    /// Prints the given message.
    ///
    /// This can be useful to test your rules, especially in combination
    /// with placeholder variables.
    ///
    /// # Example
    ///
    /// Prints `Found old file` whenever a file is discovered that was
    /// modified longer than 365 days ago.
    ///
    /// ```yaml
    /// rules:
    ///  - name: "Find files older than a year"
    ///    locations: ~/Desktop
    ///    filters:
    ///      - lastmodified:
    ///          days: 365
    ///    actions:
    ///      - echo: "Found old file"
    /// ```
    Echo { message: String },
    /// Add macOS tags.
    ///
    /// The color can be specified in brackets after the tag name.
    ///
    /// # Example
    ///
    /// Specify tag colors
    ///
    /// ```yaml
    /// rules:
    ///  - locations: "~/Documents/Invoices"
    ///    filters:
    ///      - name:
    ///          startswith: "Invoice"
    ///      - extension: pdf
    ///    actions:
    ///      - macos_tags:
    ///          - Important (green)
    ///          - Invoice (purple)
    /// ```
    ///
    /// # Example
    ///
    /// Add a templated tag with color
    ///
    /// ```yaml
    /// rules:
    ///  - locations: "~/Documents/Invoices"
    ///    filters:
    ///      - created
    ///    actions:
    ///      - macos_tags:
    ///         - Year-{created.year} (red)
    /// ```
    #[cfg(target_os = "osx")]
    MacOsTags { tags: Vec<String> },
    /// Move a file to a new location.
    ///
    /// The file can also be renamed. If the specified path does
    /// not exist it will be created.
    ///
    /// If you only want to rename the file and keep the folder,
    /// it is easier to use the rename action.
    ///
    /// # Example
    ///
    /// Use a placeholder to move all .pdf files into a "PDF" folder
    /// and all .jpg files into a "JPG" folder.
    ///
    /// Existing files will be overwritten.
    ///
    /// ```yaml
    /// rules:
    ///   - locations: ~/Desktop
    ///     filters:
    ///       - extension:
    ///           - pdf
    ///           - jpg
    ///     actions:
    ///       - move:
    ///           dest: "~/Desktop/{extension.upper()}/"
    ///           on_conflict: "overwrite"
    /// ```
    Move {
        /// The destination where the file / dir should be moved
        /// to. If `destination` ends with a slash, it is assumed
        /// to be a target directory and the file / dir will be
        /// moved into `destination`and keep its name.
        destination: String,
        /// What should happen in case dest already exists.
        /// One of skip, overwrite, trash, rename_new and rename_existing.
        ///
        /// Defaults to rename_new.
        on_conflict: OnConflict,
        /// A template for renaming the file / dir in case of a conflict.
        ///
        /// Defaults to `{name}_{counter}{extension}`
        rename_template: RenameTemplate,
        /// An opener url of the filesystem you want to move to.
        ///
        /// If this is not given, the local filesystem is used.
        filesystem: Option<String>,
    },
    /// Renames a file.
    ///
    /// # Example
    ///
    /// Convert all .PDF file extensions to lowercase (.pdf)
    ///
    /// ```yaml
    /// rules:
    ///   - name: "Convert all .PDF file extensions to lowercase (.pdf)"
    ///     locations: "~/Desktop"
    ///     filters:
    ///       - name
    ///       - extension: PDF
    ///     actions:
    ///       - rename: "{name}.pdf"
    /// ```
    Rename {
        name: String,
        /// What should happen in case dest already exists.
        /// One of skip, overwrite, trash, rename_new and rename_existing.
        ///
        /// Defaults to rename_new.
        on_conflict: OnConflict,
        /// A template for renaming the file / dir in case of a conflict.
        ///
        /// Defaults to `{name}_{counter}{extension}`
        rename_template: RenameTemplate,
    },
    /// Create a symbolic link.
    Symlink {
        /// The symlink destination.
        ///
        /// If `destination` ends with a slash `/``, create the symlink
        /// in the given directory.
        ///
        /// Only the local filesystem is supported.
        // TODO: Can contain placeholders?
        destination: String,
    },
    /// Move a file or dir into the trash.
    ///
    /// # Example
    ///
    /// Move all JPGs and PNGs on the desktop which are older than one
    /// year into the trash
    ///
    /// ```yaml
    /// rules:
    ///   - name: Move all JPGs and PNGs on the desktop which are older than one year into the trash
    ///     locations: "~/Desktop"
    ///     filters:
    ///       - lastmodified:
    ///           years: 1
    ///           mode: older
    ///       - extension:
    ///           - png
    ///           - jpg
    ///     actions:
    ///       - trash
    /// ```
    Trash,
    /// Write text to a file.
    ///
    /// If the specified path does not exist it will be created.
    ///
    /// # Example
    ///
    /// This will create a file sizes.txt in the current working folder
    /// which contains the filesizes of everything in the ~/Downloads folder.
    ///
    /// ```yaml
    /// rules:
    ///   - name: "Record file sizes"
    ///     locations: ~/Downloads
    ///     filters:
    ///       - size
    ///     actions:
    ///       - write:
    ///           text: "{size.traditional} -- {relative_path}"
    ///           path: "./sizes.txt"
    ///           mode: "append"
    ///           clear_before_first_write: true
    /// ```
    Write {
        /// The text that should be written. Supports templates.
        text: String,
        /// The file `text` should be written into. Supports templates.
        ///
        // Defaults to `organize-out.txt`
        file: WriteFile,
        /// Can be either `append` (append text to the file), `prepend`
        /// (insert text as first line) or `overwrite` (overwrite content
        /// with text).
        ///
        /// Defaults to `append`.
        mode: WriteMode,
        /// Whether to append a newline to the given text.
        ///
        /// Defaults to `true`.
        newline: Option<bool>,
        /// Clears the file before first appending / prepending text to it.
        /// This happens only the first time write_file is run. If the rule
        /// filters don't match anything the file is left as it is.
        ///
        /// Defaults to `false`.
        clear_before_first_write: Option<bool>,
        /// An opener url of the filesystem the textfile is on.
        ///
        /// If this is not given, the local filesystem is used.
        filesystem: Option<String>,
    },
    #[cfg(feature = "research_organize")]
    Python,
    #[cfg(feature = "research_organize")]
    Shell,
}

impl OrganizeAction {
    /// Returns `true` if the organize action is [`None`].
    ///
    /// [`None`]: OrganizeAction::None
    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` if the organize action is [`Confirm`].
    ///
    /// [`Confirm`]: OrganizeAction::Confirm
    #[must_use]
    pub fn is_confirm(&self) -> bool {
        matches!(self, Self::Confirm { .. })
    }

    /// Returns `true` if the organize action is [`Copy`].
    ///
    /// [`Copy`]: OrganizeAction::Copy
    #[must_use]
    pub fn is_copy(&self) -> bool {
        matches!(self, Self::Copy { .. })
    }

    /// Returns `true` if the organize action is [`Delete`].
    ///
    /// [`Delete`]: OrganizeAction::Delete
    #[must_use]
    pub fn is_delete(&self) -> bool {
        matches!(self, Self::Delete)
    }

    /// Returns `true` if the organize action is [`Echo`].
    ///
    /// [`Echo`]: OrganizeAction::Echo
    #[must_use]
    pub fn is_echo(&self) -> bool {
        matches!(self, Self::Echo { .. })
    }

    pub fn as_echo(&self) -> Option<&String> {
        if let Self::Echo { message } = self {
            Some(message)
        } else {
            None
        }
    }

    pub fn try_into_echo(self) -> Result<String, Self> {
        if let Self::Echo { message } = self {
            Ok(message)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the organize action is [`MacOsTags`].
    ///
    /// [`MacOsTags`]: OrganizeAction::MacOsTags
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

    /// Returns `true` if the organize action is [`Move`].
    ///
    /// [`Move`]: OrganizeAction::Move
    #[must_use]
    pub fn is_move(&self) -> bool {
        matches!(self, Self::Move { .. })
    }

    /// Returns `true` if the organize action is [`Rename`].
    ///
    /// [`Rename`]: OrganizeAction::Rename
    #[must_use]
    pub fn is_rename(&self) -> bool {
        matches!(self, Self::Rename { .. })
    }

    /// Returns `true` if the organize action is [`Symlink`].
    ///
    /// [`Symlink`]: OrganizeAction::Symlink
    #[must_use]
    pub fn is_symlink(&self) -> bool {
        matches!(self, Self::Symlink { .. })
    }

    pub fn as_symlink(&self) -> Option<&String> {
        if let Self::Symlink { destination } = self {
            Some(destination)
        } else {
            None
        }
    }

    pub fn try_into_symlink(self) -> Result<String, Self> {
        if let Self::Symlink { destination } = self {
            Ok(destination)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the organize action is [`Trash`].
    ///
    /// [`Trash`]: OrganizeAction::Trash
    #[must_use]
    pub fn is_trash(&self) -> bool {
        matches!(self, Self::Trash)
    }

    /// Returns `true` if the organize action is [`Write`].
    ///
    /// [`Write`]: OrganizeAction::Write
    #[must_use]
    pub fn is_write(&self) -> bool {
        matches!(self, Self::Write { .. })
    }
}

impl Default for OrganizeAction {
    fn default() -> Self {
        Self::None
    }
}
