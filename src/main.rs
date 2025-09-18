use crossterm::execute;
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
    println!();
    ExitCode::SUCCESS
}
