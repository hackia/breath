use crate::hooks::{Hooks, NODE_HOOKS, RUST_HOOKS};
use crossterm::execute;
use spinners::{Spinner, Spinners};
use std::fs::{File, create_dir_all, read_to_string};
use std::path::{MAIN_SEPARATOR_STR, Path};
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
        .current_dir(".")
        .spawn()
        .expect("Fail to spawn thread")
        .wait()
        .expect("fail to wait for thread")
        .code()
        .expect("fail to get exit code");
    sleep(std::time::Duration::from_millis(250));
    if status.eq(&0) {
        output.stop_and_persist("*", success.to_string());
        Ok(())
    } else {
        output.stop_and_persist("!", read_to_string(format!(".breathes/stderr/{file}"))?);
        Err(std::io::Error::other(failure))
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
pub fn verify(hooks: Vec<Hooks>) -> bool {
    execute!(
        std::io::stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 1),
    )
    .expect("Fail to clear terminal");
    create_dir_all(".breathes").expect("Fail to create .breathes directory");
    for hook in &hooks {
        create_dir_all(format!(".breathes{MAIN_SEPARATOR_STR}{}", hook.language))
            .expect("Fail to create .breathes/out_dir directory");
        create_dir_all(format!(
            ".breathes{MAIN_SEPARATOR_STR}{}{MAIN_SEPARATOR_STR}stdout",
            hook.language
        ))
        .expect("Fail to create .breathes/out_dir directory");
        create_dir_all(format!(
            ".breathes{MAIN_SEPARATOR_STR}{}{MAIN_SEPARATOR_STR}stderr",
            hook.language
        ))
        .expect("Fail to create .breathes/out_dir directory");

        let program = match hook.language {
            crate::hooks::Language::Node => "npm",
            crate::hooks::Language::Rust => "cargo",
        };

        if ok(
            hook.description,
            Command::new(program)
                .args(hook.command.split_whitespace())
                .current_dir(".")
                .stderr(
                    File::create(format!(".breathes{MAIN_SEPARATOR_STR}{}{MAIN_SEPARATOR_STR}stderr{MAIN_SEPARATOR_STR}{}", hook.language, hook.file))
                        .expect("Failed to create file"),
                )
                .stdout(
                    File::create(format!(".breathes{MAIN_SEPARATOR_STR}{}{MAIN_SEPARATOR_STR}stdout{MAIN_SEPARATOR_STR}{}", hook.language, hook.file))
                        .expect("Failed to create file"),
                ),
            hook.success,
            hook.failure,
            hook.file,
        )
            .is_err()
        {

            let one = read_to_string(format!(".breathes{MAIN_SEPARATOR_STR}{}{MAIN_SEPARATOR_STR}stdout{MAIN_SEPARATOR_STR}{}", hook.language, hook.file));
            let two = read_to_string(format!(".breathes{MAIN_SEPARATOR_STR}{}{MAIN_SEPARATOR_STR}stderr{MAIN_SEPARATOR_STR}{}", hook.language, hook.file));
            eprintln!("\n{}\n{}\n\n", one.expect("Fail to read file"),two.expect("Fail to read file"));
            return false;
        };
    }
    true
}

pub fn run_hooks() -> Result<(), std::io::Error> {
    if Path::new("Cargo.toml").exists() && verify(RUST_HOOKS.to_vec()).eq(&false) {
        return Err(std::io::Error::other("Some checks failed"));
    }
    if Path::new("package.json").exists() && verify(NODE_HOOKS.to_vec()).eq(&false) {
        return Err(std::io::Error::other("Some checks failed"));
    }
    Ok(())
}
pub const COMMIT_MESSAGE: &str = r"%type%(%s%): %summary%";
