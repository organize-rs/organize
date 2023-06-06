use displaydoc::Display;
// use filetime::FileTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "cli")]
use clap::ValueEnum;

// use winnow::{
//     ascii::alpha0,
//     combinator::alt,
//     error::{Error, ErrorKind},
//     token::take_while,
//     IResult, Parser,
// };

// // Display
// #[derive(Debug, Clone)]
// pub enum TemplateActionKind {
//     UpperCase(String),
//     LowerCase(String),
//     CamelCase(String),
//     SnakeCase(String),
//     KebabCase(String),
//     Counter(usize),
//     Date(FileTime),
// }

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
    /// {{date}}
    Date,
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

// // pub fn parse_left_boundary(input: &str) -> IResult<&str, (f64, &str, &str)> {
// //     (parse_digits, parse_units, parse_boundary).parse_next(input)
// // }

// pub fn parse_units(input: &str) -> IResult<&str, &str> {
//     alpha0(input)
// }
// #[cfg(test)]
// mod tests {
//     use std::path::PathBuf;

//     #[test]
//     fn test_unpacking_multiple_template_strings_passes() {
//         let extension = "toml";
//         let example = PathBuf::from(r#"C:\Users\dailyuse\Desktop\{{uppercase(extension)}}\"#);
//         let parts = example.components();
//         parts.into_iter().map(|component| {
//             component.as_os_str().to_str().map(|string| {
//                 let left = string.find("{{");
//                 let right = string.rfind("}}");

//                 let (Some(left), Some(right)) = (left, right) else {
//                     return string
//                 };

//                 // we do `left+2..right` here, because we actually extract
//                 // the outer most template directly
//                 let template = &string[left + 2..right];

//                 assert_eq!(template, "uppercase(extension)");

//                 let left = template.find("{{");
//                 let right = template.rfind("}}");
//             })
//         })
//     }
// }
