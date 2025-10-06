use crate::commit::COMMIT_TYPES;
use crate::hooks::{CSHARP_HOOKS, GO_HOOKS, Hook, JAVA_HOOKS, NODE_HOOKS, PHP_HOOKS, RUST_HOOKS};
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
/// # Panics
/// if command is not founded and if the process cannot be spawned.
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

/// Returns a sorted list of formatted commit type strings.
///
/// This function operates on a predefined constant `COMMIT_TYPES`, which is assumed
/// to be a collection of commit type objects. Each object contains the following fields:
/// `type_name`, `description`, `category`, and `mnemonic`.
///
/// For each commit type object, the function generates a formatted string that concatenates
/// its fields (`type_name`, `description`, `category`, and `mnemonic`), separated by ` ~ `.
/// Additionally, any commas in the values of these fields are removed to ensure clean formatting.
///
/// The resulting list of formatted strings is sorted alphabetically before being returned.
///
/// # Returns
/// * `Vec<String>` - A sorted vector of formatted commit type strings.
///
/// # Example
/// Given the following `COMMIT_TYPES` structure:
/// ```rust
/// const COMMIT_TYPES: [CommitType; 2] = [
///     CommitType {
///         type_name: "feat",
///         description: "A new feature",
///         category: "Feature",
///         mnemonic: "F",
///     },
///     CommitType {
///         type_name: "fix",
///         description: "A bug fix",
///         category: "Bug Fix",
///         mnemonic: "B",
///     },
/// ];
/// ```
/// Calling the `types` function will return:
/// ```rust
/// vec![
///     "feat ~ A new feature ~ Feature ~ F",
///     "fix ~ A bug fix ~ Bug Fix ~ B",
/// ];
/// ```
#[must_use]
pub fn types() -> Vec<String> {
    let mut types = COMMIT_TYPES
        .iter()
        .map(|t| {
            format!(
                "{} ~ {} ~ {} ~ {}",
                t.type_name.to_string().replace(',', ""),
                t.description.to_string().replace(',', ""),
                t.category.to_string().replace(',', ""),
                t.mnemonic.to_string().replace(',', ""),
            )
        })
        .collect::<Vec<String>>();
    types.sort();
    types
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
/// ### Errors
/// - If there is an issue reading the command's stderr output.
/// - If there is an issue executing a command.
/// - If there is an issue creating a file.
/// - If there is an issue clearing the terminal screen.
/// - If there is an issue writing to a file.
/// ### Dependencies
///
/// - `crossterm` for terminal manipulation.
/// - `cargo` commands for project verification.
/// - Logs are written to the `.breathes ` directory for each respective check.
#[must_use]
pub fn verify(hooks: &[Hook]) -> bool {
    create_dir_all(".breathes").expect("Fail to create .breathes directory");
    for hook in hooks {
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
            crate::hooks::Language::Java => "mvn",
            crate::hooks::Language::Python => "python",
            crate::hooks::Language::Go => "go",
            crate::hooks::Language::Php => "php",
            crate::hooks::Language::Ruby => "ruby",
            crate::hooks::Language::CMake => "cmake",
            crate::hooks::Language::CSharp => "dotnet",
            crate::hooks::Language::Kotlin => "gradlew",
            crate::hooks::Language::Swift => "swift",
            crate::hooks::Language::Dart => "dart",
            crate::hooks::Language::Elixir => "elixir",
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
            eprintln!("\n{}\n{}\n\n", one.expect("Fail to read file"), two.expect("Fail to read file"));
            return false;
        }
    }
    true
}

/// Runs a set of predefined checks or hooks depending on the existence of certain project configuration files.
///
/// This function is designed to verify the presence of specific dependencies, configurations, or workflows
/// for common programming environments
/// Each environment has its respective hooks validated.
///
/// # Returns
///
/// * `Ok(())` if all checks pass successfully.
/// * `Err(std::io::Error)` if one or more checks fail.
///
/// # Hook Logic for Each Environment:
///
/// 1. `Rust`: If a `Cargo.toml` file exists, runs the `RUST_HOOKS` verification.
///    Returns an error if the checks fail.
///
/// 2. `Node.js`: If a `package.json` file exists, runs the `NODE_HOOKS` verification.
///    Returns an error if the checks fail.
///
/// 3. `PHP`: If a `composer.json` file exists, runs the `PHP_HOOKS` verification.
///    Returns an error if the checks fail.
///
/// 4. `Go`: If a `go.mod` file exists, runs the `GO_HOOKS` verification.
///    Returns an error if the checks fail.
///
/// 5. `C#`: If a `.csproj` file exists, runs the `CSHARP_HOOKS` verification.
///    Returns an error if the checks fail.
///
/// 6. `Java`: If a `build.gradle` file exists, runs the `JAVA_HOOKS` verification.
///    Returns an error if the checks fail.
///
/// 7. `CMake`: If a `CMakeLists.txt` file exists:
///     - Runs `cmake` to configure the project.
///     - Runs `make` to build the project.
///     - Runs `make test` to execute tests.
///
/// If any of these commands fail, an error is returned indicating that the `CMake`
///     configuration validation failed.
///
/// # Error Handling
/// In all cases, if a required hook verification or command fails, an error of type
/// `std::io::Error` with a custom message is returned to indicate which step or
/// environment failed.
///
/// # Examples
///
/// ```rust
/// match run_hooks() {
///     Ok(_) => println!("All hooks passed successfully."),
///     Err(err) => eprintln!("A hook check failed: {err}"),
/// }
/// ```
///
/// # Panics
/// This function may panic if external commands (like `cmake` or `make`) fail to spawn or
/// if their processes terminate unexpectedly.
/// # Errors
/// If some hooks failed
/// # Dependencies
/// - This function assumes that tools like `cmake` and `make` are installed and available
///   in the system's `PATH` if a CMake-based build system is being validated.
///
/// # Notes
/// - The `verify` function and various `*_HOOKS` constants are used internally for hook validation.
///   These must be defined appropriately outside the scope of this function.
/// - The function performs validation by matching file paths at the root of the project. Ensure
///   the function is executed in the appropriate working directory.
///
pub fn run_hooks() -> Result<(), std::io::Error> {
    if Path::new("Cargo.toml").exists() && verify(&RUST_HOOKS).eq(&false) {
        return Err(std::io::Error::other("Some checks failed"));
    }
    if Path::new("package.json").exists() && verify(&NODE_HOOKS).eq(&false) {
        return Err(std::io::Error::other("Some checks failed"));
    }
    if Path::new("composer.json").exists() && verify(&PHP_HOOKS).eq(&false) {
        return Err(std::io::Error::other("Some checks failed"));
    }
    if Path::new("go.mod").exists() && verify(&GO_HOOKS).eq(&false) {
        return Err(std::io::Error::other("Some checks failed"));
    }
    if Path::new(".csproj").exists() && verify(&CSHARP_HOOKS).eq(&false) {
        return Err(std::io::Error::other("Some checks failed"));
    }
    if Path::new("build.gradle").exists() && verify(&JAVA_HOOKS).eq(&false) {
        return Err(std::io::Error::other("Some checks failed"));
    }
    if Path::new("CMakeLists.txt").exists()
        && Command::new("cmake")
            .arg(".")
            .current_dir(".")
            .spawn()
            .expect("failed")
            .wait()
            .expect("failed")
            .success()
            .eq(&false)
        && Command::new("make")
            .current_dir(".")
            .spawn()
            .expect("failed")
            .wait()
            .expect("failed")
            .success()
            .eq(&false)
        && Command::new("make")
            .arg("test")
            .current_dir(".")
            .spawn()
            .expect("failed")
            .wait()
            .expect("failed")
            .success()
            .eq(&false)
    {
        return Err(std::io::Error::other(
            "Cmake configuration validation failed",
        ));
    }
    Ok(())
}
pub const COMMIT_MESSAGE: &str = r"%type%(%s%): %summary%\n\n%body%";
