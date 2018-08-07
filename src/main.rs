#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(feature = "cargo-clippy", deny(warnings))]

extern crate dwm_status;

use std::process;

fn main() {
    if let Err(error) = dwm_status::run() {
        error.show_error();
        process::exit(1);
    }
}
