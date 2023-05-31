//! organize-rs config

use crate::{
    error::{ConfigErrorKind, OrganizeResult},
    rules::{Rule, Rules},
};
use std::io::Write;
use std::{fmt::Display, fs::File, panic, path::Path, str::FromStr, string::ParseError};

#[cfg(feature = "cli")]
use clap::ValueEnum;

use displaydoc::Display;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use winnow::combinator::todo;

/// Formats that we support for our Config files
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Display)]
// TODO: do we want to chose explicitly or derive it from extension?
// #[cfg_attr(feature = "cli", derive(ValueEnum))]
pub enum ConfigFileFormat {
    /// json
    Json,
    /// toml
    Toml,
    /// yaml
    Yaml,
    /// ron
    Ron,
}

impl FromStr for ConfigFileFormat {
    type Err = crate::error::ConfigErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".json" | "json" => Ok(Self::Json),
            ".yaml" | ".yml" | "yaml" | "yml" => Ok(Self::Yaml),
            ".ron" | "ron" => Ok(Self::Ron),
            ".toml" | "toml" => Ok(Self::Toml),
            _ => Err(crate::error::ConfigErrorKind::ConfigFileFormatNotSupported(
                s.to_string(),
            )),
        }
    }
}

impl Default for ConfigFileFormat {
    fn default() -> Self {
        Self::Ron
    }
}

/// Organize Configuration
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
// #[serde(deny_unknown_fields)]
#[serde(default)]
pub struct OrganizeConfig {
    // aliases: Vec<Reference>,
    rules: Rules,
}

impl Display for OrganizeConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rule in self.rules.iter() {
            write!(f, "{rule}")?;
        }
        Ok(())
    }
}

impl OrganizeConfig {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> Self {
        let file = std::fs::File::open(path.as_ref()).expect("opening file shouldn't fail");

        let ext = <ConfigFileFormat as std::str::FromStr>::from_str(
            path.as_ref()
                .extension()
                .expect("getting file extension shouldn't fail")
                .to_str()
                .expect("conversion from OsStr to String shouldn't fail"),
        )
        .unwrap();

        match ext {
            ConfigFileFormat::Ron => {
                todo!("not fully supported (yet)");
                // TODO: ron deser support
                // ron::de::from_reader(file).expect("config file parsing shouldn't fail")
            }
            ConfigFileFormat::Yaml => {
                serde_yaml::from_reader(file).expect("config file parsing shouldn't fail")
            }
            ConfigFileFormat::Json => {
                serde_json::from_reader(file).expect("config file parsing shouldn't fail")
            }
            ConfigFileFormat::Toml => {
                todo!("not fully supported (yet)");
                // TODO: toml deser support
                // toml::from_str(
                //     std::fs::read_to_string(path)
                //         .expect("reading config file to string shouldn't fail")
                //         .as_str(),
                // )
                // .expect("config file parsing shouldn't fail")
            }
        }
    }

    pub fn write_to_file(&self, path: impl AsRef<Path>, overwrite: bool) -> OrganizeResult<()> {
        let file = match (File::open(path.as_ref()).is_ok(), overwrite) {
            (true, true) | (false, true) | (false, false) => File::create(path.as_ref())?,
            (true, false) => {
                return Err(crate::error::ConfigErrorKind::ConfigFileAlreadyExists(
                    path.as_ref().to_path_buf(),
                )
                .into())
            }
        };

        let ext = <ConfigFileFormat as std::str::FromStr>::from_str(
            path.as_ref()
                .extension()
                .expect("getting file extension shouldn't fail")
                .to_str()
                .expect("conversion from OsStr to String shouldn't fail"),
        )
        .unwrap();

        match ext {
            ConfigFileFormat::Ron => {
                ron::ser::to_writer_pretty(file, &self, PrettyConfig::default())
                    .map_err(ConfigErrorKind::RonError)?
            }
            ConfigFileFormat::Yaml => {
                serde_yaml::to_writer(file, self).map_err(ConfigErrorKind::YamlError)?
            }
            ConfigFileFormat::Json => {
                serde_json::to_writer_pretty(file, self).map_err(ConfigErrorKind::JsonError)?
            }
            ConfigFileFormat::Toml => {
                todo!("not fully supported (yet)");
                // TODO: toml ser support
                // write!(
                //     file,
                //     "{}",
                //     toml::to_string_pretty(self).map_err(ConfigErrorKind::TomlSerializeError)?
                // )?
            }
        }

        Ok(())
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule)
    }

    pub fn add_rules(&mut self, rules: Rules) {
        self.rules.extend_from_slice(&rules)
    }

    pub fn rules(&self) -> &Rules {
        &self.rules
    }
}
