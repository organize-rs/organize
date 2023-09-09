# TODO

## Notes

- Enums: <https://github.com/Peternator7/strum>
- FS actions impl:
  <https://github.com/qarmin/czkawka/blob/master/czkawka_core/src/common_dir_traversal.rs>
- Template impl
  - <https://github.com/AmrDeveloper/GQL/blob/master/src/transformation.rs>
  - <https://github.com/AmrDeveloper/GQL/blob/master/src/expression.rs>
- Duplicates
  - <https://github.com/pkolaczk/fclones>
  - <https://github.com/qarmin/czkawka/blob/master/czkawka_core/src/>

## Templating

```rust
fn call_command(tpe: FileType, id: &Id, filename: &Path, command: &str) -> RusticResult<()> {
    let id = id.to_hex();
    let patterns = &["%file", "%type", "%id"];
    let ac = AhoCorasick::new(patterns).map_err(LocalErrorKind::FromAhoCorasick)?;
    let replace_with = &[filename.to_str().unwrap(), tpe.into(), id.as_str()];
    let actual_command = ac.replace_all(command, replace_with);
    debug!("calling {actual_command}...");
    let commands = parse_command::<()>(&actual_command)
        .map_err(LocalErrorKind::FromNomError)?
        .1;
    let status = Command::new(commands[0])
        .args(&commands[1..])
        .status()
        .map_err(LocalErrorKind::CommandExecutionFailed)?;
    if !status.success() {
        return Err(LocalErrorKind::CommandNotSuccessful {
            file_name: replace_with[0].to_owned(),
            file_type: replace_with[1].to_owned(),
            id: replace_with[2].to_owned(),
            status,
        }
        .into());
    }
    Ok(())
}
```

## Next

1. update screenshots and Readme.md
1. feat(templating): change arguments of some filters to be optional
   - [APPROACH1] FilterKind::Inspect/FilterKind::Template
     - and give information which template information we want to use (though we
       would still need to change return of `FilterClosures`)
   - [APPROACH2] use `FilterKind` to lookup available template strings
     - these can then be used in `actions` as e.g. `{{entry.size}}`
     - lookup table:
       `[key: FilterKind -> available_templates: Vec<TemplateKind>]`
1. generate example configs for integration test cases
   - check against py-organize documentation for feature parity
1. implement unit tests for actions and the actions themselves
1. implement `organize run config` with preview and destructive run
1. implement `Alias` being just defined via `Vec<Reference>` and used with the
   templating syntax `{{alias_name}}`

   ```yaml
   aliases:
   - !alias
       name: my_music_folders
       paths:
        - path1
        - path2
        - etc
   ```

   and then something like

   ```yaml
   locations:
   - {{alias.my_music_folders}}
   ```

   - check places where it is applicable
   - theoretically everywhere, where we take a `PathBuf` or `String` as part of
     a `Path`
     - e.g. extensions, locations, ignore_name, ignore_path
   - not usable in actions, or good error handling
     - e.g. someone could use an alias for a `move::dst`, which would
       essentially copy the file to these places
       - this should error out and actually be able to be checked beforehand,
         e.g. don't accept an alias that references more than one location
         string

## Features

1. implement global ignore list
   - .py
   - .js
   - .ini

1. implement file name templating
1. support batch renaming (e.g. rename all images in a directory to image_n
   where n is a number
   - remove some prefix from filenames
     - we want to have filename tags as well e.g. `dnt_` (do not touch) or
       `wip_` so these things are treated differently
     - also we want to have `{project_name}_` that we can remove after moving to
       a project folder
       - do we really want to remove that? maybe give that as an option
         `remove_keyword: bool`

1. implement Terminal UI
   - generate config interactively `generate config --interactive`
   - <https://crates.io/crates/ratatui/0.20.1> +
     <https://crates.io/crates/tui-react>

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

- it would be reasonable to have each rule only result in `1` destructive action
  (e.g. move, trash, delete, rename) so conflicts are already minimized
  - [RESEARCH] check which actions are parallelizable and for which actions it
    would make sense especially
  - add confirmation dialog for manual conflict handling

## Filters

### Filters impl

- [ ] Regex
  - `Fancy_regex` + `regex` crates
  - `Vec<Lazy<Regex>>` to have the regex compiled at runtime, but only once
    - Check regex implementation:
      <https://github.com/swanandx/lemmeknow/commit/25a98894b911e8c45954e0b8478397b06ae436bd>
- [ ] Exif
  - `kamadak-exif` crate
    - <https://docs.rs/kamadak-exif/latest/exif/fn.get_exif_attr_from_jpeg.html>
    - <https://docs.rs/kamadak-exif/latest/exif/fn.parse_exif.html>
- [ ] FileContent
- [ ] Duplicate
  - we can utilize `itertools::unique_by`
    - but that does already make it unique, so we need to see how we would
      collect the `DirEntry`, that was removed due to not being unique
      - easy: compare against a `copy`
    - we can use everything that is being exposed by the `DirEntry` itself
      (metadata.len(), filename(), etc.)
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
