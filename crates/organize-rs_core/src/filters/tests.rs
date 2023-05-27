//! unit tests for filters

use std::{
    ops::Not,
    path::{Path, PathBuf},
    str::FromStr,
};

use itertools::Itertools;
use jwalk::{DirEntry, WalkDir};
use pretty_assertions::assert_eq;

use crate::{filters::FilterKind, parsers::SizeRange};

fn get_fixtures_dir() -> PathBuf {
    vec!["tests", "fixtures", "filters"].iter().collect()
}

fn get_size_fixture() -> Vec<PathBuf> {
    vec![
        get_fixtures_dir().join("different_sizes"),
        get_fixtures_dir().join("different_sizes").join("1MiB"),
        get_fixtures_dir().join("different_sizes").join("300KiB"),
        get_fixtures_dir().join("different_sizes").join("empty.txt"),
    ]
}

fn get_ignore_path_fixture() -> Vec<PathBuf> {
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

fn get_ignore_name_fixture() -> Vec<PathBuf> {
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

#[test]
fn test_filter_file_size_2mb_passes() {
    let filter = FilterKind::Size {
        range: SizeRange::from_str("..2mb").unwrap(),
    };

    let mut entries = get_fixture_entries("different_sizes");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    let mut fixture = get_size_fixture();
    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    _ = fixture.remove(0);

    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
}

#[test]
fn test_filter_file_size_350_800_kib_passes() {
    let filter = FilterKind::Size {
        range: SizeRange::from_str("350KiB..800kib").unwrap(),
    };

    let mut entries = get_fixture_entries("different_sizes");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    let mut fixture = get_size_fixture();
    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    fixture.clear();

    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
}
#[test]
fn test_filter_file_size_250kib_passes() {
    let filter = FilterKind::Size {
        range: SizeRange::from_str("250KiB..").unwrap(),
    };

    let mut entries = get_fixture_entries("different_sizes");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    let mut fixture = get_size_fixture();
    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
    entries.retain(|f| (filter.get_filter()(f)));
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    _ = fixture.pop();
    _ = fixture.remove(0);

    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
}

#[test]
fn test_filter_ignore_single_str_is_in_path_passes() {
    let filter = FilterKind::IgnorePath {
        in_path: vec![String::from("bump")],
    };

    let mut entries = get_fixture_entries("ignore_path");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    let mut fixture = get_ignore_path_fixture();
    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
    entries.retain(|f| (filter.get_filter()(f)).not());
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    _ = fixture.drain(5..);
    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
}

#[test]
fn test_filter_ignore_multiple_strs_is_in_path_passes() {
    let filter = FilterKind::IgnorePath {
        in_path: vec![String::from("bump"), String::from("bemp")],
    };

    let mut entries = get_fixture_entries("ignore_path");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    let mut fixture = get_ignore_path_fixture();
    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
    entries.retain(|f| (filter.get_filter()(f)).not());
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    _ = fixture.drain(3..);
    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
}

#[test]
fn test_filter_ignore_single_str_is_in_name_passes() {
    let filter = FilterKind::IgnoreName {
        in_name: vec![String::from("ignore")],
    };

    let mut entries = get_fixture_entries("ignore_name");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    let mut fixture = get_ignore_name_fixture();
    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
    entries.retain(|f| (filter.get_filter()(f)).not());
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    assert_eq!(
        fixture.remove(4),
        get_fixtures_dir()
            .join("ignore_name")
            .join("bemp")
            .join("ignore.c")
    );
    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
}

#[test]
fn test_filter_ignore_multiple_strs_is_in_name_passes() {
    let filter = FilterKind::IgnoreName {
        in_name: vec![
            String::from("ignore"),
            String::from("a.txt"),
            String::from("bump"),
        ],
    };

    let mut entries = get_fixture_entries("ignore_name");
    let paths = entries.iter().map(|f| f.path()).collect_vec();
    let mut fixture = get_ignore_name_fixture();
    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
    entries.retain(|f| (filter.get_filter()(f)).not());
    let paths = entries.iter().map(|f| f.path()).collect_vec();

    let removed = fixture.drain(6..9);
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
        fixture.remove(4),
        get_fixtures_dir()
            .join("ignore_name")
            .join("bemp")
            .join("ignore.c")
    );
    assert_eq!(
        fixture.remove(1),
        get_fixtures_dir().join("ignore_name").join("a.txt")
    );
    assert_eq!(entries.len(), fixture.len());
    assert_eq!(paths, fixture);
}
