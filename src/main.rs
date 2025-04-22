use std::io::{Error, ErrorKind};
use std::process::Command;

#[doc = "hooks"]
const WAKEUP: [(&str, &str, &str); 4] = [
    (
        "checking format",
        "fmt --check",
        "source code respect code format standard",
    ),
    (
        "running source code tests",
        "test --no-fail-fast",
        "tests passes",
    ),
    ("auditing source code", "audit", "source code secure"),
    (
        "checking source code",
        "clippy -- -D clippy::all -D clippy::cargo",
        "source code successful",
    ),
];
#[doc = "commit titles"]
const COMMIT: [&str; 4] = [
    "enter a summary",
    "explain the reason to changes",
    "impact of changes",
    "resolve issues",
];

fn ok(out: &str) {
    println!("\x1b[1;32m    Finished\x1b[0m {out}");
}
fn ko(out: &str) {
    println!("\x1b[1;32m    Stopping\x1b[0m {out}");
}
fn confirm(q: &str) -> Result<bool, Error> {
    let mut x: String = String::new();
    while x.trim().is_empty() {
        ok(format!("{q}  y n ?").as_str());
        match std::io::stdin().read_line(&mut x) {
            Err(_) => {
                x.clear();
            }
            _ => break,
        }
    }
    Ok(x.contains("y"))
}
fn main() -> Result<(), Error> {
    if confirm("Do you want to run breath")?.eq(&false) {
        ok("commit aborted");
        return Ok(());
    }
    for (title, args, success) in WAKEUP {
        ok(title);
        if Command::new("cargo")
            .args(args.split_whitespace())
            .current_dir(".")
            .spawn()?
            .wait()?
            .success()
        {
            ok(success);
        } else {
            ko("warning detected");
            return Err(Error::new(ErrorKind::Other, "Command failed"));
        }
    }
    if confirm("show diff")?.eq(&true) {
        Command::new("git")
            .args(["diff", "-p"])
            .current_dir(".")
            .spawn()?
            .wait()?;
    }
    if confirm("add src to git")?.eq(&true) {
        Command::new("git")
            .args(["add", "."])
            .current_dir(".")
            .spawn()?
            .wait()?;
    } else {
        ko("source code must be added");
        return Err(Error::new(ErrorKind::Other, "Command failed"));
    }
    let mut commit = String::new();
    let mut x = String::from('\n');
    for title in COMMIT {
        while x.is_empty() {
            ok(title);
            std::io::stdin().read_line(&mut x)?;
            if x.trim().is_empty() {
                x.clear();
                continue;
            }
        }
        commit.push('\n');
        commit.push_str(x.as_str());
        commit.push('\n');

        x.clear()
    }
    if Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit.as_str())
        .current_dir(".")
        .spawn()
        .expect("Commit failed")
        .wait()
        .expect("Commit failed")
        .success()
    {
        ok("commited successfully");
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Interrupted, "Commit failed"))
    }
}
