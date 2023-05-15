# organize  [![AGPLv3+](https://www.gnu.org/graphics/agplv3-88x31.png)](https://www.gnu.org/licenses/agpl.txt)

<a href="https://crates.io/crates/organize-rs/"><img src="https://img.shields.io/crates/v/organize-rs?style=flat&amp;labelColor=342a5e&amp;color=684d81&amp;logo=Rust&amp;logoColor=white" alt="Crate Release"></a> <a href="https://docs.rs/organize-rs/"><img src="https://img.shields.io/docsrs/organize-rs?style=flat&amp;labelColor=1c1d42&amp;color=4f396a&amp;logo=Rust&amp;logoColor=white" alt="Documentation"></a>

A file management automation tool.

## Current Status

This is in really early development. Please come back later!

## Background

The Python [organize](https://organize.readthedocs.io/) is a file management automation tool.

From their docs:
> Your desktop is a mess? You cannot find anything in your downloads and documents? Sorting and renaming all these files by hand is too tedious? Time to automate it once and benefit from it forever.
> organize is a command line, open-source alternative to apps like Hazel (macOS) or File Juggler (Windows).

This is a Rust implementation of the same concept.

## Migration from `py-organize`

- copy your `config.yaml` and rename it to `organize.yaml`
- rework `anchors` in your new `organize.yaml` config file
  from:

  ```yaml
  desktop_folder: &desktop
  - '~/Desktop/'
  applications: &apps
  - exe
  - msi
  - apk  
  ```

  to

  ```yaml
  aliases:
  - name: desktop
    kind: folder
    items:
      - '~/Desktop/'
  - name: apps
    kind: ext
    items:
      - exe
      - msi
      - apk
  ```

- then rework the `aliases` correspondingly from:

  ```yaml
  # Rule for desktop/downloads to move applications into @Apps
  - folders: 
      - *downloads
      - *desktop
      - *inbox
    subfolders: false
    filters:
      - extension: 
          - *apps
    actions:
      - move: '~/backup/INBOX/@Apps/'
  ```

  ```yaml
  # Rule for desktop/downloads to move applications into @Apps
  - locations: 
      - ref|downloads
      - ref|desktop
      - ref|inbox
    subfolders: false
    filters:
      - extension: 
          - ref|apps
    actions:
      - move: '~/backup/INBOX/@Apps/'
  ```

## Goals

For now the first goal for this Rust version of `organize` is to have feature parity (commands) with its Python equivalent.
Though, breaking changes may occur going forward, for the beginning it should work as a drop-in replacement.

A big factor for the Rust port for me is that I like Rust. I want to reimplement `organize` in language that has better error handling, and makes it easier to maintain software. It's fast and at the same time makes development less error prone.

I'm the first user of the Rust implementation, and will be going to use it with my private files. Thus an important goal for me is stability.

## Non goals

The Python implementation supports filters and actions that can be passed in via the config file and are written themselves in `Python`. For me it's not a reasonable effort to support `Python` features in that regards.

## License

**AGPL-3.0-or-later**; see [LICENSE](./LICENSE).
