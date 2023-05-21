# TODO

## General

- think about how to combine filters with actions
  - maybe something like an `ActionPack` where `filters` and their corresponding `actions` are stored
  - what is a good data type for that?
- how can we do a single run over a directory/files and apply many filters and actions?
  - how to safe guard against the case, that one filter can match a file/folder and multiple actions would want to do different things to them?
    - maybe the solution is to execute each action immediately
      - maybe have a separate worker that just gets the `location` + `action_fn` and executes it
      - would also make more sense for `conflict handling`, because the files are actually already known to be there
    - or would it make more sense to apply the new `path` for each `location` and then do `conflict handling` before an action is even started?

## Filters

### Filters impl

- [ ] Regex
- [ ] Exif
- [ ] FileContent
- [ ] Duplicate
- [ ] Added (OSX?)
- [ ] LastUsed (OSX?)
  - [RESEARCH] how does it relate to `LastAccessed`?

## Actions

### Actions impl

- [ ] None
  - [RESEARCH] is a `do nothing` action that useful even?
    - for non-destructive actions we have the `dry-run` mode and `destructive` flag
- [ ] Confirm
- [ ] Copy
- [ ] Delete
- [ ] Echo
- [ ] Move
- [ ] Rename
- [ ] Symlink
- [ ] Trash
- [ ] Write
- [ ] Shell
- [ ] Email (?)
