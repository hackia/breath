pub mod git;

pub mod hg;

mod commit;
pub mod hooks;
pub mod utils;

use crate::utils::{configure_git, configure_hg, run_hooks};
use clap::Arg;
use std::path::Path;
use std::process::{Command, ExitCode};

fn call(program: &str, arg: &str) -> bool {
    Command::new(program)
        .arg(arg)
        .current_dir(".")
        .spawn()
        .expect("Fail to execute command")
        .wait()
        .expect("Fail to execute command")
        .success()
}
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
        .subcommand(clap::Command::new("log").about("Show the commit log"))
        .subcommand(
            clap::Command::new("diff")
                .about("Show the changes between the working directory and the index"),
        )
        .get_matches()
}
fn main() -> ExitCode {
    let mercurial = Path::new(".hg").is_dir();
    let git = Path::new(".git").is_dir();
    let app = breathes();

    match app.subcommand() {
        Some(("config", config)) => {
            let vcs = config.get_one::<String>("vcs").unwrap();
            match vcs.as_str() {
                "git" if configure_git() => ExitCode::SUCCESS,
                "hg" if configure_hg() => ExitCode::SUCCESS,
                _ => ExitCode::FAILURE,
            }
        }
        Some(("health", _)) => match run_hooks() {
            Ok(()) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        },
        Some(("commit", _)) => match (mercurial, git) {
            (true, _) => hg::run(),
            (_, true) => git::run(),
            _ => ExitCode::FAILURE,
        },
        Some((cmd @ ("push" | "pull" | "status" | "log" | "diff"), _)) => match (mercurial, git) {
            (true, _) if call("hg", cmd) => ExitCode::SUCCESS,
            (_, true) if call("git", cmd) => ExitCode::SUCCESS,
            _ => ExitCode::FAILURE,
        },
        _ => ExitCode::FAILURE,
    }
}
