//! organize
//! The Python [organize](https://organize.readthedocs.io/) is a file management automation tool.
//!
//! From their docs:
//! > Your desktop is a mess? You cannot find anything in your downloads and documents? Sorting and renaming all these files by hand is too tedious? Time to automate it once and benefit from it forever.
//! > organize is a command line, open-source alternative to apps like Hazel (macOS) or File Juggler (Windows).
//!  
//! This is a Rust implementation of the same concept.
//!
//! This application is based on the [Abscissa] framework.
//!
//! [Abscissa]: https://github.com/iqlusioninc/abscissa

#![forbid(unsafe_code)]
#![warn(
    // missing_docs, // TODO: uncomment if you feel like writing some more docs
    clippy::pedantic,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::nursery,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]
#[allow(clippy::module_name_repetitions)]
// Modules of the app
pub mod application;
pub mod commands;
pub mod config;
pub mod error;
pub mod prelude;
pub mod scripting;

// re-exports for documentation purposes
pub use organize_rs_core::actions::{
    conflicts::ConflictResolutionKind, ActionKind, TemplateStringKind,
};
pub use organize_rs_core::filters::FilterKind;
