pub mod commit;
pub mod hooks;
pub mod utils;
use crate::commit::{Commit, run_commit, vcs};
use crate::utils::{call, run_hooks, zen};
use clap::Arg;
use std::process::ExitCode;

fn breathes() -> clap::ArgMatches {
    clap::Command::new("breath")
        .version(env!("CARGO_PKG_VERSION"))
        .author("hackia <dev@hackia.org>")
        .about("A tool for managing Git and Mercurial repositories")
        .subcommand(
            clap::Command::new("config")
                .about("Manage configuration")
                .arg(
                    Arg::new("vcs")
                        .value_names(["git", "hg"])
                        .default_missing_value("git")
                        .default_value("git")
                        .required(true),
                ),
        )
        .subcommand(clap::Command::new("health").about("Verify repository health"))
        .subcommand(clap::Command::new("commit").about("Commit changes to the repository"))
        .subcommand(clap::Command::new("push").about("Push changes to remote repositories"))
        .subcommand(clap::Command::new("pull").about("Pull changes from remote repositories"))
        .subcommand(clap::Command::new("status").about("Show the status of the repository"))
        .subcommand(clap::Command::new("zen").about("display a loop menu to interact with breath"))
        .subcommand(clap::Command::new("log").about("Show the commit log"))
        .subcommand(
            clap::Command::new("diff")
                .about("Show the changes between the working directory and the index"),
        )
        .get_matches()
}

fn main() -> ExitCode {
    let mut commit = Commit::new();
    let app = breathes();

    match app.subcommand() {
        Some(("health", _)) => {
            if run_hooks().is_err() {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Some(("zen", _)) => {
            if zen().is_err() {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Some(("commit", _)) => {
            if run_hooks().is_ok() {
                commit.commit().map_or(ExitCode::FAILURE, |c| {
                    if run_commit(c).is_err() {
                        ExitCode::FAILURE
                    } else {
                        ExitCode::SUCCESS
                    }
                })
            } else {
                // Les hooks ont échoué
                ExitCode::FAILURE
            }
        }
        // Ce bloc corrige toutes les erreurs de répétition
        Some((cmd @ ("push" | "pull" | "status" | "log" | "diff"), _)) => {
            if call(vcs().as_str(), cmd).is_ok() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        _ => ExitCode::FAILURE,
    }
}
