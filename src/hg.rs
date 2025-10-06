use crate::utils::{COMMIT_MESSAGE, ok, types, verify};
use crossterm::execute;
use inquire::{Confirm, Editor, Select, Text};
use std::process::{Command, ExitCode};

/// Executes a series of system commands to display version control status,
/// introduce a delay, and then display file differences.
///
/// This function performs the following steps:
/// 1. Executes the "hg status" command using the Mercurial version control tool
///    to display the status of changes in the repository.
/// 2. Prints a blank line for output formatting.
/// 3. Calls the `ok` function with a custom delay of 7 seconds. The `ok` function
///    appears to execute a command and logs the output to "waiting.log".
/// 4. Executes the "hg diff -p" command to display a patch-like view of differences
///    between files.
///
/// # Commands Used
/// - `hg status`: Displays the working directory status for Mercurial repositories.
/// - `sleep 7`: Pauses the execution for 7 seconds (introduced through the `ok` function).
/// - `hg diff -p`: Displays the differences between changes in a patch format.
///
/// # Panics
/// - If any of the commands fail to execute, the program will panic with an error
///   message:
///   - "Fail to execute command" is displayed for both `hg` commands.
///   - Failures in the `ok` function call propagate with a similar error message.
///
/// # Note
/// - Ensure that the `hg` (Mercurial) command-line tool is installed and available in
///   the system's PATH.
/// - The `ok` function is user-defined, and its behavior depends on its implementation.
///   Make sure it is properly implemented and available in the scope.
///
/// # Example
/// ```ignore
/// diff(); // This will run the commands as described above.
/// ```
fn diff() {
    Command::new("hg")
        .arg("status")
        .status()
        .expect("Fail to execute command");

    println!();

    ok(
        "waiting",
        &mut Command::new("sleep").arg("7"),
        "waiting",
        "waiting",
        "waiting.log",
    )
    .expect("Fail to execute command");
    Command::new("hg")
        .arg("diff")
        .arg("-p")
        .status()
        .expect("Fail to execute command");
}
/// This function interacts with the user to commit changes to a Mercurial (hg) repository.
/// It guides the user through confirming the commit, adding files, composing the commit message,
/// and optionally pushing the changes to the remote repository.
///
/// # Flow
///
/// 1. Executes a `diff` to show the changes to be committed.
/// 2. Prompts the user to confirm whether they want to commit the changes.
///    - If the user chooses to abort, the process exits successfully without committing.
/// 3. Adds all changes in the current working directory to the staging area.
/// 4. Prompts the user to select a commit type from predefined categories.
/// 5. Asks the user for a commit scope (context) and a summary of the changes.
/// 6. Constructs a commit message using a predefined template and commits the changes using `hg commit`.
/// 7. Clears the terminal to provide a clean visual output for subsequent actions.
/// 8. Optionally prompts the user for confirmation to push the committed changes to the remote repository:
///    - If confirmed, pushes the changes using `hg push`.
///
/// # Return
///
/// - Returns [`ExitCode::SUCCESS`](std::process::ExitCode) upon successful completion
///   or if the user opts to abort before committing.
///
/// # Errors
///
/// - Panics if any of the following operations fail:
///   - Fetching user input via `Confirm`, `Select`, or `Text`.
///   - Executing system commands like `hg add`, `hg commit`, or `hg push`.
///   - Clearing the terminal output.
///
/// # Dependencies
///
/// - `hg` (Mercurial): Must be available in the system's PATH and should be properly configured.
/// - Requires external crates:
///   * `dialoguer`: To handle user prompts.
///   * `crossterm`: To handle terminal clear and cursor movement.
///
/// # Example
///
/// ```no_run
/// use std::process::ExitCode;
///
/// fn main() -> ExitCode {
///     commit()
/// }
///
/// // Running `main` will start the interactive commit process.
/// ```
fn commit() -> ExitCode {
    println!();
    diff();
    println!();
    if Confirm::new("Do you want commit this code ?")
        .with_default(false)
        .prompt()
        .expect("failed")
    {
        Command::new("hg")
            .arg("add")
            .arg(".")
            .status()
            .expect("Fail to execute command");
        let t = Select::new("Commit types", types())
            .prompt()
            .expect("failed to get scope");
        let s = Text::new("Commit scope")
            .prompt()
            .expect("failed to get scope");
        let summary = Text::new("Commit summary")
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
        Command::new("hg")
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
            Command::new("hg")
                .arg("push")
                .status()
                .expect("Fail to execute command");
        }
        println!();
        return ExitCode::SUCCESS;
    }
    println!("Commit aborted");
    ExitCode::SUCCESS
}
/// The `run` function is the entry point for executing a sequence of operations
/// involved in the application's runtime execution.
///
/// # Behavior
///
/// - This function starts by verifying certain conditions or prerequisites necessary
///   for the subsequent operations to proceed by calling the `verify` function.
///   If `verify` does not return `true`, the program will panic due to `assert!`.
/// - If the verification succeeds, the function proceeds to call the `commit` function,
///   which is expected to handle committing changes or completing the required task.
///
/// # Returns
///
/// - The function returns an `ExitCode` representing the application's exit status.
///   - Successful completion: Typically indicates a proper and graceful exit.
///   - If `verify` fails, the program will panic, and no `ExitCode` is returned in that case.
///
/// # Panics
///
/// - This function panics if `verify` does not return `true`.
///
/// # Examples
///
/// ```rust
/// use std::process::ExitCode;
///
/// fn verify() -> bool { true }
/// fn commit() -> ExitCode { ExitCode::SUCCESS }
///
/// fn main() {
///     let exit_code = run();
///     std::process::exit(exit_code.code().unwrap_or(1)); // Handles exit
/// }
///
/// pub fn run() -> ExitCode {
///     assert!(verify());
///     commit()
/// }
/// ```
///
/// # Note
///
/// The behavior of this function heavily relies on the implementation of both
/// the `verify` and `commit` functions, which are expected to have side effects
/// or perform critical operations.
///
/// # Dependencies
///
/// - `verify`: Function must validate necessary conditions, returning a boolean.
/// - `commit`: Function must perform the final operation and return an `ExitCode`.
#[must_use]
pub fn run() -> ExitCode {
    commit()
}
