use crossterm::execute;
use spinners::{Spinner, Spinners};
use std::fs::{File, create_dir_all, read_to_string};
use std::process::Command;
use std::thread::sleep;

/// Executes a command and provides real-time visual feedback while processing.
///
/// This function displays a spinner animation while a command is executed. If the command
/// succeeds, the spinner stops with a success message. If the command fails, the spinner
/// stops with the error output displayed and an error is returned.
///
/// # Arguments
///
/// * `message` - A message to be displayed alongside the spinner while the command runs.
/// * `cmd` - A mutable reference to a `Command` object that specifies the process to be executed.
/// * `success` - A string message displayed when the command executes successfully.
/// * `failure` - A string error message returned when the command fails.
/// * `file` - A file name used to store the command's stdout and stderr logs under the `.breathes` directory.
///
/// # Returns
///
/// Returns `Ok(())` if the command executes successfully, or `Err(std::io::Error)` if the command fails.
///
/// # Side Effects
///
/// - The function creates two files in the `.breathes` directory:
///   - `stdout/{file}`: Stores the standard output of the command.
///   - `stderr/{file}`: Stores the standard error of the command.
/// - If these files cannot be created or the command cannot be executed, the function panics with an appropriate error message.
///
/// # Errors
///
/// - Returns `Err(std::io::Error)` with the failure message if the command exits unsuccessfully.
/// - Returns `Err(std::io::Error)` if there is an issue reading the command's stderr output.
///
/// # Examples
///
/// ```no_run
/// use std::process::Command;
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     let mut cmd = Command::new("ls");
///     ok(
///         "Executing command...",
///         &mut cmd,
///         "Command executed successfully!",
///         "Command execution failed.",
///         "output.log",
///     )
/// }
/// ```
pub fn ok(
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

/// Verifies the quality, formatting, and security of a Rust project through a series of checks.
///
/// This function performs the following tasks sequentially:
/// 1. Clears the terminal screen for better visibility of the process.
/// 2. Creates the necessary directories (`.breathes`, `.breathes/stdout`, `.breathes/stderr`) to
///    store output logs and other relevant data.
/// 3. Checks if the source code compiles without warnings using `cargo check`.
/// 4. Verifies adherence to formatting standards using `cargo fmt --check`.
/// 5. Runs all unit tests via `cargo test --no-fail-fast`.
/// 6. Ensures code quality without warnings using `cargo clippy`.
/// 7. Generates project documentation with `cargo doc --no-deps`.
/// 8. Audits the project for vulnerabilities using `cargo audit`.
///
/// For each task, the function attempts to log outputs to files (e.g., `check.log`, `test.log`,
/// `clippy.log`, etc.) while printing errors and warnings directly to the terminal if they occur.
///
/// ### Returns
///
/// - `true` if all checks pass successfully.
/// - `false` if any of the checks fail, along with an error message printed to `stderr`.
///
/// ### Panics
///
/// - If there is a failure while clearing the terminal screen.
/// - If the `.breathes` directory or its subdirectories cannot be created.
///
/// ### Examples
///
/// ```
/// let result = verify();
/// if result {
///     println!("All checks passed!");
/// } else {
///     eprintln!("Some checks failed.");
/// }
/// ```
///
/// ### Dependencies
///
/// - `crossterm` for terminal manipulation.
/// - `cargo` commands for project verification.
/// - Logs are written to the `.breathes ` directory for each respective check.
pub fn verify() -> bool {
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
        eprintln!(">>> Cargo check detect warning");
        return false;
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
        eprintln!(">>> Cargo fmt detect warning");
        return false;
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
        eprintln!(">>> Cargo test detect failures");
        return false;
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
        eprintln!(">>> Cargo clippy detect warning");
        return false;
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
        eprintln!(">>> Cargo doc detect warning");
        return false;
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
        eprintln!(">>> cargo audit detect warning");
        return false;
    }
    true
}
pub const COMMIT_MESSAGE: &str = r"%type%(%s%): %summary%";
