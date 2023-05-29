//! unit tests for filters

// use quickcheck_macros::quickcheck;

use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use rstest::*;

use itertools::Itertools;
use jwalk::{DirEntry, WalkDir};
use pretty_assertions::assert_eq;

use filetime::{self, FileTime};

use crate::{
    filters::{FilterKind, NameFilterArgs},
    parsers::{PeriodRange, SizeRange},
};

fn get_fixtures_dir() -> PathBuf {
    vec!["tests", "fixtures", "filters"].iter().collect()
}

fn get_fixture_entries(sub_dir: impl AsRef<Path>) -> Vec<DirEntry<((), ())>> {
    let mut to_walk = get_fixtures_dir();
    to_walk.push(sub_dir.as_ref());
    WalkDir::new(to_walk)
        .into_iter()
        .filter_map(|f| f.ok())
        .collect_vec()
}

fn get_base_values(root: impl AsRef<Path>, filter: FilterKind) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut entries = get_fixture_entries(root);
    let before = entries.iter().map(|f| f.path()).collect_vec();

    entries.retain(|f| (filter.get_filter()(f)));
    let after = entries.iter().map(|f| f.path()).collect_vec();

    (before, after)
}

#[rstest]
#[should_panic]
#[case(FileTime::now().seconds() + 1, "3w..12w")] // -1 second
#[should_panic]
#[case(FileTime::now().seconds() - 2 * 7 * 24 * 60 * 60, "3w..12w")] // 2 weeks
#[should_panic]
#[case(FileTime::now().seconds() - 8 * 24 * 60 * 60, "..7d")] // 8 days
#[should_panic]
#[case(FileTime::now().seconds() - 24 * 60 * 60, "3d..")] // 1 day
#[case(FileTime::now().seconds() - 5 * 7 * 24 * 60 * 60, "3w..12w")] // 5 weeks
#[case(FileTime::now().seconds() - 6 * 24 * 60 * 60, "..7d")] // 6 days
#[case(FileTime::now().seconds() - 4 * 24 * 60 * 60, "3d..")] // 4 days
fn test_matches_date_passes(#[case] time: i64, #[case] period: PeriodRange) {
    assert!(FilterKind::matches_date(time, &period))
}

#[rstest]
#[should_panic]
fn test_filter_mimetype_image_fails() {
    let filter = FilterKind::Mimetype {
        mimetype: vec![String::from("image")],
    };

    let (mut before, after) = get_base_values("mimetype", filter);

    before.remove(3);
    before.remove(1);
    before.remove(0);

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_mimetype_jpg_odt_passes() {
    let filter = FilterKind::Mimetype {
        mimetype: vec![
            String::from("application/vnd.oasis.opendocument.text"),
            String::from("image/jpeg"),
        ],
    };

    let (mut before, after) = get_base_values("mimetype", filter);

    before.remove(1);
    before.remove(0);

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_all_items_passes() {
    let filter = FilterKind::AllItems {
        i_agree_it_is_dangerous: true,
    };

    let (before, after) = get_base_values("by_name", filter);

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_no_filter_passes() {
    let filter = FilterKind::NoFilter;

    let (mut before, after) = get_base_values("by_name", filter);

    before.clear();

    assert_eq!(before, after);
}

#[rstest]
#[case(true)]
#[case(false)]
fn test_filter_name_full_passes(#[case] case_insensitive: bool) {
    let filter = FilterKind::Name {
        arguments: NameFilterArgs {
            contains: vec![String::from("TaSt"), String::from("-|uTEST")],
            starts_with: vec![String::from("123")],
            simple_match: vec![],
            ends_with: vec![String::from("2")],
        },
        case_insensitive,
    };

    let (mut before, after) = get_base_values("by_name", filter);

    before.remove(5);
    before.remove(4);
    before.remove(0);

    assert_eq!(before, after);
}

#[rstest]
#[case(true)]
#[case(false)]
fn test_filter_name_contains_multiple_names_and_inverted_passes(#[case] case_insensitive: bool) {
    let filter = FilterKind::Name {
        arguments: NameFilterArgs {
            contains: vec![
                String::from("test"),
                String::from("TaSt"),
                String::from("-|uTEST"),
            ],
            starts_with: vec![],
            simple_match: vec![],
            ends_with: vec![],
        },
        case_insensitive,
    };

    let (mut before, after) = get_base_values("by_name", filter);

    if case_insensitive {
        before.remove(5);
        before.remove(0);
    } else {
        before.remove(5);
        before.remove(4);
        before.remove(0);
    }

    assert_eq!(before, after);
}

#[rstest]
#[case(true)]
#[case(false)]
fn test_filter_name_contains_multiple_names_passes(#[case] case_insensitive: bool) {
    let filter = FilterKind::Name {
        arguments: NameFilterArgs {
            contains: vec![String::from("test"), String::from("TaSt")],
            starts_with: vec![],
            simple_match: vec![],
            ends_with: vec![],
        },
        case_insensitive,
    };

    let (mut before, after) = get_base_values("by_name", filter);

    if case_insensitive {
        before.remove(0);
    } else {
        before.remove(5);
        before.remove(4);
        before.remove(0);
    }

    assert_eq!(before, after);
}

#[rstest]
#[case(true)]
#[case(false)]
fn test_filter_name_contains_single_name_passes(#[case] case_insensitive: bool) {
    let filter = FilterKind::Name {
        arguments: NameFilterArgs {
            contains: vec![String::from("test")],
            starts_with: vec![],
            simple_match: vec![],
            ends_with: vec![],
        },
        case_insensitive,
    };

    let (mut before, after) = get_base_values("by_name", filter);

    if case_insensitive {
        before.remove(3);
        before.remove(0);
    } else {
        before.remove(5);
        before.remove(4);
        before.remove(3);
        before.remove(0);
    }

    assert_eq!(before, after);
}

#[rstest]
#[case(true)]
#[case(false)]
fn test_filter_name_args_does_not_match_anything_passes(#[case] case_insensitive: bool) {
    let filter = FilterKind::Name {
        arguments: NameFilterArgs {
            contains: vec![String::from("toast")],
            starts_with: vec![],
            simple_match: vec![],
            ends_with: vec![],
        },
        case_insensitive,
    };

    let (mut before, after) = get_base_values("by_name", filter);

    before.clear();

    assert_eq!(before, after);
}

#[rstest]
#[case(true)]
#[case(false)]
fn test_filter_name_args_empty_passes(#[case] case_insensitive: bool) {
    let filter = FilterKind::Name {
        arguments: NameFilterArgs {
            contains: vec![],
            starts_with: vec![],
            simple_match: vec![],
            ends_with: vec![],
        },
        case_insensitive,
    };

    let (mut before, after) = get_base_values("by_name", filter);

    before.clear();

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_multiple_extension_with_dot_passes() {
    let filter = FilterKind::Extension {
        exts: vec![String::from(".toml"), String::from(".jpg")],
    };

    let (mut before, after) = get_base_values("by_extension", filter);

    before.remove(0);

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_multiple_extensions_passes() {
    let filter = FilterKind::Extension {
        exts: vec![String::from("toml"), String::from("jpg")],
    };

    let (mut before, after) = get_base_values("by_extension", filter);

    before.remove(0);

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_single_extension_passes() {
    let filter = FilterKind::Extension {
        exts: vec![String::from("toml")],
    };

    let (mut before, after) = get_base_values("by_extension", filter);

    before.drain(..2);

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_folder_empty_passes() {
    std::fs::create_dir_all(get_fixtures_dir().join("empty_folder"))
        .expect("should be able to create dir structure.");
    let filter = FilterKind::Empty;

    let (before, after) = get_base_values("empty_folder", filter);

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_file_empty_passes() {
    let filter = FilterKind::Empty;

    let (mut before, after) = get_base_values("empty_file", filter);

    before.remove(0);
    before.remove(1);

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_file_size_2mb_passes() {
    let filter = FilterKind::Size {
        range: SizeRange::from_str("..2mb").unwrap(),
    };

    let (mut before, after) = get_base_values("size_based", filter);

    _ = before.remove(0);

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_file_size_350_800_kib_passes() {
    let filter = FilterKind::Size {
        range: SizeRange::from_str("350KiB..800kib").unwrap(),
    };

    let (mut before, after) = get_base_values("size_based", filter);

    before.clear();

    assert_eq!(before, after);
}
#[rstest]
fn test_filter_file_size_250kib_passes() {
    let filter = FilterKind::Size {
        range: SizeRange::from_str("250KiB..").unwrap(),
    };

    let (mut before, after) = get_base_values("size_based", filter);

    _ = before.pop();
    _ = before.remove(0);

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_ignore_single_str_is_in_path_passes() {
    let filter = FilterKind::IgnorePath {
        in_path: vec![String::from("bump")],
    };

    let (mut before, after) = get_base_values("ignore_path", filter);

    _ = before.drain(5..);

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_ignore_multiple_strs_is_in_path_passes() {
    let filter = FilterKind::IgnorePath {
        in_path: vec![String::from("bump"), String::from("bemp")],
    };

    let (mut before, after) = get_base_values("ignore_path", filter);

    _ = before.drain(3..);

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_ignore_single_str_is_in_name_passes() {
    let filter = FilterKind::IgnoreName {
        in_name: vec![String::from("ignore")],
    };

    let (mut before, after) = get_base_values("ignore_name", filter);

    assert_eq!(
        before.remove(4),
        get_fixtures_dir()
            .join("ignore_name")
            .join("bemp")
            .join("ignore.c")
    );

    assert_eq!(before, after);
}

#[rstest]
fn test_filter_ignore_multiple_strs_is_in_name_passes() {
    let filter = FilterKind::IgnoreName {
        in_name: vec![
            String::from("ignore"),
            String::from("a.txt"),
            String::from("bump"),
        ],
    };

    let (mut before, after) = get_base_values("ignore_name", filter);

    let removed = before.drain(6..9);
    assert_eq!(
        removed.into_iter().collect_vec(),
        vec![
            get_fixtures_dir()
                .join("ignore_name")
                .join("bump")
                .join("a.txt"),
            get_fixtures_dir()
                .join("ignore_name")
                .join("bump")
                .join("bump.txt"),
            get_fixtures_dir().join("ignore_name").join("bump.txt"),
        ]
    );
    assert_eq!(
        before.remove(4),
        get_fixtures_dir()
            .join("ignore_name")
            .join("bemp")
            .join("ignore.c")
    );
    assert_eq!(
        before.remove(1),
        get_fixtures_dir().join("ignore_name").join("a.txt")
    );
    assert_eq!(before, after);
}
