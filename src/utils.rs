use crate::commit::{Commit, run_commit, vcs};
use breathes::hooks::run_hooks;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use inquire::validator::{StringValidator, Validation};
use inquire::{Confirm, CustomUserError, InquireError, Select};
use regex::Regex;
use spinners::{Spinner, Spinners};
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::io::{Error, stdout};
use std::process::{Command, exit};

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
pub fn zen() -> Result<i32, InquireError> {
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
                    && let Ok(c) = Commit::default().commit()
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
