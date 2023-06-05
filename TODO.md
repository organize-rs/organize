# TODO

## Next

1. update screenshots and Readme.md
1. change arguments to some filters to be optional
    - so they return a broad range of information, that can be used to `echo`
    - or rather create a new `info`/`inspect` filter for that
        - which we can pass as an argument, which attribute we want to inspect
1. generate example configs for integration test cases
    - check against py-organize documentation for feature parity
1. implement unit tests for actions and the actions themselves
1. implement `organize run config` with preview and destructive run
1. implement `Alias` being just defined via `Vec<Reference>` and used with the templating syntax `{{alias_name}}`
    - check places where it is applicable
    - theoretically everywhere, where we take a `PathBuf` or `String` as part of a `Path`
      - e.g. extensions, locations, ignore_name, ignore_path
    - not usable in actions, or good error handling
      - e.g. someone could use an alias for a `move::dst`, which would essentially copy the file to these places
        - this should error out and actually be able to be checked beforehand, e.g. don't accept an alias that references more than one location string

## Features

1. implement global ignore list
    - .py
    - .js
    - .ini

1. implement file name templating
    - e.g. with <https://lib.rs/crates/tinytemplate> or <https://lib.rs/crates/handlebars>
    - or bare with `.replace` and own placeholders like `{{file_name}}`, `{{counter}}` etc.
    - support batch renaming (e.g. rename all images in a directory to image_n where n is a number
    - remove some prefix from filenames
      - we want to have filename tags as well e.g. `dnt_` (do not touch) or `wip_` so these things are treated differently
      - also we want to have `{project_name}_` that we can remove after moving to a project folder

1. implement Terminal UI
    - generate config interactively `generate config --interactive`
    - <https://crates.io/crates/ratatui/0.20.1> + <https://crates.io/crates/tui-react>

1. save state/event log between runs
    - <https://lib.rs/crates/sled>

1. implement scripting
1. build GUI

## Testing

1. implement unit tests for (new upcoming) filters

## De-/Serialisation

1. use <https://docs.rs/serde_with/3.0.0/serde_with/index.html>
1. implement deserializer for `py-organize` config

## General

- it would be reasonable to have each rule only result in `1` destructive action (e.g. move, trash, delete, rename) so conflicts are already minimized
  - [RESEARCH] check which actions are parallelizable and for which actions it would make sense especially
  - add confirmation dialog for manual conflict handling  

## Filters

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
- [ ] BrokenLinks
  - matching soft-links whose files are non-existent
- [ ] Added (OSX?)
- [ ] LastUsed (OSX?)
  - [RESEARCH] how does it relate to `LastAccessed`?

## Actions

### Actions impl

- [ ] Copy
  - `copy_to`
- [ ] Move
  - `move_to`
- [ ] Rename
  - `rename_to`
- [ ] Write
- [ ] Echo
- [ ] Confirm

#### Later

- [ ] Shell
- [ ] Email
  - <https://crates.io/crates/lettre>
  - think about the best use case
