//! unit tests for filters

// use quickcheck_macros::quickcheck;

use std::{
    ops::Not,
    path::{Path, PathBuf},
    str::FromStr,
    time::SystemTime,
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

#[fixture]
fn name() -> Vec<PathBuf> {
    vec![
        get_fixtures_dir().join("by_name"),
        get_fixtures_dir().join("by_name").join("123test1.txt"),
        get_fixtures_dir().join("by_name").join("456test2.txt"),
        get_fixtures_dir().join("by_name").join("789TaSt.jpg"),
        get_fixtures_dir().join("by_name").join("TEST123.txt"),
        get_fixtures_dir().join("by_name").join("uTEST.txt"),
    ]
}

#[fixture]
fn mimetype() -> Vec<PathBuf> {
    vec![
        get_fixtures_dir().join("mimetype"),
        get_fixtures_dir().join("mimetype").join("a.txt"),
        get_fixtures_dir().join("mimetype").join("b.jpg"),
        get_fixtures_dir().join("mimetype").join("c.odt"),
    ]
}

#[fixture]
fn extension() -> Vec<PathBuf> {
    vec![
        get_fixtures_dir().join("by_extension"),
        get_fixtures_dir().join("by_extension").join("test.jpg"),
        get_fixtures_dir().join("by_extension").join("test.toml"),
    ]
}

#[fixture]
fn empty_directory() -> Vec<PathBuf> {
    vec![get_fixtures_dir().join("empty_folder")]
}

#[fixture]
fn empty_file() -> Vec<PathBuf> {
    vec![
        get_fixtures_dir().join("empty_file"),
        get_fixtures_dir().join("empty_file").join("empty.txt"),
        get_fixtures_dir().join("empty_file").join("not_empty.txt"),
    ]
}

#[fixture]
fn size() -> Vec<PathBuf> {
    vec![
        get_fixtures_dir().join("size_based"),
        get_fixtures_dir().join("size_based").join("1MiB"),
        get_fixtures_dir().join("size_based").join("300KiB"),
        get_fixtures_dir().join("size_based").join("empty.txt"),
    ]
}

#[fixture]
fn ignore_path() -> Vec<PathBuf> {
    vec![
        get_fixtures_dir().join("ignore_path"),
        get_fixtures_dir().join("ignore_path").join("a.txt"),
        get_fixtures_dir().join("ignore_path").join("b.txt"),
        get_fixtures_dir().join("ignore_path").join("bemp"),
        get_fixtures_dir()
            .join("ignore_path")
            .join("bemp")
            .join("d.txt"),
        get_fixtures_dir().join("ignore_path").join("bump"),
        get_fixtures_dir()
            .join("ignore_path")
            .join("bump")
            .join("c.txt"),
    ]
}

#[fixture]
fn ignore_name() -> Vec<PathBuf> {
    vec![
        get_fixtures_dir().join("ignore_name"),
        get_fixtures_dir().join("ignore_name").join("a.txt"),
        get_fixtures_dir().join("ignore_name").join("bemp"),
        get_fixtures_dir()
            .join("ignore_name")
            .join("bemp")
            .join("d.txt"),
        get_fixtures_dir()
            .join("ignore_name")
            .join("bemp")
            .join("ignore.c"),
        get_fixtures_dir().join("ignore_name").join("bump"),
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
}

fn get_fixture_entries(sub_dir: impl AsRef<Path>) -> Vec<DirEntry<((), ())>> {
    let mut to_walk = get_fixtures_dir();
    to_walk.push(sub_dir.as_ref());
    WalkDir::new(to_walk)
        .into_iter()
        .filter_map(|f| f.ok())
        .collect_vec()
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
fn test_filter_mimetype_image_fails(mut mimetype: Vec<PathBuf>) {
    let filter = FilterKind::Mimetype {
        mimetype: vec![String::from("image")],
    };

    let mut entries = get_fixture_entries("mimetype");
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), mimetype.len());
    assert_eq!(paths, mimetype);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    mimetype.remove(3);
    mimetype.remove(1);
    mimetype.remove(0);
    assert_eq!(entries.len(), mimetype.len());
    assert_eq!(paths, mimetype);
}

#[rstest]
fn test_filter_mimetype_jpg_odt_passes(mut mimetype: Vec<PathBuf>) {
    let filter = FilterKind::Mimetype {
        mimetype: vec![
            String::from("application/vnd.oasis.opendocument.text"),
            String::from("image/jpeg"),
        ],
    };

    let mut entries = get_fixture_entries("mimetype");
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), mimetype.len());
    assert_eq!(paths, mimetype);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    mimetype.remove(1);
    mimetype.remove(0);
    assert_eq!(entries.len(), mimetype.len());
    assert_eq!(paths, mimetype);
}

#[rstest]
fn test_filter_all_items_passes(name: Vec<PathBuf>) {
    let filter = FilterKind::AllItems {
        i_agree_it_is_dangerous: true,
    };

    let mut entries = get_fixture_entries("by_name");
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), name.len());
    assert_eq!(paths, name);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), name.len());
    assert_eq!(paths, name);
}

#[rstest]
fn test_filter_no_filter_passes(mut name: Vec<PathBuf>) {
    let filter = FilterKind::NoFilter;

    let mut entries = get_fixture_entries("by_name");
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), name.len());
    assert_eq!(paths, name);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    name.clear();
    assert_eq!(entries.len(), name.len());
    assert_eq!(paths, name);
}

#[rstest]
#[case(true)]
#[case(false)]
fn test_filter_name_full_passes(mut name: Vec<PathBuf>, #[case] case_insensitive: bool) {
    let filter = FilterKind::Name {
        arguments: NameFilterArgs {
            contains: vec![String::from("TaSt"), String::from("-|uTEST")],
            starts_with: vec![String::from("123")],
            simple_match: vec![],
            ends_with: vec![String::from("2")],
        },
        case_insensitive,
    };

    let mut entries = get_fixture_entries("by_name");
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), name.len());
    assert_eq!(paths, name);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    if case_insensitive {
        name.remove(5);
        name.remove(4);
        name.remove(0);
        assert_eq!(entries.len(), name.len());
        assert_eq!(paths, name);
    } else {
        name.remove(5);
        name.remove(4);
        name.remove(0);
        assert_eq!(entries.len(), name.len());
        assert_eq!(paths, name);
    }
}

#[rstest]
#[case(true)]
#[case(false)]
fn test_filter_name_contains_multiple_names_and_inverted_passes(
    mut name: Vec<PathBuf>,
    #[case] case_insensitive: bool,
) {
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

    let mut entries = get_fixture_entries("by_name");
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), name.len());
    assert_eq!(paths, name);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    if case_insensitive {
        name.remove(5);
        name.remove(0);
        assert_eq!(entries.len(), name.len());
        assert_eq!(paths, name);
    } else {
        name.remove(5);
        name.remove(4);
        name.remove(0);
        assert_eq!(entries.len(), name.len());
        assert_eq!(paths, name);
    }
}

#[rstest]
#[case(true)]
#[case(false)]
fn test_filter_name_contains_multiple_names_passes(
    mut name: Vec<PathBuf>,
    #[case] case_insensitive: bool,
) {
    let filter = FilterKind::Name {
        arguments: NameFilterArgs {
            contains: vec![String::from("test"), String::from("TaSt")],
            starts_with: vec![],
            simple_match: vec![],
            ends_with: vec![],
        },
        case_insensitive,
    };

    let mut entries = get_fixture_entries("by_name");
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), name.len());
    assert_eq!(paths, name);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    if case_insensitive {
        name.remove(0);
        assert_eq!(entries.len(), name.len());
        assert_eq!(paths, name);
    } else {
        name.remove(5);
        name.remove(4);
        name.remove(0);
        assert_eq!(entries.len(), name.len());
        assert_eq!(paths, name);
    }
}

#[rstest]
#[case(true)]
#[case(false)]
fn test_filter_name_contains_single_name_passes(
    mut name: Vec<PathBuf>,
    #[case] case_insensitive: bool,
) {
    let filter = FilterKind::Name {
        arguments: NameFilterArgs {
            contains: vec![String::from("test")],
            starts_with: vec![],
            simple_match: vec![],
            ends_with: vec![],
        },
        case_insensitive,
    };

    let mut entries = get_fixture_entries("by_name");
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), name.len());
    assert_eq!(paths, name);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    if case_insensitive {
        dbg!(&entries);
        name.remove(3);
        name.remove(0);
        assert_eq!(entries.len(), name.len());
        assert_eq!(paths, name);
    } else {
        name.remove(5);
        name.remove(4);
        name.remove(3);
        name.remove(0);
        assert_eq!(entries.len(), name.len());
        assert_eq!(paths, name);
    }
}

#[rstest]
#[case(true)]
#[case(false)]
fn test_filter_name_args_does_not_match_anything_passes(
    name: Vec<PathBuf>,
    #[case] case_insensitive: bool,
) {
    let filter = FilterKind::Name {
        arguments: NameFilterArgs {
            contains: vec![String::from("toast")],
            starts_with: vec![],
            simple_match: vec![],
            ends_with: vec![],
        },
        case_insensitive,
    };

    let mut entries = get_fixture_entries("by_name");
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), name.len());
    assert_eq!(paths, name);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert!(entries.is_empty());
    assert!(paths.is_empty());
}

#[rstest]
#[case(true)]
#[case(false)]
fn test_filter_name_args_empty_passes(name: Vec<PathBuf>, #[case] case_insensitive: bool) {
    let filter = FilterKind::Name {
        arguments: NameFilterArgs {
            contains: vec![],
            starts_with: vec![],
            simple_match: vec![],
            ends_with: vec![],
        },
        case_insensitive,
    };

    let mut entries = get_fixture_entries("by_name");
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), name.len());
    assert_eq!(paths, name);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert!(entries.is_empty());
    assert!(paths.is_empty());
}

#[rstest]
fn test_filter_multiple_extension_with_dot_passes(mut extension: Vec<PathBuf>) {
    let filter = FilterKind::Extension {
        exts: vec![String::from(".toml"), String::from(".jpg")],
    };

    let mut entries = get_fixture_entries("by_extension");
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), extension.len());
    assert_eq!(paths, extension);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    extension.remove(0);
    assert_eq!(entries.len(), extension.len());
    assert_eq!(paths, extension);
}

#[rstest]
fn test_filter_multiple_extensions_passes(mut extension: Vec<PathBuf>) {
    let filter = FilterKind::Extension {
        exts: vec![String::from("toml"), String::from("jpg")],
    };

    let mut entries = get_fixture_entries("by_extension");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    assert_eq!(entries.len(), extension.len());
    assert_eq!(paths, extension);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    extension.remove(0);
    assert_eq!(entries.len(), extension.len());
    assert_eq!(paths, extension);
}

#[rstest]
fn test_filter_single_extension_passes(mut extension: Vec<PathBuf>) {
    let filter = FilterKind::Extension {
        exts: vec![String::from("toml")],
    };

    let mut entries = get_fixture_entries("by_extension");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    assert_eq!(entries.len(), extension.len());
    assert_eq!(paths, extension);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    extension.drain(..2);
    assert_eq!(entries.len(), extension.len());
    assert_eq!(paths, extension);
}

#[rstest]
fn test_filter_folder_empty_passes(empty_directory: Vec<PathBuf>) {
    std::fs::create_dir_all(get_fixtures_dir().join("empty_folder"))
        .expect("should be able to create dir structure.");
    let filter = FilterKind::Empty;

    let mut entries = get_fixture_entries("empty_folder");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    assert_eq!(entries.len(), empty_directory.len());
    assert_eq!(paths, empty_directory);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), empty_directory.len());
    assert_eq!(paths, empty_directory);
}

#[rstest]
fn test_filter_file_empty_passes(mut empty_file: Vec<PathBuf>) {
    let filter = FilterKind::Empty;

    let mut entries = get_fixture_entries("empty_file");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    assert_eq!(entries.len(), empty_file.len());
    assert_eq!(paths, empty_file);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    empty_file.remove(0);
    empty_file.remove(1);
    assert_eq!(entries.len(), empty_file.len());
    assert_eq!(paths, empty_file);
}

#[rstest]
fn test_filter_file_size_2mb_passes(mut size: Vec<PathBuf>) {
    let filter = FilterKind::Size {
        range: SizeRange::from_str("..2mb").unwrap(),
    };

    let mut entries = get_fixture_entries("size_based");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    assert_eq!(entries.len(), size.len());
    assert_eq!(paths, size);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    _ = size.remove(0);

    assert_eq!(entries.len(), size.len());
    assert_eq!(paths, size);
}

#[rstest]
fn test_filter_file_size_350_800_kib_passes(mut size: Vec<PathBuf>) {
    let filter = FilterKind::Size {
        range: SizeRange::from_str("350KiB..800kib").unwrap(),
    };

    let mut entries = get_fixture_entries("size_based");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    assert_eq!(entries.len(), size.len());
    assert_eq!(paths, size);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    size.clear();

    assert_eq!(entries.len(), size.len());
    assert_eq!(paths, size);
}
#[rstest]
fn test_filter_file_size_250kib_passes(mut size: Vec<PathBuf>) {
    let filter = FilterKind::Size {
        range: SizeRange::from_str("250KiB..").unwrap(),
    };

    let mut entries = get_fixture_entries("size_based");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    assert_eq!(entries.len(), size.len());
    assert_eq!(paths, size);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    _ = size.pop();
    _ = size.remove(0);

    assert_eq!(entries.len(), size.len());
    assert_eq!(paths, size);
}

#[rstest]
fn test_filter_ignore_single_str_is_in_path_passes(mut ignore_path: Vec<PathBuf>) {
    let filter = FilterKind::IgnorePath {
        in_path: vec![String::from("bump")],
    };

    let mut entries = get_fixture_entries("ignore_path");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    assert_eq!(entries.len(), ignore_path.len());
    assert_eq!(paths, ignore_path);
    entries.retain(|f| (filter.get_filter()(f)).not());
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    _ = ignore_path.drain(5..);
    assert_eq!(entries.len(), ignore_path.len());
    assert_eq!(paths, ignore_path);
}

#[rstest]
fn test_filter_ignore_multiple_strs_is_in_path_passes(mut ignore_path: Vec<PathBuf>) {
    let filter = FilterKind::IgnorePath {
        in_path: vec![String::from("bump"), String::from("bemp")],
    };

    let mut entries = get_fixture_entries("ignore_path");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    assert_eq!(entries.len(), ignore_path.len());
    assert_eq!(paths, ignore_path);
    entries.retain(|f| (filter.get_filter()(f)).not());
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    _ = ignore_path.drain(3..);
    assert_eq!(entries.len(), ignore_path.len());
    assert_eq!(paths, ignore_path);
}

#[rstest]
fn test_filter_ignore_single_str_is_in_name_passes(mut ignore_name: Vec<PathBuf>) {
    let filter = FilterKind::IgnoreName {
        in_name: vec![String::from("ignore")],
    };

    let mut entries = get_fixture_entries("ignore_name");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    assert_eq!(entries.len(), ignore_name.len());
    assert_eq!(paths, ignore_name);
    entries.retain(|f| (filter.get_filter()(f)).not());
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    assert_eq!(
        ignore_name.remove(4),
        get_fixtures_dir()
            .join("ignore_name")
            .join("bemp")
            .join("ignore.c")
    );
    assert_eq!(entries.len(), ignore_name.len());
    assert_eq!(paths, ignore_name);
}

#[rstest]
fn test_filter_ignore_multiple_strs_is_in_name_passes(mut ignore_name: Vec<PathBuf>) {
    let filter = FilterKind::IgnoreName {
        in_name: vec![
            String::from("ignore"),
            String::from("a.txt"),
            String::from("bump"),
        ],
    };

    let mut entries = get_fixture_entries("ignore_name");
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    assert_eq!(entries.len(), ignore_name.len());
    assert_eq!(paths, ignore_name);
    entries.retain(|f| (filter.get_filter()(f)).not());
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    let removed = ignore_name.drain(6..9);
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
        ignore_name.remove(4),
        get_fixtures_dir()
            .join("ignore_name")
            .join("bemp")
            .join("ignore.c")
    );
    assert_eq!(
        ignore_name.remove(1),
        get_fixtures_dir().join("ignore_name").join("a.txt")
    );
    assert_eq!(entries.len(), ignore_name.len());
    assert_eq!(paths, ignore_name);
}
