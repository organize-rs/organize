#[cfg(feature = "cli")]
use clap::Args;
use displaydoc::Display;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    error::FilterErrorKind,
    filters::{Filter, FilterOperationKind},
};

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct NameArgs {
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
}

/// Arguments for `name` filter
#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Display, Debug, Clone, Deserialize, Serialize, Default)]
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

impl Filter for NameArgs {
    fn apply(&self, entry: &jwalk::DirEntry<((), ())>) -> bool {
        let NameFilterArgs {
            simple_match: _, // TODO:  implement simple match filter
            starts_with,
            contains,
            ends_with,
        } = self.arguments;

        let make_lowercase_if = |string: String| {
            if self.case_insensitive {
                string.to_lowercase()
            } else {
                string
            }
        };

        // Extract the file stem, if we can't do that, it's senseless to continue
        let Some(Some(file_stem)) = entry
                .path()
                .file_stem()
                .map(|f|{
                    f.to_str().map(|f| f.to_owned())}) else {
                        return false
                    };

        let file_stem = make_lowercase_if(file_stem);

        let to_filter_applikation_kind = |string: String| -> FilterOperationKind<String> {
            if !string.starts_with(crate::filters::constants::NEGATE_STRING) {
                FilterOperationKind::Apply(string)
            } else {
                FilterOperationKind::Invert({
                    string
                        .strip_prefix(crate::filters::constants::NEGATE_STRING)
                        .map_or_else(|| string.clone(), |f| f.to_owned())
                })
            }
        };

        let contains_filter = |wrapped_string: FilterOperationKind<String>| match wrapped_string {
            FilterOperationKind::Invert(invert) => match file_stem.contains(&invert) {
                true => Err(FilterErrorKind::InvertedItem(invert)),
                false => Ok(false),
            },
            FilterOperationKind::Apply(apply) => Ok(file_stem.contains(&apply)),
        };

        let starts_with_filter = |wrapped_string: FilterOperationKind<String>| match wrapped_string
        {
            FilterOperationKind::Invert(invert) => match file_stem.starts_with(&invert) {
                true => Err(FilterErrorKind::InvertedItem(invert)),
                false => Ok(false),
            },
            FilterOperationKind::Apply(apply) => Ok(file_stem.starts_with(&apply)),
        };

        let ends_with_filter = |wrapped_string: FilterOperationKind<String>| match wrapped_string {
            FilterOperationKind::Invert(invert) => match file_stem.ends_with(&invert) {
                true => Err(FilterErrorKind::InvertedItem(invert)),
                false => Ok(false),
            },
            FilterOperationKind::Apply(apply) => Ok(file_stem.ends_with(&apply)),
        };

        let (contains_oks, contains_errs): (Vec<_>, Vec<_>) = if let Some(contains) = contains {
            contains
                .into_iter()
                .map(make_lowercase_if)
                .map(to_filter_applikation_kind)
                .map(contains_filter)
                .partition_result()
        } else {
            (vec![], vec![])
        };

        let (starts_with_oks, starts_with_errs): (Vec<_>, Vec<_>) =
            if let Some(starts_with) = starts_with {
                starts_with
                    .into_iter()
                    .map(make_lowercase_if)
                    .map(to_filter_applikation_kind)
                    .map(starts_with_filter)
                    .partition_result()
            } else {
                (vec![], vec![])
            };

        let (ends_with_oks, ends_with_errs): (Vec<_>, Vec<_>) = if let Some(ends_with) = ends_with {
            ends_with
                .into_iter()
                .map(make_lowercase_if)
                .map(to_filter_applikation_kind)
                .map(ends_with_filter)
                .partition_result()
        } else {
            (vec![], vec![])
        };

        // return early if we have an item that should be skipped due to being inverted
        if !(ends_with_errs.is_empty() & starts_with_errs.is_empty() & contains_errs.is_empty()) {
            return false;
        };

        let mut oks = contains_oks;
        oks.extend(starts_with_oks);
        oks.extend(ends_with_oks);

        oks.into_iter().any(|f| f)
    }
}
