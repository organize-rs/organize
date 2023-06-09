use std::str::FromStr;

use winnow::error::Error;

use crate::parsers::template::{
    parse_dotted_template, parse_strftime_template, parse_transform_template,
};

type FunctionClosure = Box<dyn FnOnce(String) -> String>;
type FormatString = String;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformationKind {
    /// uppercase
    UpperCase,
    /// lowercase
    LowerCase,
    /// camelcase
    CamelCase,
    /// snakecase
    SnakeCase,
    /// kebabcase
    KebabCase,
    /// strftime
    DateTime,
}

impl FromStr for TransformationKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "uppercase" => Self::UpperCase,
            "lowercase" => Self::LowerCase,
            "camelcase" => Self::CamelCase,
            "snakecase" => Self::SnakeCase,
            "kebabcase" => Self::KebabCase,
            "strftime" => Self::DateTime,
            _ => return Err("FunctionKind not recognized.".to_string()),
        })
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaDataKind {
    DateAdded(DateAttributeArgKind),
    Created(DateAttributeArgKind),
    LastModified(DateAttributeArgKind),
    Name,
    Extension,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateAttributeArgKind {
    Year,
    Month,
    Day,
    NotSet,
}

impl Default for DateAttributeArgKind {
    fn default() -> Self {
        Self::NotSet
    }
}

impl FromStr for DateAttributeArgKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "year" => Self::Year,
            "month" => Self::Month,
            "day" => Self::Day,
            _ => Self::default(),
        };

        Ok(result)
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UtilityKind {
    Counter,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateFeatureKind {
    Utility(UtilityKind),
    MetaData(MetaDataKind),
    Content(String),
    NotRecognized,
}

impl From<&[&str]> for TemplateFeatureKind {
    fn from(value: &[&str]) -> Self {
        match value {
            ["utility", "counter"] => Self::Utility(UtilityKind::Counter),
            ["metadata", "extension"] => Self::MetaData(MetaDataKind::Extension),
            ["metadata", "date_added"] => {
                Self::MetaData(MetaDataKind::DateAdded(DateAttributeArgKind::NotSet))
            }
            ["metadata", "date_added", date] => Self::MetaData(MetaDataKind::DateAdded(
                DateAttributeArgKind::from_str(date).expect("conversion to date shouldn't fail"),
            )),
            ["metadata", "created"] => {
                Self::MetaData(MetaDataKind::Created(DateAttributeArgKind::NotSet))
            }
            ["metadata", "created", date] => Self::MetaData(MetaDataKind::Created(
                DateAttributeArgKind::from_str(date).expect("conversion to date shouldn't fail"),
            )),
            ["metadata", "last_modified"] => {
                Self::MetaData(MetaDataKind::LastModified(DateAttributeArgKind::NotSet))
            }
            ["metadata", "last_modified", date] => Self::MetaData(MetaDataKind::LastModified(
                DateAttributeArgKind::from_str(date).expect("conversion to date shouldn't fail"),
            )),
            ["metadata", "name"] => Self::MetaData(MetaDataKind::Name),
            ["content", last] => Self::Content(last.to_string()),
            _ => Self::NotRecognized,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateKind {
    Dotted {
        data: TemplateFeatureKind,
    },
    FormattedTransformation {
        kind: TransformationKind,
        data: TemplateFeatureKind,
        format: FormatString,
    },
    Transformation {
        kind: TransformationKind,
        data: TemplateFeatureKind,
    },
    Uninitialized
}

impl TemplateKind {
    pub fn get_template(&self) -> String {
        match self {
            TemplateKind::Dotted { data: _ } => todo!(),
            TemplateKind::FormattedTransformation {
                kind: _,
                data: _,
                format: _,
            } => todo!(),
            TemplateKind::Transformation { kind: _, data: _ } => todo!(),
            TemplateKind::Uninitialized => todo!(),
        }
    }
}

impl FromStr for TemplateKind {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let result = match (
            parse_transform_template(input),
            parse_strftime_template(input),
            parse_dotted_template(input),
        ) {
            (Ok((_, (kind, items))), _, _) => Self::Transformation {
                kind: TransformationKind::from_str(kind).map_err(|err| Error {
                    input: err,
                    kind: winnow::error::ErrorKind::Fail,
                })?,
                data: TemplateFeatureKind::from(items.as_slice()),
            },
            (_, Ok((_, (kind, items, format))), _) => Self::FormattedTransformation {
                kind: TransformationKind::from_str(kind).map_err(|err| Error {
                    input: err,
                    kind: winnow::error::ErrorKind::Fail,
                })?,
                data: TemplateFeatureKind::from(items.as_slice()),
                format: format.to_string(),
            },
            (_, _, Ok((_, path))) => Self::Dotted {
                data: TemplateFeatureKind::from(path.as_slice()),
            },
            _ => {
                return Err(Error {
                    input: format!("Not a valid template: {input}"),
                    kind: winnow::error::ErrorKind::Verify,
                })
            }
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        borrow::{Borrow},
        ffi::OsStr,
        path::{PathBuf},
        str::FromStr,
    };

    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_extraction_of_template_passes() {
        let extension = "pdf".to_string();
        let example =
            PathBuf::from(r#"C:\Users\dailyuse\Desktop\{{uppercase(metadata.extension)}}\"#);

        let mut item_position = 0;
        let mut template_kind = TemplateKind::Uninitialized;

        let mut editable = example
            .iter()
            .enumerate()
            .map(|(idx, component)| {
                let Ok(template) = TemplateKind::from_str(component.to_string_lossy().borrow()) else {
                        return component
                    };
                
                assert_eq!(
                    template,
                    TemplateKind::Transformation {
                        kind: TransformationKind::UpperCase,
                        data: TemplateFeatureKind::MetaData(MetaDataKind::Extension),
                    }
                );

                template_kind = template;
                item_position = idx;

                component
            })
            .collect_vec();

        let uppercase = extension.to_uppercase();
        editable[item_position] = OsStr::new(&uppercase);
        let path = editable.iter().collect::<PathBuf>();

        assert_eq!(path, PathBuf::from(r#"C:\Users\dailyuse\Desktop\PDF"#))
    }

    // #[test]
    // fn test_template_on_path_end_passes() {
    //     let entry = PathBuf::from(r#"C:\Users\dailyuse\Desktop\test.pdf"#);
    //     let example = PathBuf::from(r#"C:\Users\dailyuse\Desktop\{{uppercase(entry.extension)}}\"#);

    //     let extension = entry.extension().unwrap();
    //     let extension = extension.to_ascii_uppercase();

    //     let mut out_path = example;
    //     out_path.pop();
    //     out_path.push(extension);

    //     assert_eq!(out_path, PathBuf::from(r#"C:\Users\dailyuse\Desktop\PDF\"#))
    // }

    #[test]
    fn test_parsing_uppercase_extension_templatekind_passes() {
        let template = "{{uppercase(metadata.extension)}}";

        let template_kind = TemplateKind::from_str(template).unwrap();

        assert_eq!(
            template_kind,
            TemplateKind::Transformation {
                kind: TransformationKind::UpperCase,
                data: TemplateFeatureKind::MetaData(MetaDataKind::Extension),
            }
        );
    }

    #[test]
    fn test_parsing_counter_templatekind_passes() {
        let template = "{{utility.counter}}";

        let template_kind = TemplateKind::from_str(template).unwrap();

        assert_eq!(
            template_kind,
            TemplateKind::Dotted {
                data: TemplateFeatureKind::Utility(UtilityKind::Counter)
            }
        );
    }

    #[test]
    fn test_parsing_content_templatekind_passes() {
        let template = "{{content.customer}}";

        let template_kind = TemplateKind::from_str(template).unwrap();

        assert_eq!(
            template_kind,
            TemplateKind::Dotted {
                data: TemplateFeatureKind::Content("customer".to_string())
            }
        );
    }

    #[test]
    fn test_parsing_metadata_templatekind_passes() {
        let template = "{{metadata.last_modified.year}}";

        let template_kind = TemplateKind::from_str(template).unwrap();

        assert_eq!(
            template_kind,
            TemplateKind::Dotted {
                data: TemplateFeatureKind::MetaData(MetaDataKind::LastModified(
                    DateAttributeArgKind::Year
                ))
            }
        );
    }

    #[test]
    fn test_parsing_strftime_templatekind_passes() {
        let template = "{{strftime(metadata.date_added, '%Y-%m-%d')}}";

        let template_kind = TemplateKind::from_str(template).unwrap();

        assert_eq!(
            template_kind,
            TemplateKind::FormattedTransformation {
                kind: TransformationKind::DateTime,
                data: TemplateFeatureKind::MetaData(MetaDataKind::DateAdded(
                    DateAttributeArgKind::NotSet
                )),
                format: FormatString::from("%Y-%m-%d")
            }
        );
    }
}
