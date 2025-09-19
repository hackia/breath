use crossterm::execute;
use inquire::{Confirm, Select, Text};
use spinners::{Spinner, Spinners};
use std::fs::{File, create_dir_all, read_to_string};
use std::process::{Command, ExitCode};
use std::thread::sleep;

fn ok(
    message: &str,
    cmd: &mut Command,
    success: &str,
    failure: &str,
    file: &str,
) -> std::io::Result<()> {
    let mut output = Spinner::new(Spinners::Dots2, message.to_string());
    let status = cmd
        .stderr(File::create(format!(".breathes/stderr/{file}")).expect("Fail to create file"))
        .stdout(File::create(format!(".breathes/stdout/{file}")).expect("Fail to create file"))
        .status()
        .expect("Fail to execute command");
    sleep(std::time::Duration::from_millis(250));
    if status.success() {
        output.stop_and_persist("*", success.to_string());
        Ok(())
    } else {
        output.stop_and_persist("!", read_to_string(format!(".breathes/stderr/{file}"))?);
        Err(std::io::Error::new(std::io::ErrorKind::Other, failure))
    }
}

const COMMIT_MESSAGE: &str = r"%type%(%s%): %summary%";

fn diff() {
    Command::new("git")
        .arg("status")
        .status()
        .expect("Fail to execute command");

    ok(
        "waiting",
        &mut Command::new("sleep").arg("7"),
        "waiting",
        "waiting",
        "waiting.log",
    )
    .expect("Fail to execute command");
    Command::new("git")
        .arg("diff")
        .arg("-p")
        .status()
        .expect("Fail to execute command");
}
fn commit() -> ExitCode {
    println!();
    diff();
    if Confirm::new("Do you want commit this code ?")
        .with_default(false)
        .prompt()
        .expect("failed")
    {
        Command::new("git")
            .arg("add")
            .arg(".")
            .status()
            .expect("Fail to execute command");
        let types = vec!["feat", "fix", "chore", "docs", "refactor", "style", "test"];
        let t = Select::new("Commit types", types.to_vec())
            .prompt()
            .expect("failed to get scope");
        let s = Text::new("Commit scope")
            .prompt()
            .expect("failed to get scope");
        let summary = Text::new("Commit summary")
            .prompt()
            .expect("failed to get summary");
        let comm = COMMIT_MESSAGE
            .replace("%type%", t)
            .replace("%s%", s.as_str())
            .replace("%summary%", summary.as_str());

        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(comm)
            .status()
            .expect("Fail to execute command");
        execute!(
            std::io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 1),
        )
        .expect("Fail to clear terminal");
        if Confirm::new("Do you want push to remotes?")
            .with_default(true)
            .prompt()
            .expect("failed to get confirm")
        {
            Command::new("git")
                .arg("push")
                .arg("--all")
                .status()
                .expect("Fail to execute command");
            Command::new("git")
                .arg("push")
                .arg("--tags")
                .status()
                .expect("failed to execute command");
        }
        println!();
        ExitCode::SUCCESS
    } else {
        println!("Abort commit");
        ExitCode::SUCCESS
    }
}
fn main() -> ExitCode {
    execute!(
        std::io::stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 1),
    )
    .expect("Fail to clear terminal");
    create_dir_all(".breathes").expect("Fail to create .breathes directory");
    create_dir_all(".breathes/stdout").expect("Fail to create .breathes/stdout directory");
    create_dir_all(".breathes/stderr").expect("Fail to create .breathes/stderr directory");
    if ok(
        "checking format",
        Command::new("cargo").arg("check").current_dir("."),
        "source code respect code format standard",
        "source code not respect code format standard",
        "check.log",
    )
    .is_err()
    {
        eprintln!("Cargo check detect warning");
        return ExitCode::FAILURE;
    };
    if ok(
        "checking code format",
        Command::new("cargo")
            .arg("fmt")
            .arg("--check")
            .current_dir("."),
        "source code respect code format standard",
        "source code not respect code format standard",
        "check.log",
    )
    .is_err()
    {
        eprintln!("Cargo fmt detect warning");
        return ExitCode::FAILURE;
    }
    if ok(
        "checking test",
        Command::new("cargo")
            .arg("test")
            .arg("--no-fail-fast")
            .current_dir("."),
        "test success",
        "test fail",
        "test.log",
    )
    .is_err()
    {
        eprintln!("Cargo test detect warning");
        return ExitCode::FAILURE;
    }

    if ok(
        "checking clippy",
        Command::new("cargo")
            .arg("clippy")
            .arg("--")
            .arg("-D")
            .arg("clippy:all")
            .current_dir("."),
        "No warning",
        "Detect warning",
        "clippy.log",
    )
    .is_err()
    {
        eprintln!("Cargo clippy detect warning");
        return ExitCode::FAILURE;
    }
    if ok(
        "generate documentation",
        Command::new("cargo")
            .arg("doc")
            .arg("--no-deps")
            .current_dir("."),
        "documentation generated",
        "documentation not generated",
        "doc.log",
    )
    .is_err()
    {
        return ExitCode::FAILURE;
    }
    if ok(
        "audit",
        Command::new("cargo").arg("audit"),
        "no vulnerabilities founded",
        "vulnerabilities",
        "audit.log",
    )
    .is_err()
    {
        eprintln!("Cargo audit detect warning");
        return ExitCode::FAILURE;
    }
    commit()
}
