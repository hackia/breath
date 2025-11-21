use crate::config::load_config;
use crate::tree::get_tree;
use crate::utils::types;
use breathes::hooks::{ok, run_hooks};
use inquire::error::InquireResult;
use inquire::{Confirm, Editor, InquireError, MultiSelect, Select, Text};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io::Error;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize)]
pub struct Config {
    pub scopes: Vec<String>,
    pub types: Vec<String>,
    pub repository: String,
    pub me: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[doc = "Represent a commit type"]
pub struct CommitType {
    pub category: String,
    pub type_name: String,
    pub mnemonic: String,
    pub description: String,
}
#[doc = "detect the VCS used in the current directory"]
#[must_use]
pub fn vcs() -> String {
    let mercurial = Path::new(".hg").is_dir();
    let git = Path::new(".git").is_dir();
    let vcs = if git {
        "git"
    } else if mercurial {
        "hg"
    } else {
        "git"
    };
    String::from(vcs)
}
///
/// Add source code
/// # Panics
/// if bad config
/// # Errors
/// Return an error if the underlying VCS command fails or exits with a non-success status.
///
pub fn add() -> Result<(), Error> {
    if run_hooks().is_ok() {
        let selected = MultiSelect::new("", get_tree())
            .prompt()
            .expect("failed to select");
        let vcs = vcs();
        for file in &selected {
            assert!(
                ok(
                    "added file",
                    std::process::Command::new(vcs.as_str())
                        .arg("add")
                        .arg(file.as_str()),
                    "added file",
                    "failed to add file",
                )
                .is_ok()
            );
        }
        return Ok(());
    }
    Err(Error::other("failed to run add"))
}

///
/// Display the status of the working directory
///
/// # Errors
///
/// Returns an error if the underlying VCS command fails or exits with a non-success status.
pub fn diff() -> Result<(), Error> {
    if Command::new(vcs())
        .arg("diff")
        .arg("-p")
        .arg("--stat")
        .current_dir(".")
        .spawn()?
        .wait()?
        .success()
    {
        Ok(())
    } else {
        Err(Error::other("failed to run diff"))
    }
}

///
/// Display the status of the working directory
///
/// # Errors
///
/// Returns an error if the underlying VCS command fails or exits with a non-success status.
pub fn status() -> Result<(), Error> {
    if Command::new(vcs()).arg("status").spawn()?.wait()?.success() {
        return Ok(());
    }
    Err(Error::other("failed to run status"))
}

///
/// # Errors
///
/// Returns an error if the underlying VCS `commit` command fails.
pub fn run_commit(c: &mut Commit) -> Result<i32, Error> {
    if Command::new(vcs())
        .arg("commit")
        .arg("-m")
        .arg(c.to_string().as_str())
        .current_dir(".")
        .spawn()?
        .wait()?
        .success()
    {
        Ok(0)
    } else {
        Err(Error::other("failed to run commit"))
    }
}

#[derive(Default, Debug, Clone)]
pub struct Commit {
    pub t: String,
    pub scopes: Vec<String>,
    pub summary: String,
    pub why: String,
    pub who: String,
    pub roles: Vec<String>,
    pub what: String,
    pub benefits: String,
    pub breaking_changes: String,
    pub notes: String,
    pub resolves: Vec<String>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub enum Role {
    Team,
    Manager,
    Developer,
    Tester,
    Packager,
    Product,
    Engineering,
    Design,
    Marketing,
    Customer,
}

impl Role {
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::Team,
            Self::Manager,
            Self::Developer,
            Self::Tester,
            Self::Packager,
            Self::Product,
            Self::Engineering,
            Self::Design,
            Self::Marketing,
            Self::Customer,
        ]
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Team => write!(f, "Team"),
            Self::Manager => write!(f, "Manager"),
            Self::Developer => write!(f, "Developer"),
            Self::Tester => write!(f, "Tester"),
            Self::Packager => write!(f, "Packager"),
            Self::Product => write!(f, "Product"),
            Self::Engineering => write!(f, "Engineering"),
            Self::Design => write!(f, "Design"),
            Self::Marketing => write!(f, "Marketing"),
            Self::Customer => write!(f, "Customer"),
        }
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}({}) ~ {}",
            self.t,
            self.scopes.join(","),
            self.summary
        )?;
        writeln!(f, "\n\tWhy changes?\n")?;
        let why_lines = self.why.split('\n').collect::<Vec<&str>>();
        for line in why_lines {
            if line.is_empty() {
                writeln!(f, "\n")?;
                continue;
            }
            writeln!(f, "\t\t* {line}")?;
        }
        writeln!(f, "\n\tBreaking Changes:\n")?;
        let breaking_changes_lines = self.breaking_changes.split('\n').collect::<Vec<&str>>();
        for line in breaking_changes_lines {
            if line.is_empty() {
                writeln!(f, "\n")?;
                continue;
            }
            writeln!(f, "\t\t* {line}")?;
        }
        writeln!(f, "\n\tWhat changes?\n")?;
        let what_lines = self.what.split('\n').collect::<Vec<&str>>();
        for line in what_lines {
            if line.is_empty() {
                writeln!(f, "\n")?;
                continue;
            }
            writeln!(f, "\t\t* {line}")?;
        }
        writeln!(f, "\n\tWho changes?\n")?;
        writeln!(
            f,
            "\t\t* @{} ~ {} ",
            self.who,
            self.roles.join(" ").as_str()
        )?;
        writeln!(f, "\n\tBenefits:\n")?;
        let benefits_lines = self.benefits.split('\n').collect::<Vec<&str>>();
        for line in benefits_lines {
            if line.is_empty() {
                writeln!(f, "\n")?;
                continue;
            }
            writeln!(f, "\t\t* {line}")?;
        }
        writeln!(f, "\n\tNotes:\n")?;
        let notes_lines = self.notes.split('\n').collect::<Vec<&str>>();
        for line in notes_lines {
            if line.is_empty() {
                writeln!(f, "\n")?;
                continue;
            }
            writeln!(f, "\t\t* {line}")?;
        }
        writeln!(f, "\n\tResolves\n")?;
        for resolve in &self.resolves {
            let issue = resolve.split('~').collect::<Vec<&str>>();
            let re = issue.first().expect("bad resolve");
            writeln!(f, "\t\tFixes #{}", re.trim())?;
        }
        writeln!(f, "\n")?;
        Ok(())
    }
}
impl Commit {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    ///
    ///
    /// # Errors
    ///
    /// Bad user or cancel by user
    ///
    pub fn confirm(&mut self) -> InquireResult<&mut Self> {
        println!("{self}");
        if Confirm::new("Confirm commit?")
            .with_default(true)
            .prompt()?
        {
            Ok(self)
        } else {
            Err(InquireError::from(Error::other("commit aborted")))
        }
    }

    ///
    /// Commit the changes to the repository
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn commit(&mut self) -> InquireResult<&mut Self> {
        self.show_status()?
            .show_diff()?
            .add()?
            .ask_type()?
            .ask_scopes()?
            .ask_summary()?
            .ask_roles()?
            .ask_why()?
            .breaking_changes()?
            .ask_what()?
            .ask_who()?
            .ask_benefits()?
            .ask_notes()?
            .ask_resolves()?
            .confirm()
    }

    ///
    /// Show diff between the working directory and the last commit
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn show_diff(&mut self) -> InquireResult<&mut Self> {
        diff()?;
        Ok(self)
    }

    ///
    /// Show status of the working directory
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn show_status(&mut self) -> InquireResult<&mut Self> {
        status()?;
        Ok(self)
    }

    ///
    /// Add the changes to the repository
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn add(&mut self) -> InquireResult<&mut Self> {
        if Confirm::new("Add changes to the repository?")
            .with_default(false)
            .prompt()?
            .eq(&false)
        {
            return Err(InquireError::from(Error::other("commit aborted")));
        }
        add()?;
        Ok(self)
    }
    ///
    /// Ask teams notes
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_notes(&mut self) -> InquireResult<&mut Self> {
        self.notes.clear();
        self.notes
            .push_str(Editor::new("The teams notes:").prompt()?.as_str());
        Ok(self)
    }

    ///
    /// Ask a commit type
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_type(&mut self) -> InquireResult<&mut Self> {
        self.t.clear();
        let x = Select::new("Commit types", types()).prompt()?;
        let all = x.split('~').collect::<Vec<&str>>();
        if let Some(t) = all.first() {
            self.t.push_str(t);
            return Ok(self);
        }
        Err(InquireError::from(Error::other("bad commit type")))
    }

    ///
    /// Ask the author role
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_roles(&mut self) -> InquireResult<&mut Self> {
        self.roles.clear();
        let mut x = Vec::new();
        let mut r = Role::all();
        r.sort();
        for role in &r {
            x.push(role.to_string());
        }
        while self.roles.is_empty() {
            self.roles.clear();
            if x.is_empty() {
                return Err(InquireError::from(Error::other("bad roles")));
            }
            self.roles = MultiSelect::new("Select roles", x.clone()).prompt()?;
        }
        if self.roles.is_empty() {
            return Err(InquireError::from(Error::other("bad roles")));
        }
        Ok(self)
    }
    ///
    /// The small description of the changes
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_summary(&mut self) -> InquireResult<&mut Self> {
        self.summary.clear();
        while self.summary.is_empty() {
            self.summary.clear();
            self.summary
                .push_str(Text::new("Commit summary:").prompt()?.as_str());
        }
        if self.summary.is_empty() {
            return Err(InquireError::from(Error::other("bad summary")));
        }
        Ok(self)
    }
    ///
    /// Why are you making these changes?
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_why(&mut self) -> InquireResult<&mut Self> {
        self.why.clear();
        while self.why.is_empty() {
            self.why.clear();
            self.why.push_str(
                Editor::new("Why are you making this change?")
                    .prompt()?
                    .as_str(),
            );
        }
        if self.why.is_empty() {
            return Err(InquireError::from(Error::other("bad why")));
        }
        Ok(self)
    }

    ///
    /// Who are you in the team?
    ///
    /// # Errors
    /// On bad user inputs
    /// # Panics
    /// if bad config
    ///
    pub fn ask_who(&mut self) -> InquireResult<&mut Self> {
        self.who.clear();
        while self.who.is_empty() {
            self.who.clear();
            if cfg!(windows) {
                self.who
                    .push_str(Text::new("Who are you:").prompt()?.as_str());
            } else {
                self.who.push_str(
                    Text::new("Who are you:")
                        .with_default(env!("USER"))
                        .prompt()?
                        .as_str(),
                );
            }
        }
        if self.who.is_empty() {
            return Err(InquireError::from(Error::other("bad who")));
        }
        Ok(self)
    }

    ///
    /// What changes are you making?
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_what(&mut self) -> InquireResult<&mut Self> {
        self.what.clear();
        while self.what.is_empty() {
            self.what.clear();
            self.what.push_str(
                Editor::new("What changes are you making?")
                    .prompt()?
                    .as_str(),
            );
        }
        if self.what.is_empty() {
            return Err(InquireError::from(Error::other("bad what")));
        }
        Ok(self)
    }

    ///
    /// What code resolve
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_benefits(&mut self) -> InquireResult<&mut Self> {
        self.benefits.clear();
        while self.benefits.is_empty() {
            self.benefits.clear();
            self.benefits.push_str(
                Editor::new("What benefits does this change provide?")
                    .prompt()?
                    .as_str(),
            );
        }
        if self.benefits.is_empty() {
            return Err(InquireError::from(Error::other("bad benefits")));
        }
        Ok(self)
    }

    /// # Panics
    /// if bad config
    /// # Errors
    /// On bad user inputs
    pub fn ask_resolves(&mut self) -> InquireResult<&mut Self> {
        let mut issues: Vec<String> = Vec::new();
        self.resolves.clear();
        while issues.is_empty() {
            issues.clear();
            issues.push(Text::new("Issues number:").prompt()?);
        }
        self.resolves.append(&mut issues);
        issues.clear();
        if Confirm::new("Commit resolve more than one issue")
            .with_default(false)
            .prompt()?
        {
            loop {
                while issues.is_empty() {
                    issues.clear();
                    issues.push(Text::new("Issues number:").prompt()?);
                }
                if Confirm::new("Add more issue?")
                    .with_default(false)
                    .prompt()?
                {
                    self.resolves.append(&mut issues);
                } else {
                    break;
                }
            }
        }
        if self.resolves.is_empty() {
            return Err(InquireError::from(Error::other("bad resolves")));
        }
        Ok(self)
    }

    ///
    /// What code resolve
    ///
    /// # Panics
    /// if bad config
    ///
    /// # Errors
    /// On bad user inputs
    ///
    pub fn ask_scopes(&mut self) -> InquireResult<&mut Self> {
        self.scopes.clear();
        let mut scopes = Vec::new();
        let conf = load_config();
        for scope in &conf.breathes.scopes {
            scopes.push(scope.clone());
        }
        while self.scopes.is_empty() {
            self.scopes.clear();
            self.scopes = MultiSelect::new("Select scopes", scopes.clone()).prompt()?;
        }
        if self.scopes.is_empty() {
            return Err(InquireError::from(Error::other("bad scopes")));
        }
        Ok(self)
    }

    ///
    /// Ask for after changes
    ///
    /// # Errors
    /// On bad user inputs
    ///
    pub fn breaking_changes(&mut self) -> InquireResult<&mut Self> {
        self.breaking_changes.clear();
        while self.breaking_changes.is_empty() {
            self.breaking_changes.clear();
            self.breaking_changes
                .push_str(Editor::new("Breaking changes?").prompt()?.as_str());
        }
        if self.breaking_changes.is_empty() {
            return Err(InquireError::from(Error::other("bad breaking changes")));
        }
        Ok(self)
    }
}
