//! Actions that can be used in the config file and
//! `organize` applieds to matching rules

pub mod conflicts;
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

use crate::{
    actions::conflicts::{ConflictKind, ConflictResolutionKind},
    error::OrganizeError, templating::TemplateStringKind,
};

type ActionClosure<'a, C> =
    Box<dyn FnMut(&DirEntry<C>, bool) -> Result<ActionResultKind, OrganizeError> + 'a>;

/// A preview for an action to be executed
#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum ActionResultKind {
    /// A preview of a to be executed action
    Preview {
        /// message to be printed
        msg: String,
        /// file or directory path the action should be executed on
        path: PathBuf,
        /// corresponding action
        action: ActionKind,
    },
    /// action has been successful
    Successful,
    /// conflict
    Conflicted(ConflictKind),
}

#[derive(Debug, Clone, Deserialize, Serialize, Display, Default)]
#[serde(transparent)]
pub struct ActionApplicationCollection(Vec<ActionContainer>);

impl std::ops::DerefMut for ActionApplicationCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Deref for ActionApplicationCollection {
    type Target = Vec<ActionContainer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// how an action is applied
#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub enum ActionApplicationKind {
    /// Dry run, as in preview an action
    #[serde(rename = "preview")]
    Preview,
    /// Execute action destructively
    #[serde(rename = "destructive")]
    Destructive,
    /// Wait for user input
    #[serde(rename = "input")]
    UserInput,
}

impl ActionApplicationKind {
    /// Returns `true` if the action application kind is [`Preview`].
    ///
    /// [`Preview`]: ActionApplicationKind::Preview
    #[must_use]
    pub fn is_preview(&self) -> bool {
        matches!(self, Self::Preview)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Display)]
pub struct ActionContainer {
    pub mode: ActionApplicationKind,
    pub action: ActionKind,
}

impl Default for ActionApplicationKind {
    fn default() -> Self {
        Self::Preview
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


/// Mode how should be written to a file
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WriteModeKind {
    /// append text to the file
    #[serde(rename = "append")]
    Append,
    /// insert text as first line
    #[serde(rename = "prepend")]
    Prepend,
    /// overwrite content with text
    #[serde(rename = "overwrite")]
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

// TODO: Shell support
// adapted from: https://organize.readthedocs.io/en/latest/actions/
//
/// Contains action variants that organize can
/// execute on matched [`jwalk::DirEntry`]
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
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Delete duplicates with confirmation
    ///      enabled: true
    ///      locations:
    ///         - !default_settings ~/Downloads
    ///         - !default_settings ~/Documents
    ///      filter_groups:
    ///        - filters:
    ///            - !empty
    ///          results: exclude
    ///          match: all
    ///        - filters:
    ///            - !duplicate
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: input
    ///          action: !confirm
    ///            msg: "Delete {{entry}}?"
    ///        - mode: destructive
    ///          action: !trash
    ///      tags:
    ///        - !custom Test::Action::Confirm
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
    #[serde(rename = "confirm")]
    Confirm {
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "Option::default")]
        msg: Option<String>,
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "Option::default")]
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
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Copy pdfs into PDF folder
    ///      enabled: true
    ///      locations:
    ///         - !default_settings ~/Desktop
    ///      filter_groups:
    ///        - filters:
    ///            - !extension
    ///              exts:
    ///                - pdf
    ///                - jpg
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: destructive
    ///          action: !copy
    ///            dst: ~/Desktop/{{uppercase(extension)}}
    ///            on_conflict: overwrite
    ///      tags:
    ///        - !custom Test::Action::Copy
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
    #[serde(rename = "copy")]
    Copy {
        /// The destination where the file / dir should be copied
        /// to. If `dst` ends with a slash, it is assumed
        /// to be a target directory and the file / dir will be
        /// copied into `dst` and keep its name.
        #[cfg_attr(feature = "cli", arg(long))]
        dst: PathBuf,
        /// What should happen in case dest already exists.
        /// One of skip, overwrite, trash, rename_new and rename_existing.
        ///
        /// Defaults to rename_new.
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "ConflictResolutionKind::default")]
        on_conflict: ConflictResolutionKind,
        /// A template for renaming the file / dir in case of a conflict.
        ///
        /// Defaults to `{name}_{counter}{extension}`
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "Option::default")]
        rename_template: Option<Vec<TemplateStringKind>>,
        /// An opener url of the filesystem you want to copy to.
        ///
        /// If this is not given, the local filesystem is used.
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "Option::default")]
        filesystem: Option<String>,
    },
    /// Delete a file from disk
    ///
    /// Deleted files have no recovery option!
    /// Using the `Trash` action is strongly advised for most use-cases!
    ///
    /// # Example
    ///
    /// Delete *.png and *.jpg inside the downloads folder that haven't been
    /// modified in the last 365 days
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Delete png and jpg with confirmation
    ///      enabled: true
    ///      locations:
    ///         - !default_settings ~/Downloads
    ///      filter_groups:
    ///        - filters:
    ///            - !extension
    ///              exts:
    ///                - png
    ///                - jpg
    ///            - !last_modified
    ///              range: 365d..
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: input
    ///          action: !confirm
    ///            msg: "Delete {{entry}}?"
    ///        - mode: destructive
    ///          action: !delete
    ///      tags:
    ///        - !custom Test::Action::Delete
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
    /// Prints `Found old file: {filename}` whenever a file is discovered that was
    /// modified longer than 365 days ago.
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Find files older than a year
    ///      enabled: true
    ///      locations:
    ///         - !default_settings ~/Desktop
    ///      filter_groups:
    ///        - filters:
    ///            - !last_modified
    ///              range: 365d..
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !echo
    ///            msg: "Found old file: {{entry}}"
    ///      tags:
    ///        - !custom Test::Action::Echo
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Specify tag colors
    ///      enabled: true
    ///      locations:
    ///         - !default_settings ~/Documents/Invoices
    ///      filter_groups:
    ///        - filters:
    ///            - !name
    ///              starts_with:
    ///                - Invoice
    ///            - !extension
    ///              exts:
    ///                - pdf
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: destructive
    ///          action: !macos_tags
    ///            tags:
    ///              - Important (green)
    ///              - Invoice (purple)
    ///      tags:
    ///        - !custom Test::Action::MacOsTags
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
    ///
    /// # Example
    ///
    /// Add a templated tag with color
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Add a templated tag with color
    ///      enabled: true
    ///      locations:
    ///         - !default_settings ~/Documents/Invoices
    ///      filter_groups:
    ///        - filters:
    ///            - !created
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: destructive
    ///          action: !macos_tags
    ///            tags:
    ///              - Year-{created.year} (red)
    ///      tags:
    ///        - !custom Test::Action::MacOsTagsTemplated
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Move pdfs into PDF folder
    ///      enabled: true
    ///      locations:
    ///         - !default_settings ~/Desktop
    ///      filter_groups:
    ///        - filters:
    ///            - !extension
    ///              exts:
    ///                - pdf
    ///                - jpg
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: input
    ///          action: !confirm
    ///            msg: "Move {{entry}}?"
    ///        - mode: destructive
    ///          action: !move
    ///            dst: ~/Desktop/{{uppercase(extension)}}/
    ///            on_conflict: overwrite
    ///      tags:
    ///        - !custom Test::Action::Move
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
    #[serde(rename = "move")]
    Move {
        /// The destination where the file / dir should be moved
        /// to. If `dst` ends with a slash, it is assumed
        /// to be a target directory and the file / dir will be
        /// moved into `dst`and keep its name.
        #[cfg_attr(feature = "cli", arg(long))]
        dst: PathBuf,
        /// What should happen in case dest already exists.
        /// One of skip, overwrite, trash, rename_new and rename_existing.
        ///
        /// Defaults to rename_new.
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "ConflictResolutionKind::default")]
        on_conflict: ConflictResolutionKind,
        /// A template for renaming the file / dir in case of a conflict.
        ///
        /// Defaults to `{name}_{counter}{extension}`
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "Option::default")]
        rename_template: Option<Vec<TemplateStringKind>>,
        /// An opener url of the filesystem you want to move to.
        ///
        /// If this is not given, the local filesystem is used.
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "Option::default")]
        filesystem: Option<String>,
    },
    /// Do nothing
    #[serde(rename = "do_nothing")]
    NoAction,
    /// Rename a file
    ///
    /// # Example
    ///
    /// Convert all .PDF file extensions to lowercase (.pdf)
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Convert all .PDF file extensions to lowercase (.pdf)
    ///      enabled: true
    ///      locations:
    ///         - !default_settings ~/Desktop
    ///      filter_groups:
    ///        - filters:
    ///            - !extension
    ///              exts:
    ///                - pdf
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: destructive
    ///          action: !rename
    ///            name: "{{lowercase(entry.name)}}"
    ///            on_conflict: skip
    ///      tags:
    ///        - !custom Test::Action::Rename
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
        #[serde(default = "ConflictResolutionKind::default")]
        on_conflict: ConflictResolutionKind,
        /// A template for renaming the file / dir in case of a conflict.
        ///
        /// Defaults to `{name}_{counter}{extension}`
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "Option::default")]
        rename_template: Option<Vec<TemplateStringKind>>,
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
    /// On macOS: Open all pdfs on your desktop
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Open all pdfs on your desktop
    ///      enabled: true
    ///      locations:
    ///         - !default_settings ~/Desktop
    ///      filter_groups:
    ///        - filters:
    ///            - !extension
    ///              exts:
    ///                - pdf
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: destructive
    ///          action: !shell
    ///            command: "open '{{entry}}'"
    ///      tags:
    ///        - !custom Test::Action::Shell
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
    /// ```
    #[serde(rename = "shell")]
    Shell {
        /// The command to execute.
        #[cfg_attr(feature = "cli", arg(long))]
        command: String,
        /// Whether to execute in simulation mode
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "bool::default")]
        run_in_simulation: bool,
        /// Whether to continue on erros (return codes !=0)
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "bool::default")]
        ignore_errors: bool,
        /// The value of `{shell.output}` if run in simulation
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "Option::default")]
        simulation_output: Option<String>,
        /// The value of `{shell.returncode}` if run in simulation
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "u64::default")]
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
        dst: PathBuf,
    },
    /// Move a file or directory into the trash
    ///
    /// # Example
    ///
    /// Move all JPGs and PNGs on the desktop which are older than one
    /// year into the trash
    ///
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Move all JPGs and PNGs on the desktop which are older than one year into the trash
    ///      enabled: true
    ///      locations:
    ///         - !non_recursive
    ///           path: ~/Desktop
    ///           target: files
    ///      filter_groups:
    ///        - filters:
    ///            - !last_modified
    ///              range: 1y..
    ///            - !extension
    ///              exts:
    ///                - png
    ///                - jpg
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: destructive
    ///          action: !trash
    ///      tags:
    ///        - !custom Test::Action::Trash
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
    /// ```rust
    /// # use organize_rs_core::config::{OrganizeConfig, ConfigFileFormat};
    /// # let rule = r#"
    /// rules:
    ///    - name: Record file sizes
    ///      enabled: true
    ///      locations:
    ///         - !recursive
    ///           path: ~/Downloads
    ///           max_depth: 10
    ///           target: both
    ///      filter_groups:
    ///        - filters:
    ///            - !size
    ///          results: include
    ///          match: all
    ///      actions:
    ///        - mode: preview
    ///          action: !write
    ///            txt: "{{size.traditional}} -- {{relative_path}}"
    ///            file: ./sizes.txt
    ///            mode: append
    ///            clear_before_first_write: true
    ///      tags:
    ///        - !custom Test::Action::Write
    /// # "#;
    /// # let config = OrganizeConfig::load_from_string(rule, ConfigFileFormat::Yaml);
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
        file: PathBuf,
        /// Can be either `append` (append text to the file), `prepend`
        /// (insert text as first line) or `overwrite` (overwrite content
        /// with text).
        ///
        /// Defaults to `append`.
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "WriteModeKind::default")]
        mode: WriteModeKind,
        /// Whether to append a newline to the given text.
        ///
        /// Defaults to `false`.
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "bool::default")]
        newline: bool,
        /// Clears the file before first appending / prepending text to it.
        /// This happens only the first time write_file is run. If the rule
        /// filters don't match anything the file is left as it is.
        ///
        /// Defaults to `false`.
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "bool::default")]
        clear_before_first_write: bool,
        /// An opener url of the filesystem the textfile is on.
        ///
        /// If this is not given, the local filesystem is used.
        #[cfg_attr(feature = "cli", arg(long))]
        #[serde(default = "Option::default")]
        filesystem: Option<PathBuf>,
    },
}
