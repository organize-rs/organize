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

## Example

- Filter `files`, which file stem ends with `go`, recursive, maximum `2` levels deep:

  `organize filter --recursive --max-depth 2 name --locations "C:\organize\" --targets files --ends-with "go"`

- Filter `files` in `two locations`, which extensions match `rs` or `toml`, recursive, maximum `2` levels deep

  `organize filter --recursive --max-depth 2 extension --locations C:\organize --locations D:\folders --targets files --exts rs --exts toml`

- Filter `files` and `folders`, which are empty (`0 bytes` or `no files` in directory), recursive, maximum `4` levels deep, ignore `git` in path names

  `organize filter --recursive --max-depth 4 empty --locations "C:\Users\dailyuse\dev-src\organize\" --targets both --ignore-path git`

- Filter `files` and `folders`, which are empty (`0 bytes` or `no files` in directory), recursive, maximum `4` levels deep, ignore `git` only in file names

  `organize filter --recursive --max-depth 4 empty --locations "C:\Users\dailyuse\dev-src\organize\" --targets both --ignore-name git`

## Media

**Be aware: This is WIP. Not all functionality is implemented, (yet).**

### Main

![organize main menu](https://github.com/organize-rs/organize/blob/main/docs/screenshots/main.png?raw=true)

### Filters

![organize filters](https://github.com/organize-rs/organize/blob/main/docs/screenshots/filters.png?raw=true)

### Actions

![organize actions](https://github.com/organize-rs/organize/blob/main/docs/screenshots/actions.png?raw=true)

## Goals

For now the first goal for this Rust version of `organize` is to have feature parity with its Python equivalent.

**BUT**: I want to take another approach on tackling the problem. It is also relatively complicated to map all the stuff
within a `config` file, because we are bound to the syntax of `yaml`/`json`/`toml`/`ron`.

*Maybe this is exactly the problem to solve!*

Basically you want to have a configuration file, which replaces a mini scripting language.
With predefined `filters` and `actions` that are applied to the items that the filter spits out.

Basically almost everything in the configuration files are parameters for functions/methods.

This makes everything more complicated.

1. What if we implement rusty `organize` in a way, that we can call `organize filter extension --ext "exe, msi, apk" --path {}`
and it spits out all the paths that match this `precoded` filter?
This way we can already utilize it easily within shell scripts.

1. On the second implementation stage, we can embed a scripting engine like [rhai](https://crates.io/crates/rhai), where we expose some functionality of rusty `organize` as `functions` and register them with the `rhai` engine.

1. Instead of pressing everything in a complicated configuration file syntax, we can utilize a scripting language and boil it down to its minimum syntax.

That being said, a big factor for the Rust reiteration for me is that I like Rust. I want to reimplement `organize`'s approach in a language that has better error handling, and makes it easier to maintain software. That is fast and at the same time makes development less error prone.

I'm the first user of the Rust implementation, and will be going to use it with my private files. Thus an important goal for me is stability.

## License

**AGPL-3.0-or-later**; see [LICENSE](./LICENSE).
