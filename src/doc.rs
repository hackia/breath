use crate::config::load_config;
use crate::utils::ok;
use regex::Regex;
use std::io::Error;
use std::process::{Command, ExitCode};

/// check if a command it's safe to execute
/// # Panics
/// if the regex is not valid
#[must_use]
pub fn is_safe_command(cmd: &str) -> bool {
    let re = Regex::new(r"([;&|`$]|\.\.)").expect("invalid regex for is_safe_command");
    let starts_with_slash = cmd.trim().starts_with('/');
    let allowed_bins = ["cargo", "doxygen", "rm", "cp", "mkdir", "touch"];
    let binary_name = cmd.split_whitespace().next().unwrap_or("");
    let is_allowed_bin = allowed_bins.contains(&binary_name);
    !re.is_match(cmd) && !starts_with_slash && is_allowed_bin
}

/// run a command and describe it with success and failure messages
///
/// # Errors
/// on no founded command or bad user input
pub fn describe(it: &str, command: &str, success: &str, failure: &str) -> Result<(), Error> {
    if !is_safe_command(command) {
        return Err(Error::other("Security check failed"));
    }
    if cfg!(windows) {
        ok(
            it,
            Command::new("cmd").arg("/C").arg(command),
            success,
            failure,
        )
    } else {
        ok(
            it,
            Command::new("sh").arg("-c").arg(command),
            success,
            failure,
        )
    }
}

///
/// Generate man pages
///
/// # Errors
/// On bad user inputs
///
#[must_use]
pub fn generate_man() -> ExitCode {
    let config = load_config();
    let len = config.documentation.man.len();
    for (i, command) in config.documentation.man.iter().enumerate() {
        if describe(
            format!("Building Man pages {}/{len}", i + 1).as_str(),
            command.as_str(),
            format!("Man step {}/{len} has been completed successfully", i + 1).as_str(),
            format!("Man step {}/{len} has failed", i + 1).as_str(),
        )
        .is_err()
        {
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
///
/// generate github pages docs
///
/// # Errors
/// On bad user inputs
#[must_use]
pub fn generate_doc() -> ExitCode {
    let config = load_config();
    let len = config.documentation.doc.len();
    for (i, command) in config.documentation.doc.iter().enumerate() {
        if describe(
            format!("Building documentation {}/{len}", i + 1).as_str(),
            command.as_str(),
            format!(
                "Documentation step {}/{len} has been completed successfully",
                i + 1
            )
            .as_str(),
            format!("Documentation step {}/{len} has failed", i + 1).as_str(),
        )
        .is_err()
        {
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
