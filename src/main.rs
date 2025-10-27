pub mod commit;
pub mod hooks;
pub mod utils;

use crate::commit::{Commit, vcs};
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
    if app.subcommand_matches("health").is_some() && run_hooks().is_err() {
        return ExitCode::FAILURE;
    }
    if app.subcommand_matches("zen").is_some() && zen().is_err() {
        ExitCode::FAILURE
    } else if app.subcommand_matches("commit").is_some()
        && run_hooks().is_ok()
        && let Ok(c) = commit.commit()
    {
        println!("Commited: {c}");
        return ExitCode::SUCCESS;
    } else if app.subcommand_matches("push").is_some() && call(vcs().as_str(), "push") {
        return ExitCode::SUCCESS;
    } else if app.subcommand_matches("pull").is_some() && call(vcs().as_str(), "pull") {
        return ExitCode::SUCCESS;
    } else if app.subcommand_matches("status").is_some() && call(vcs().as_str(), "status") {
        return ExitCode::SUCCESS;
    } else if app.subcommand_matches("log").is_some() && call(vcs().as_str(), "log") {
        return ExitCode::SUCCESS;
    } else {
        ExitCode::FAILURE
    }
}
