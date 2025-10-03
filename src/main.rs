#[cfg(all(feature = "git", feature = "hg"))]
compile_error!("Cannot enable both git and hg features");

#[cfg(all(feature = "hg", feature = "git"))]
compile_error!("Cannot enable both hg and git features");

#[cfg(feature = "default")]
#[cfg(feature = "git")]
pub mod git;

#[cfg(feature = "hg")]
pub mod hg;

pub mod hooks;
pub mod utils;
use crate::utils::run_hooks;
use std::process::ExitCode;
#[cfg(feature = "git")]
fn main() -> ExitCode {
    if run_hooks().is_err() {
        return ExitCode::FAILURE;
    }
    git::run()
}

#[cfg(feature = "hg")]
fn main() -> ExitCode {
    if run_hooks().is_err() {
        return ExitCode::FAILURE;
    }
    hg::run()
}
