pub mod commit;
pub mod utils;

mod config;
pub mod doc;
mod tree;

use crate::commit::{Commit, add, run_commit, vcs};
use crate::config::init_config;
use crate::doc::{generate_doc, generate_man};
use crate::utils::{call, zen};
use breathes::hooks::run_hooks;
use clap::{Arg, Command};
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::io::stdout;
use std::process::ExitCode;

fn breathes() -> Command {
    Command::new("breath")
        .version(env!("CARGO_PKG_VERSION"))
        .author("hackia <dev@hackia.org>")
        .about("A tool for managing Git and Mercurial repositories")
        .subcommand(Command::new("init").about("Initialize a new breath repository"))
        .subcommand(Command::new("add").about("Add files to the index"))
        .subcommand(
            Command::new("rm").about("Remove files from the working tree and from the index"),
        )
        .subcommand(Command::new("mv").about("Move or rename a file, a directory, or a symlink"))
        .subcommand(Command::new("reset").about("Reset current HEAD to the specified state"))
        .subcommand(Command::new("checkout").about("Switch branches or restore working tree files"))
        .subcommand(Command::new("clean").about("Remove untracked files from the working tree"))
        .subcommand(
            Command::new("feature")
                .about("create a feature branch from the current branch")
                .subcommand(Command::new("list").about("list all feature branches")),
        )
        .subcommand(
            Command::new("tag")
                .about("List, create, delete, or verify a tag object signed with GPG"),
        )
        .subcommand(Command::new("describe").about("Show information about commits"))
        .subcommand(
            Command::new("config").about("Manage configuration").arg(
                Arg::new("vcs")
                    .value_names(["git", "hg"])
                    .default_missing_value("git")
                    .default_value("git")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("gen")
                .about("Generators for documentation and manuals")
                .subcommand(Command::new("man").about("Generate man pages"))
                .subcommand(Command::new("doc").about("Generate documentation")),
        )
        .subcommand(Command::new("health").about("Verify repository health"))
        .subcommand(Command::new("commit").about("Commit changes to the repository"))
        .subcommand(Command::new("push").about("Push changes to remote repositories"))
        .subcommand(Command::new("pull").about("Pull changes from remote repositories"))
        .subcommand(Command::new("status").about("Show the status of the repository"))
        .subcommand(Command::new("zen").about("display a loop menu to interact with breath"))
        .subcommand(Command::new("log").about("Show the commit log"))
        .subcommand(
            Command::new("diff")
                .about("Show the changes between the working directory and the index"),
        )
}

#[must_use]
pub fn main() -> ExitCode {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).expect("failed to clear screen");
    let mut commit = Commit::new();
    let app = breathes();
    let matches = app.get_matches();
    match matches.subcommand() {
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
                ExitCode::FAILURE
            }
        }
        Some(("add", _)) => {
            if add().is_ok() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Some(("init", _)) => {
            if init_config().is_ok() {
                println!("breath initialized successfully");
                ExitCode::SUCCESS
            } else {
                eprintln!("failed to initialize breath");
                ExitCode::FAILURE
            }
        }
        Some((cmd @ ("push" | "pull" | "status" | "log" | "diff"), _)) => {
            if call(vcs().as_str(), cmd).is_ok() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Some(("gen", sub_matches)) => match sub_matches.subcommand() {
            Some(("man", _)) => generate_man(),
            Some(("doc", _)) => generate_doc(),
            _ => ExitCode::FAILURE,
        },
        _ => ExitCode::FAILURE,
    }
}
