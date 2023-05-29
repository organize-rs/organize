# TODO

## Next

1. implement unit tests for filters
1. implement unit tests for actions
1. generate example configs for integration test cases
    - check against py-organize documentation for feature parity
1. implement `organize run config` for preview
1. implement destructive run

## Features

1. implement Terminal UI
    - generate config interactively `generate config --interactive`
    - <https://crates.io/crates/ratatui/0.20.1> + <https://crates.io/crates/tui-react>

1. implement scripting
1. build GUI

## Config

- generate fully commentated example config `generate config --template`

## Optional

1. implement deserializer for `py-organize` config

## General

- it would be reasonable to have each rule only result in `1` destructive action (e.g. move, trash, delete, rename) so conflicts are already minimized
  - check which actions are parallelizable
  - add confirmation dialog for manual conflict handling  

## Notes

- support batch renaming (e.g. rename all images in a directory to image_n where n is a number

- remove some prefix from filenames
  - we want to have filename tags as well e.g. `dnt_` (do not touch) or `wip_` so these things are treated differently
  - also we want to have `{project_name}_` that we can remove after moving to a project folder

## Filters

- global ignore list
  - .py
  - .js
  - .ini
  -

### Filters impl

- [ ] Regex
  - `Fancy_regex` + `regex` crates
  - `Vec<Lazy<Regex>>` to have the regex compiled at runtime, but only once
    - Check regex implementation: <https://github.com/swanandx/lemmeknow/commit/25a98894b911e8c45954e0b8478397b06ae436bd>
- [ ] Exif
  - `kamadak-exif` crate
    - <https://docs.rs/kamadak-exif/latest/exif/fn.get_exif_attr_from_jpeg.html>
    - <https://docs.rs/kamadak-exif/latest/exif/fn.parse_exif.html>
- [ ] FileContent
- [ ] Duplicate
  - we can utilize `itertools::unique_by`
    - but that does already make it unique, so we need to see how we would collect the `DirEntry`, that was removed due to not being unique
      - easy: compare against a `copy`
    - we can use everything that is being exposed by the `DirEntry` itself (metadata.len(), filename(), etc.)
- [ ] Added (OSX?)
- [ ] LastUsed (OSX?)
  - [RESEARCH] how does it relate to `LastAccessed`?

## Actions

- Check: <https://github.com/Byron/jwalk/blob/main/tests/util/mod.rs> for impl details

### Actions impl

- [ ] Trash
- [ ] Copy
- [ ] Move
- [ ] Rename
- [ ] Write
- [ ] Delete
- [ ] Symlink

#### Later

- [ ] Confirm
- [ ] Shell
- [ ] Echo
- [ ] Email
  - <https://crates.io/crates/lettre>
  - think about the best use case
