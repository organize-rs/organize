//! Actions that can be used in the config file and
//! `organize` applieds to matching rules

mod conflicts;
mod impl_;
mod impl_traits;
#[cfg(test)]
mod tests;

use std::{fmt::Display, path::PathBuf};

#[cfg(feature = "cli")]
use clap::{Subcommand, ValueEnum};

use displaydoc::Display;
use jwalk::DirEntry;
use serde::{Deserialize, Serialize};

use crate::{error::OrganizeError, actions::conflicts::OnConflictKind};

type ActionClosure<'a, C> =
    Box<dyn FnMut(&DirEntry<C>, bool) -> Result<ActionResultKind, OrganizeError> + 'a>;

/// A preview for an action to be executed
#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum ActionResultKind {
    /// A preview of a to be executed action
    Preview {
        /// file or directory path the action should be executed on
        path: PathBuf,
        /// corresponding action
        action: ActionKind,
    },
    /// action has been successful
    Successful,
}

#[derive(Debug, Clone, Deserialize, Serialize, Display, Default)]
#[serde(transparent)]
pub struct ActionApplicationCollection(Vec<ActionApplicationKind>);

impl std::ops::DerefMut for ActionApplicationCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for ActionApplicationCollection {
    type Target = Vec<ActionApplicationKind>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// how an action is applied
#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum ActionApplicationKind {
    /// Dry run, as in preview an action
    Preview(ActionKind),
    /// Execute action destructively
    Destructive(ActionKind),
}

impl Default for ActionApplicationKind {
    fn default() -> Self {
        Self::Preview(ActionKind::default())
    }
}

/// Colours for `MacOS` tags
#[cfg(target_os = "osx")]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum MacOsTagColourKind {
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
impl MacOsTagColourKind {
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
impl Default for MacOsTagColourKind {
    fn default() -> Self {
        Self::None
    }
}

/// Support template strings
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Display)]
pub enum TemplateStringKind {
    /// {{filename}}
    Filename,
    /// {{counter}}
    Counter,
    /// {{extension}}
    Extension,
}

impl TemplateStringKind {
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

/// Mode how should be written to a file
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WriteModeKind {
    /// append text to the file
    Append,
    /// insert text as first line
    Prepend,
    /// overwrite content with text
    Overwrite,
}

impl Display for WriteModeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteModeKind::Append => write!(f, "Append"),
            WriteModeKind::Prepend => write!(f, "Prepend"),
            WriteModeKind::Overwrite => write!(f, "Overwrite"),
        }
    }
}

impl WriteModeKind {
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

impl Default for WriteModeKind {
    fn default() -> Self {
        Self::Append
    }
}

// TODO: Shell supported
// adapted from: https://organize.readthedocs.io/en/latest/actions/
//
/// Actions that can be used within the config file
#[cfg_attr(feature = "cli", derive(Subcommand))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ActionKind {
    /// Confirm before deleting a duplicate
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
    #[serde(rename = "confirm")]
    Confirm {
        #[cfg_attr(feature = "cli", arg(long))]
        text: Option<String>,
        #[cfg_attr(feature = "cli", arg(long))]
        vars: Option<Vec<String>>,
    },
    /// Copy a file or directory to a new location
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
    #[serde(rename = "copy")]
    Copy {
        /// The destination where the file / dir should be copied
        /// to. If `dst` ends with a slash, it is assumed
        /// to be a target directory and the file / dir will be
        /// copied into `dst` and keep its name.
        #[cfg_attr(feature = "cli", arg(long))]
        dst: String,
        /// What should happen in case dest already exists.
        /// One of skip, overwrite, trash, rename_new and rename_existing.
        ///
        /// Defaults to rename_new.
        #[cfg_attr(feature = "cli", arg(long))]
        on_conflict: OnConflictKind,
        /// A template for renaming the file / dir in case of a conflict.
        ///
        /// Defaults to `{name}_{counter}{extension}`
        #[cfg_attr(feature = "cli", arg(long))]
        rename_template: Vec<TemplateStringKind>,
        /// An opener url of the filesystem you want to copy to.
        ///
        /// If this is not given, the local filesystem is used.
        #[cfg_attr(feature = "cli", arg(long))]
        filesystem: Option<String>,
    },
    /// Delete a file from disk
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
    #[serde(rename = "delete")]
    Delete,
    /// Print a given message
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
    #[serde(rename = "echo")]
    Echo {
        #[cfg_attr(feature = "cli", arg(long))]
        msg: String,
    },
    /// Add macOS tags
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
    #[serde(rename = "macos_tags")]
    MacOsTags {
        #[cfg_attr(feature = "cli", arg(long))]
        tags: Vec<String>,
    },
    /// Move a file to a new location
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
    #[serde(rename = "move")]
    Move {
        /// The destination where the file / dir should be moved
        /// to. If `dst` ends with a slash, it is assumed
        /// to be a target directory and the file / dir will be
        /// moved into `dst`and keep its name.
        #[cfg_attr(feature = "cli", arg(long))]
        dst: String,
        /// What should happen in case dest already exists.
        /// One of skip, overwrite, trash, rename_new and rename_existing.
        ///
        /// Defaults to rename_new.
        #[cfg_attr(feature = "cli", arg(long))]
        on_conflict: OnConflictKind,
        /// A template for renaming the file / dir in case of a conflict.
        ///
        /// Defaults to `{name}_{counter}{extension}`
        #[cfg_attr(feature = "cli", arg(long))]
        rename_template: Vec<TemplateStringKind>,
        /// An opener url of the filesystem you want to move to.
        ///
        /// If this is not given, the local filesystem is used.
        #[cfg_attr(feature = "cli", arg(long))]
        filesystem: Option<String>,
    },
    /// Do nothing
    NoAction,
    /// Rename a file
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
    #[serde(rename = "rename")]
    Rename {
        /// The new name for the file / dir.
        #[cfg_attr(feature = "cli", arg(long))]
        name: String,
        /// What should happen in case dest already exists.
        /// One of skip, overwrite, trash, rename_new and rename_existing.
        ///
        /// Defaults to rename_new.
        #[cfg_attr(feature = "cli", arg(long))]
        on_conflict: OnConflictKind,
        /// A template for renaming the file / dir in case of a conflict.
        ///
        /// Defaults to `{name}_{counter}{extension}`
        #[cfg_attr(feature = "cli", arg(long))]
        rename_template: Vec<TemplateStringKind>,
    },
    /// Execute a shell command
    ///
    /// # Result
    ///
    /// `{shell.output}`: The stdout of the executed process.
    /// `{shell.returncode}`: The returncode of the executed process.
    ///
    /// # Example
    ///
    /// ```yaml
    /// rules:
    ///   - name: "On macOS: Open all pdfs on your desktop"
    ///     locations: "~/Desktop"
    ///     filters:
    ///       - extension: pdf
    ///     actions:
    ///       - shell: 'open "{path}"'
    /// ```
    #[serde(rename = "shell")]
    Shell {
        /// The command to execute.
        #[cfg_attr(feature = "cli", arg(long))]
        command: String,
        /// Whether to execute in simulation mode
        #[cfg_attr(feature = "cli", arg(long))]
        run_in_simulation: bool,
        /// Whether to continue on erros (return codes !=0)
        #[cfg_attr(feature = "cli", arg(long))]
        ignore_errors: bool,
        /// The value of `{shell.output}` if run in simulation
        #[cfg_attr(feature = "cli", arg(long))]
        simulation_output: String,
        /// The value of `{shell.returncode}` if run in simulation
        #[cfg_attr(feature = "cli", arg(long))]
        simulation_returncode: u64,
    },
    /// Create a symbolic link.
    #[serde(rename = "symlink")]
    Symlink {
        /// The symlink destination.
        ///
        /// If `dst` ends with a slash `/``, create the symlink
        /// in the given directory.
        ///
        /// Only the local filesystem is supported.
        // TODO: Can contain placeholders?
        #[cfg_attr(feature = "cli", arg(long))]
        dst: String,
    },
    /// Move a file or directory into the trash
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
    #[serde(rename = "trash")]
    Trash,
    /// Write text to a file
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
    #[serde(rename = "write")]
    Write {
        /// The text that should be written. Supports templates.
        #[cfg_attr(feature = "cli", arg(long))]
        txt: String,
        /// The file `text` should be written into. Supports templates.
        ///
        // Defaults to `organize-out.txt`
        #[cfg_attr(feature = "cli", arg(long))]
        file: String,
        /// Can be either `append` (append text to the file), `prepend`
        /// (insert text as first line) or `overwrite` (overwrite content
        /// with text).
        ///
        /// Defaults to `append`.
        #[cfg_attr(feature = "cli", arg(long))]
        mode: WriteModeKind,
        /// Whether to append a newline to the given text.
        ///
        /// Defaults to `true`.
        #[cfg_attr(feature = "cli", arg(long))]
        newline: Option<bool>,
        /// Clears the file before first appending / prepending text to it.
        /// This happens only the first time write_file is run. If the rule
        /// filters don't match anything the file is left as it is.
        ///
        /// Defaults to `false`.
        #[cfg_attr(feature = "cli", arg(long))]
        clear_before_first_write: Option<bool>,
        /// An opener url of the filesystem the textfile is on.
        ///
        /// If this is not given, the local filesystem is used.
        #[cfg_attr(feature = "cli", arg(long))]
        filesystem: Option<String>,
    },
}
