# TODO

## Next

1. generate example configs for integration test cases
    - check against py-organize documentation for feature parity
1. implement `organize run config` with preview
1. implement destructive run

## Features

1. implement Terminal UI
    - generate config interactively `generate config --interactive`

1. implement scripting
1. build GUI

## Config

- generate fully commentated example config `generate config --template`

## Optional

1. implement deserializer for `py-organize` config

## General

- how can we do a single run over a directory/files and apply many filters and actions?
  - how to safe guard against the case, that one filter can match a file/folder and multiple actions would want to do different things to them?
    - maybe the solution is to execute each rule + action immediately
      - maybe have a separate worker that just gets the `location` + `action_fn` and executes it
      - would also make more sense for `conflict handling`, because the files are actually already known to be there
    - or would it make more sense to apply the new `path` for each `location` and then do `conflict handling` before an action is even started?
    - maybe it woul be more reasonable to have each filter only result in `1` destructive action (e.g. move, trash, delete, rename)
      - check which actions are parallelizable

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
- [ ] Exif
- [ ] FileContent
- [ ] Duplicate
  - we can utilize `itertools::unique_by`
    - but that does already make it unique, so we need to see how we would collect the `DirEntry`, that was removed due to not being unique
    - we can use everything that is being exposed by the `DirEntry` itself (metadata.len(), filename(), etc.)
- [ ] Added (OSX?)
- [ ] LastUsed (OSX?)
  - [RESEARCH] how does it relate to `LastAccessed`?

## Actions

### Actions impl

- [ ] Trash
- [ ] Copy
- [ ] Move
- [ ] Rename
- [ ] Write
- [ ] Confirm
- [ ] Delete
- [ ] Shell
- [ ] Symlink
- [ ] Echo
- [ ] Email (?)
