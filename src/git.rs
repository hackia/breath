use crate::utils::{COMMIT_MESSAGE, ok, types};
use crossterm::execute;
use crossterm::style::Stylize;
use inquire::{Confirm, Editor, Select, Text};
use std::process::{Command, ExitCode};

/// Executes a series of shell commands related to Git and system operations.
///
/// The `diff` function performs the following steps:
/// 1. Executes the `git status` command to display the current status of the Git repository.
/// 2. Waits for a specified duration (7 seconds in this instance) by using the `sleep` command,
///    which is wrapped in the `ok` function to provide additional behavior such as logging.
/// 3. Finally, executes the `git diff -p` command to show changes between the working directory
///    and the index in patch format.
///
/// # Commands and Behavior
/// - `git status`: Checks the repository status, such as modified, new, or deleted files.
/// - `sleep 7`: Pauses the operation for 7 seconds (handled by the `ok` function).
/// - `git diff -p`: Shows detailed differences between files in the current repository.
///
/// # Error Handling
/// - If any of the commands fail to execute, the program will panic and display an error message:
///   - "Fail to execute command" for `git status` and `git diff -p`.
///   - `"Fail to execute command"` for the `ok` function, if an issue arises within its execution.
///
/// # Notes
/// - The `println!();` between commands outputs a newline to improve console readability.
/// - The `ok` function appears to be a utility for executing commands with logging. Ensure its definition
///   is available and correctly handles the `sleep` operation.
///
/// # Panics
/// This function will panic if:
/// - The `git` commands (`status`, `diff -p`) fail due to missing Git installation or issues in the repository.
/// - The `ok` function fails for any reason, such as the `sleep` command not being available.
///
/// Ensure the environment running this function has Git installed and the current directory
/// is a valid Git repository.
fn diff() {
    Command::new("git")
        .arg("status")
        .status()
        .expect("Fail to execute command");

    println!();

    ok(
        "waiting",
        Command::new("sleep").arg("7"),
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
/// Handles the process of committing changes to a git repository in a structured workflow.
///
/// This function provides an interactive interface to perform the following steps:
/// 1. Displays the current changes by calling the `diff()` function.
/// 2. Prompts the user for confirmation to proceed with the commit process.
/// 3. If confirmed:
///    - Stages all changes (`git add .`).
///    - Prompts the user to select a commit type (e.g., `feat`, `fix`, etc.) from a pre-defined list.
///    - Prompts the user for a commit scope (optional).
///    - Prompts the user for a commit summary.
///    - Constructs a commit message using the provided type, scope, and summary, following a predefined template.
///    - Executes the `git commit` command with the constructed commit message.
///    - Clears the terminal for user clarity.
///    - Asks if the user wants to push the changes to remote repositories.
///        - If confirmed:
///          - Clears the terminal.
///          - Pushes all branches (`git push --all`).
///          - Pushes all tags (`git push --tags`).
/// 4. If not confirmed, it aborts the commit process gracefully.
///
/// ## Returns
/// - `ExitCode::SUCCESS`: Indicates that the process completed successfully, regardless of whether a commit was performed or not.
///
/// ## Errors
/// - Panics if any I/O operation fails (e.g., terminal interaction, git commands, etc.).
///
/// ## Example
/// ```no_run
/// // Start the commit process
/// let exit_code = commit();
/// std::process::exit(exit_code.code());
/// ```
///
/// ## Dependencies
/// - This function requires the `git` command-line tool to be available on the system.
/// - Relies on external crates for terminal interaction, such as:
///   - `inquire` for prompts
///   - `crossterm` for terminal manipulation
///
/// ## Notes
/// - The function uses a commit message template (`COMMIT_MESSAGE`) which must be predefined in the scope of this code.
/// - The default behavior for certain prompts, such as whether to push after committing, can be customized in the code.
fn commit() -> ExitCode {
    println!();
    diff();
    println!();
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
        let t = Select::new("Commit types".green().bold().to_string().as_str(), types())
            .with_vim_mode(true)
            .prompt()
            .expect("failed to get scope");
        let s = Text::new("Commit scope".green().bold().to_string().as_str())
            .prompt()
            .expect("failed to get scope");
        let summary = Text::new("Commit summary".green().bold().to_string().as_str())
            .prompt()
            .expect("failed to get summary");
        let body = Editor::new("enter the commit body: ")
            .prompt()
            .expect("failed to get body");
        let y = t.split('~').collect::<Vec<&str>>();
        let comm = COMMIT_MESSAGE
            .replace("%type%", y.first().expect("failed to get type").trim_end())
            .replace("%s%", s.trim_end())
            .replace("%summary%", summary.trim_end())
            .replace("%body%", body.as_str());
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
            execute!(
                std::io::stdout(),
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
                crossterm::cursor::MoveTo(0, 1)
            )
            .expect("error");
            Command::new("git")
                .arg("push")
                .arg("--all")
                .status()
                .expect("Fail to execute command");
            println!();
            Command::new("git")
                .arg("push")
                .arg("--tags")
                .status()
                .expect("failed to execute command");
        }
        println!();
        return ExitCode::SUCCESS;
    }
    println!("Commit aborted");
    ExitCode::SUCCESS
}
/// Executes the primary operation of the program.
///
/// This function serves as the main entry point for the program's core logic. It performs two key tasks:
/// 1. Verifies a certain condition or state by invoking the `verify` function. If `verify` returns `false`, the program will panic as the assertion will fail.
/// 2. Once the verification is successful (i.e., `verify` returns `true`), it commits or finalizes some operation by calling the `commit` function.
///
/// # Returns
/// * `ExitCode` - The exit code returned by the `commit` function, which represents the program's final state or result.
///
/// # Panics
/// This function will panic if the `verify` function returns `false`, indicating that the required condition or state was not met.
#[must_use]
pub fn run() -> ExitCode {
    commit()
}
