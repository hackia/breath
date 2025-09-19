#[cfg(all(feature = "git", feature = "hg"))]
compile_error!("Cannot enable both git and hg features");

#[cfg(all(feature = "git", feature = "default"))]
compile_error!("Cannot enable both git and default features");

#[cfg(all(feature = "hg", feature = "git"))]
compile_error!("Cannot enable both hg and git features");

#[cfg(all(feature = "hg", feature = "default"))]
compile_error!("Cannot enable both hg and default features");

#[cfg(feature = "default")]
use crate::utils::verify;

#[cfg(feature = "git")]
pub mod git;

#[cfg(feature = "hg")]
pub mod hg;

pub mod utils;
use std::process::ExitCode;

#[cfg(feature = "default")]
fn main() -> ExitCode {
    if verify() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(feature = "git")]
fn main() -> ExitCode {
    git::run()
}

#[cfg(feature = "hg")]
fn main() -> ExitCode {
    hg::run()
}
