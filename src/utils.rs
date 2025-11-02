use crate::commit::{Commit, run_commit, vcs};
use crate::hooks::Language::{CSharp, D};
use crate::hooks::{Hook, LANGUAGES, Language};
use crate::issues::get_issues;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use glob::glob;
use inquire::validator::{StringValidator, Validation};
use inquire::{Confirm, CustomUserError, InquireError, Select};
use regex::Regex;
use spinners::{Spinner, Spinners};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::{File, create_dir_all, read_to_string};
use std::io::{Error, stdout};
use std::path::{MAIN_SEPARATOR_STR, Path};
use std::process::{Command, exit};
use std::time::Instant;
use tabled::settings::Style;

pub const OK: i32 = 7;
pub const KO: i32 = 8;
pub const QUIT: i32 = 0;

#[derive(Clone)]
pub struct EmailValidator;

impl StringValidator for EmailValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        if Regex::new(r"^[a-zA-Z0-9._+-]+@([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}$")?.is_match(input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("No a valid email".into()))
        }
    }
}
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
pub fn ok(message: &str, cmd: &mut Command, success: &str, failure: &str) -> Result<(), Error> {
    let mut output = Spinner::new(Spinners::Line, message.white().to_string());
    let status = cmd.current_dir(".").spawn()?.wait()?.code();
    if let Some(response) = status
        && response.eq(&0)
    {
        output.stop_and_persist(
            "✓".green().to_string().as_str(),
            success.dark_cyan().to_string(),
        );
        Ok(())
    } else {
        output.stop_and_persist("!".red().to_string().as_str(), failure.yellow().to_string());
        Err(Error::other(failure))
    }
}
///
/// Call git or mercurial with arg
///
/// # Errors
///
/// On failure to execute the command
///
pub fn call(program: &str, arg: &str) -> Result<i32, Error> {
    let to = if arg.is_empty() {
        program.to_string()
    } else {
        format!("{program} {arg}")
    };
    if !Command::new("sh")
        .arg("-c")
        .arg(to.as_str())
        .current_dir(".")
        .spawn()?
        .wait()?
        .success()
    {
        return Err(Error::other(format!("{program} not founded")));
    }
    Ok(OK)
}

/// # Panics
/// if failed to parse breathes.toml
#[must_use]
pub fn types() -> Vec<String> {
    let conf: crate::commit::Config = toml::from_str(
        read_to_string("breathes.toml")
            .expect("failed to parse breathes.toml")
            .as_str(),
    )
    .expect("bad breathes.toml");
    let mut types = conf.types;
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
/// # Errors
/// - If there is an issue reading the command's stderr output.
/// - If there is an issue executing a command.
/// - If there is an issue creating a file.
/// - If there is an issue clearing the terminal screen.
/// - If there is an issue writing to a file.
/// # Dependencies
///
/// - `crossterm` for terminal manipulation.
/// - `cargo` commands for project verification.
/// - Logs are written to the `breathes ` directory for each respective check
///
pub fn verify(hooks: &[Hook]) -> Result<(bool, u64), Error> {
    let start = Instant::now();
    let mut status: Vec<bool> = Vec::new();
    create_dir_all("breathes")?;
    for hook in hooks {
        create_dir_all(format!("breathes{MAIN_SEPARATOR_STR}{}", hook.language))?;
        create_dir_all(format!(
            "breathes{MAIN_SEPARATOR_STR}{}/stdout",
            hook.language
        ))?;
        create_dir_all(format!(
            "breathes{MAIN_SEPARATOR_STR}{}/stderr",
            hook.language
        ))?;

        if ok(
            hook.description,
            Command::new("sh").arg("-c")
                .arg(hook.command)
                .current_dir(".")
                .stderr(
                    File::create(format!("breathes{MAIN_SEPARATOR_STR}{}{MAIN_SEPARATOR_STR}stderr{MAIN_SEPARATOR_STR}{}", hook.language, hook.file))?
                )
                .stdout(
                    File::create(format!("breathes{MAIN_SEPARATOR_STR}{}{MAIN_SEPARATOR_STR}stdout{MAIN_SEPARATOR_STR}{}", hook.language, hook.file))?
                ),
            hook.success,
            hook.failure,
        )
            .is_err()
        {
            status.push(false);
        } else {
            status.push(true);
        }
    }
    Ok((
        status.contains(&false).eq(&false),
        start.elapsed().as_secs(),
    ))
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
/// In all cases, if a required hook verification or command fails, an error of a type
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
pub fn run_hooks() -> Result<i32, Error> {
    let start = Instant::now();
    let mut all: HashMap<String, (bool, u64)> = HashMap::new();
    let mut table = tabled::builder::Builder::default();
    let mut response: Vec<bool> = Vec::new();
    let l = detect();
    if l.is_empty() {
        return Err(Error::other("No language detected"));
    }
    for lang in &l {
        if run_hook(*lang, &mut all).is_err() {
            return Err(Error::other("Failed to run hook"));
        }
    }
    table.push_record(["Language", "Status", "Take"]);
    for (language, &status) in &all {
        response.push(status.0);
        if status.0 {
            table.push_record([
                language.clone(),
                "Success".to_string(),
                format!("{}s", status.1),
            ]);
        } else {
            table.push_record([
                language.clone(),
                "Failure".to_string(),
                format!("{}s", status.1),
            ]);
        }
    }
    if response.contains(&false) {
        table.push_record([
            "All".to_string(),
            String::from("Failure"),
            format!("{}s", start.elapsed().as_secs()),
        ]);
    } else {
        table.push_record([
            "All".to_string(),
            String::from("Success"),
            format!("{}s", start.elapsed().as_secs()),
        ]);
    }
    let mut report = table.build();
    println!("{}", report.with(Style::modern_rounded()));
    if response.contains(&false) {
        return Err(Error::other("Some checks failed."));
    }
    Ok(0)
}

impl Display for ZenOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quit => write!(f, "Quit"),
            Self::Add => write!(f, "Add"),
            Self::Health => write!(f, "Health"),
            Self::Log => write!(f, "Log"),
            Self::Diff => write!(f, "Diff"),
            Self::Email => write!(f, "Email"),
            Self::ListTags => write!(f, "List Tags"),
            Self::Status => write!(f, "Status"),
            Self::Edit => write!(f, "Editor"),
            Self::Commit => write!(f, "Commit"),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Hash, Copy)]
pub enum ZenOption {
    Quit,
    Add,
    Edit,
    Commit,
    Health,
    Log,
    Diff,
    Email,
    Status,
    ListTags,
}
impl ZenOption {
    #[must_use]
    pub fn all() -> Vec<Self> {
        let mut x = vec![
            Self::Quit,
            Self::Add,
            Self::Edit,
            Self::Commit,
            Self::Health,
            Self::Log,
            Self::Diff,
            Self::Email,
            Self::Status,
            Self::ListTags,
        ];
        x.sort();
        x
    }
}
impl From<&str> for ZenOption {
    fn from(value: &str) -> Self {
        match value {
            "Add" => Self::Add,
            "Health" => Self::Health,
            "Log" => Self::Log,
            "Diff" => Self::Diff,
            "Email" => Self::Email,
            "ListTags" => Self::ListTags,
            "Status" => Self::Status,
            "Editor" => Self::Edit,
            "Commit" => Self::Commit,
            _ => Self::Quit,
        }
    }
}

///
/// Interact with the developer.
///
/// # Panics
/// On bad input
/// # Errors
/// - If there is an issue reading the command's stderr output.
/// - If there is an issue executing a command.
/// - If there is an issue creating a file.
/// - If there is an issue clearing the terminal screen.
/// - If there is an issue writing to a file.
/// # Dependencies
/// - `crossterm` for terminal manipulation.
/// - `git` for version control.
/// - `git-commit-template` for commit templates.
///
pub async fn zen() -> Result<i32, InquireError> {
    loop {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        let option = Select::new(
            "@wishes:".green().bold().to_string().as_str(),
            ZenOption::all(),
        )
        .prompt()?;
        let response: Result<i32, Error> = match option {
            ZenOption::Quit => {
                execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
                exit(QUIT);
            }
            ZenOption::Add => call(vcs().as_str(), "add ."),
            ZenOption::Health => run_hooks(),
            ZenOption::Log => call(vcs().as_str(), "log"),
            ZenOption::Status => call(vcs().as_str(), "status"),
            ZenOption::Diff => call(vcs().as_str(), "diff"),
            ZenOption::Email => call("aerc", ""),
            ZenOption::ListTags => call(vcs().as_str(), "tag"),
            ZenOption::Edit => call("broot", "."),
            ZenOption::Commit => {
                if run_hooks().is_ok()
                    && let Ok(c) =
                        Commit::default().commit(&get_issues().await.expect("failed to get issues"))
                    && run_commit(c).is_ok()
                {
                    call(vcs().as_str(), "push")
                } else {
                    continue;
                }
            }
        };
        if response.is_err() && Confirm::new("Exit").with_default(false).prompt()?
            || response.is_ok() && Confirm::new("Exit").with_default(false).prompt()?
        {
            exit(QUIT);
        }
    }
}
fn run_hook(lang: Language, all: &mut HashMap<String, (bool, u64)>) -> Result<(), Error> {
    let hooks = Hook::get(lang);
    all.insert(lang.to_string(), verify(&hooks)?);
    Ok(())
}
fn add_if_exists(file: &str, language: Language, vec: &mut Vec<Language>) {
    if language == CSharp
        && let Ok(files) = glob(file)
    {
        for file in files {
            if let Ok(file) = file
                && file.is_file()
            {
                vec.push(language);
            }
        }
    } else if language == D
        && let Ok(files) = glob(file)
    {
        for file in files {
            if let Ok(file) = file
                && file.is_file()
            {
                vec.push(language);
            }
        }
    } else if Path::new(file).is_file() {
        vec.push(language);
    }
}
#[must_use]
pub fn detect() -> Vec<Language> {
    let mut all: Vec<Language> = Vec::new();
    for (l, file) in &LANGUAGES {
        add_if_exists(file, *l, &mut all);
    }
    all
}
