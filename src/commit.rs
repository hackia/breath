use crate::utils::types;
use inquire::error::InquireResult;
use inquire::{Confirm, Editor, MultiSelect, Select, Text};
use std::fmt::{Display, Formatter};
use std::io::Error;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
#[doc = "Represent a commit type"]
pub struct CommitType {
    pub category: &'static str,
    pub type_name: &'static str,
    pub mnemonic: &'static str,
    pub description: &'static str,
}

#[doc = "all commit types supported"]
pub const COMMIT_TYPES: [CommitType; 46] = [
    CommitType {
        category: "Core Changes",
        type_name: "Star",
        mnemonic: "Shiny Technology Added or Refined",
        description: "New feature or enhancement",
    },
    CommitType {
        category: "Core Changes",
        type_name: "Comet",
        mnemonic: "Code or Module Error Terminated",
        description: "Bug fix or error resolution",
    },
    CommitType {
        category: "Core Changes",
        type_name: "Nebula",
        mnemonic: "New Efficient Better Understandable Logic Achieved",
        description: "Code refactoring",
    },
    CommitType {
        category: "Core Changes",
        type_name: "Pulsar",
        mnemonic: "Powerful Upgrade, Less Sluggish, Agile Response",
        description: "Performance improvement",
    },
    CommitType {
        category: "Core Changes",
        type_name: "Quasar",
        mnemonic: "Quick Adjustments for Superior Accuracy and Readability",
        description: "Documentation or clarity improvement",
    },
    CommitType {
        category: "Maintenance & Infrastructure",
        type_name: "Asteroid Belt",
        mnemonic: "Adjustments, Sweeps, Tidy-ups, Elimination, Reordering of Items, Decrease Bloat",
        description: "Code cleanup and maintenance",
    },
    CommitType {
        category: "Maintenance & Infrastructure",
        type_name: "Solar Flare",
        mnemonic: "Securing Our Logic Against Regressions, Failures, and Latencies Actively, Rigorously Ensured",
        description: "Adding or updating tests",
    },
    CommitType {
        category: "Maintenance & Infrastructure",
        type_name: "Dwarf Planet",
        mnemonic: "Details Warranted Attention, Refined Further, Polished Little Aspects Neatly Enhanced",
        description: "Tiny Minor but essential updates or fixes",
    },
    CommitType {
        category: "Maintenance & Infrastructure",
        type_name: "Terraform",
        mnemonic: "Technology Engineering Resources Readily Automated, Foundation of Reliable Management",
        description: "Infrastructure changes",
    },
    CommitType {
        category: "Project Events",
        type_name: "Black Hole",
        mnemonic: "Big Legacy Aspects Consumed, Killing Heavy, Old Loads Entirely",
        description: "Removing large chunks of code or features",
    },
    CommitType {
        category: "Project Events",
        type_name: "Wormhole",
        mnemonic: "Weaving or Reconnecting Modules, Hitching onto Linked Elements",
        description: "Merging branches or connecting code parts",
    },
    CommitType {
        category: "Project Events",
        type_name: "Big Bang",
        mnemonic: "Birth of Initial Greatness, Beginning All New Growth",
        description: "Initial commit of a project or major feature",
    },
    CommitType {
        category: "Project Events",
        type_name: "Launch",
        mnemonic: "Lifting Application Upward, New Code Entering Production",
        description: "Deploying to production or releasing a version",
    },
    CommitType {
        category: "Communication & Collaboration",
        type_name: "Lightspeed",
        mnemonic: "Lightening Speed Enhancements",
        description: "Significant performance improvements",
    },
    CommitType {
        category: "Communication & Collaboration",
        type_name: "Mission Control",
        mnemonic: "Managing Changes, Issues, Scope, Teamwork, and Release On Time",
        description: "Project management changes",
    },
    CommitType {
        category: "Communication & Collaboration",
        type_name: "Spacewalk",
        mnemonic: "Swift Work Above Limits, Keeping All Systems Extra Safe",
        description: "Urgent hotfixes or critical production updates",
    },
    CommitType {
        category: "Communication & Collaboration",
        type_name: "Moon Landing",
        mnemonic: "Major Leaps Over Night, New Doors and Incredible Achievements",
        description: "Completing major milestones or goals",
    },
    CommitType {
        category: "Communication & Collaboration",
        type_name: "First Contact",
        mnemonic: "Forge Initial Connections, Open New Territories",
        description: "Establishing initial connections or integrations",
    },
    CommitType {
        category: "Communication & Collaboration",
        type_name: "Interstellar Communication",
        mnemonic: "Informing, Sharing, Teaching, Educating, & Learning Lucidly & Clearly",
        description: "Improving documentation or communication",
    },
    CommitType {
        category: "Celestial Events",
        type_name: "Solar Eclipse",
        mnemonic: "Sun Escapes, Legacy Code Lurks",
        description: "Temporarily masking functionality",
    },
    CommitType {
        category: "Celestial Events",
        type_name: "Supernova",
        mnemonic: "Sudden Unbelievable Performance Revolution, New Version Arrives",
        description: "Major, transformative change or improvement",
    },
    CommitType {
        category: "Celestial Events",
        type_name: "Meteor Shower",
        mnemonic: "Many Edits, Tiny Overall Result, Overhaul Routines",
        description: "Series of small changes or fixes",
    },
    CommitType {
        category: "Celestial Events",
        type_name: "Cosmic Dawn",
        mnemonic: "Creating Original, Simple, Minimal Initial Draft",
        description: "Initial implementation of a feature",
    },
    CommitType {
        category: "Celestial Events",
        type_name: "Solar Storm",
        mnemonic: "Sudden Transformations Occur Rapidly, Modifications",
        description: "Rapid, impactful changes",
    },
    CommitType {
        category: "Celestial Events",
        type_name: "Lunar Transit",
        mnemonic: "Little Update, Now Adjustments Require Testing",
        description: "Minor, temporary change",
    },
    CommitType {
        category: "Celestial Events",
        type_name: "Perihelion",
        mnemonic: "Perfect Ending, Refined, Improved, High Efficiency, Low Obstacles, Near Goal",
        description: "Significant milestone or feature completion",
    },
    CommitType {
        category: "Celestial Events",
        type_name: "Aphelion",
        mnemonic: "Away From Perfection, High Effort, Long Overhaul, Intense Overhaul, Obstacles",
        description: "Refactor, dependency update, or architecture change",
    },
    CommitType {
        category: "Celestial Objects",
        type_name: "White Dwarf",
        mnemonic: "Writing, Improving, Detailed Documentation For All",
        description: "Improving code comments or documentation",
    },
    CommitType {
        category: "Celestial Objects",
        type_name: "Red Giant",
        mnemonic: "Refactoring, Enhancing, Growing, Increasing, Adding New Things",
        description: "Expanding a feature or functionality",
    },
    CommitType {
        category: "Celestial Objects",
        type_name: "Neutron Star",
        mnemonic: "New Efficient Utility, Tweaks, Robust Optimization, Nimble Solution",
        description: "Optimizing code for performance",
    },
    CommitType {
        category: "Celestial Objects",
        type_name: "Binary Star",
        mnemonic: "Bringing In New And Revised, Yielding Integrated Results",
        description: "Merging features or components",
    },
    CommitType {
        category: "Celestial Objects",
        type_name: "Brown Dwarf",
        mnemonic: "Barely Developed, Requires Work, Ongoing Development For Future",
        description: "Undeveloped feature with potential",
    },
    CommitType {
        category: "Celestial Objects",
        type_name: "Quark Star",
        mnemonic: "Questionable, Unstable, Anticipated Results, Risky, Keen Experiment",
        description: "Experimental or speculative change",
    },
    CommitType {
        category: "Celestial Objects",
        type_name: "Rogue Planet",
        mnemonic: "Refactoring Or Generating Operations, Unique Path, Leaping Ahead",
        description: "Independent change unrelated to the main codebase",
    },
    CommitType {
        category: "Celestial Objects",
        type_name: "Stellar Nursery",
        mnemonic: "Starting To Enhance, Laying Layers, Launching New Requirements",
        description: "Creating new components",
    },
    CommitType {
        category: "Celestial Objects",
        type_name: "Planetary Nebula",
        mnemonic: "Pruning, Leaving, Abandoning, Nostalgic Era, Totally Removed",
        description: "Removal or deprecation of a component",
    },
    CommitType {
        category: "Celestial Objects",
        type_name: "Globular Cluster",
        mnemonic: "Gathering, Linking, Operations, Bringing Unity, Lots of Adjustments, All Related",
        description: "Collection of related changes",
    },
    CommitType {
        category: "Celestial Objects",
        type_name: "Void",
        mnemonic: "Vanished, Obliterated, Irrelevant, Deleted",
        description: "Removal of a module, component, or feature",
    },
    CommitType {
        category: "Astronomical Concepts",
        type_name: "Gravity",
        mnemonic: "Glitch Resolution, Adjusting Versions, Integrating, Troubleshooting Yielding",
        description: "Resolving merge conflicts or dependencies",
    },
    CommitType {
        category: "Astronomical Concepts",
        type_name: "Dark Matter",
        mnemonic: "Debugging And Resolving Mysterious Attributes, Tricky issues Removed",
        description: "Fixing unknown or mysterious bugs",
    },
    CommitType {
        category: "Astronomical Concepts",
        type_name: "Time Dilation",
        mnemonic: "Time Is Dilated, Improvements Leverage Agility, Time-Saving",
        description: "Improving code performance or reducing execution time",
    },
    CommitType {
        category: "Space Exploration",
        type_name: "Space Probe",
        mnemonic: "Surveying, Planning, Analysing, Checking Every Nook",
        description: "Testing new features or technologies",
    },
    CommitType {
        category: "Space Exploration",
        type_name: "Space Station",
        mnemonic: "Setting Up The Area, Testing In Orbit, Optimising New",
        description: "Creating or improving environments",
    },
    CommitType {
        category: "Space Exploration",
        type_name: "Rocket Launch",
        mnemonic: "Releasing Our Code, Keenly Entering The Production",
        description: "Deploying to production",
    },
    CommitType {
        category: "Space Exploration",
        type_name: "Spacewalk",
        mnemonic: "Swift Patches And Lookout Work, Keeping Systems Extra safe",
        description: "Urgent production hotfixes",
    },
    CommitType {
        category: "Space Exploration",
        type_name: "Space Elevator",
        mnemonic: "Streamlined Access, Providing Easy Vertical On boarding, Lifting Entries",
        description: "Making code base more accessible",
    },
];

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
///
/// # Errors
///
/// Return an error if the underlying VCS command fails or exits with a non-success status.
///
pub fn add() -> Result<(), Error> {
    if Command::new(vcs())
        .arg("add")
        .arg(".")
        .current_dir(".")
        .spawn()?
        .wait()?
        .success()
    {
        Ok(())
    } else {
        Err(Error::other("failed to add files"))
    }
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
    if Command::new(vcs())
        .arg("status")
        .current_dir(".")
        .spawn()?
        .wait()?
        .success()
    {
        return Ok(());
    }
    Err(Error::other("failed to run status"))
}

///
/// # Errors
///
/// Returns an error if the underlying VCS `commit` command fails.
pub fn run_commit(c: &Commit) -> Result<i32, Error> {
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
    pub summary: String,
    pub why: String,
    pub who: String,
    pub roles: Vec<Role>,
    pub when: String,
    pub before: String,
    pub after: String,
    pub requirements: String,
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
    Other(String),
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
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}
impl Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} ~ {}", self.t, self.summary)?;
        writeln!(f, "## Why changes")?;
        writeln!(f, "{}", self.why)?;
        writeln!(f, "## Who changes")?;
        writeln!(f, "@{} ", self.who)?;
        writeln!(f, "## Roles")?;
        for role in &self.roles {
            writeln!(f, "@{role}")?;
        }
        writeln!(f, "## When changes")?;
        writeln!(f, "{}", self.when)?;
        writeln!(f, "## Before changes")?;
        writeln!(f, "{}", self.before)?;
        writeln!(f, "## After changes")?;
        writeln!(f, "{}", self.after)?;
        writeln!(f, "## Requirements")?;
        writeln!(f, "{}", self.requirements)?;
        writeln!(f, "## Teams notes")?;
        writeln!(f, "{}", self.notes)?;
        writeln!(f, "## Resolves")?;
        for resolve in &self.resolves {
            writeln!(f, "Fixes #{resolve}")?;
        }
        Ok(())
    }
}
impl Commit {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    ///
    /// Commit the changes to the repository
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn commit(&mut self) -> InquireResult<&mut Self> {
        self.ask_type()?
            .ask_summary()?
            .ask_roles()?
            .ask_why()?
            .ask_when()?
            .ask_who()?
            .ask_before()?
            .ask_after()?
            .ask_requirements()?
            .ask_notes()?
            .ask_resolves()
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
        self.t
            .push_str(Select::new("Commit types", types()).prompt()?.as_str());
        Ok(self)
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
        let mut roles = Role::all();
        roles.sort();
        self.roles = MultiSelect::new("Select roles", roles).prompt()?;
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
        self.summary
            .push_str(Text::new("Commit summary:").prompt()?.as_str());
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
        self.why.push_str(
            Editor::new("Why are you making this change:")
                .prompt()?
                .as_str(),
        );
        Ok(self)
    }

    ///
    /// Who are you in the team?
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_who(&mut self) -> InquireResult<&mut Self> {
        self.who.clear();
        self.who.push_str(
            Text::new("Who are you:")
                .with_default(env!("USER"))
                .prompt()?
                .as_str(),
        );
        Ok(self)
    }

    ///
    /// When are you making these changes?
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_when(&mut self) -> InquireResult<&mut Self> {
        self.when.push_str(
            Text::new("When are you making this change:")
                .prompt()?
                .as_str(),
        );
        Ok(self)
    }

    ///
    /// What code resolve
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_before(&mut self) -> InquireResult<&mut Self> {
        self.before
            .push_str(Editor::new("Before making this change:").prompt()?.as_str());
        Ok(self)
    }

    ///
    /// Ask for after changes
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_after(&mut self) -> InquireResult<&mut Self> {
        self.after
            .push_str(Text::new("After making this change:").prompt()?.as_str());
        Ok(self)
    }
    ///
    /// ask the requirements after changes
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_requirements(&mut self) -> InquireResult<&mut Self> {
        self.requirements.clear();
        self.requirements
            .push_str(Editor::new("Requirements:").prompt()?.as_str());
        Ok(self)
    }
    ///
    /// Issue resolved
    ///
    /// # Errors
    ///
    /// On bad user inputs
    ///
    pub fn ask_resolves(&mut self) -> InquireResult<&mut Self> {
        self.resolves
            .push(Text::new("Resolves:").prompt()?.as_str().to_string());
        while Confirm::new("Add another resolve?").prompt()? {
            self.resolves
                .push(Text::new("Resolves:").prompt()?.as_str().to_string());
        }
        Ok(self)
    }
}
