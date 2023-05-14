//! Organize
//!
//! Application based on the [Abscissa] framework.
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
pub mod application;
pub mod commands;
pub mod config;
pub mod error;
pub mod prelude;
