//! Main entry point for Organize

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use organize::application::ORGANIZE_APP;

/// Boot Organize
fn main() {
    abscissa_core::boot(&ORGANIZE_APP);
}
