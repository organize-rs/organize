//! Main entry point for Organize

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use human_panic::setup_panic;
use organize_rs::application::ORGANIZE_APP;

/// Boot Organize
fn main() {
    setup_panic!();
    abscissa_core::boot(&ORGANIZE_APP);
}
