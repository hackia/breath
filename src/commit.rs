use crate::utils::{run_hooks, types};
use crossterm::style::Stylize;
use inquire::{Confirm, Editor, Select, Text};
use std::path::Path;
use std::process::{Command, ExitCode};
#[derive(Debug)]
pub struct CommitType {
    pub category: &'static str,
    pub type_name: &'static str,
    pub mnemonic: &'static str,
    pub description: &'static str,
}

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

fn vcs() -> String {
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

fn add() {
    assert!(
        Command::new(vcs())
            .arg("add")
            .arg(".")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success()
    );
}
fn diff() {
    if Confirm::new("show diff")
        .with_default(true)
        .prompt()
        .expect("failed to get prompt")
    {
        assert!(
            Command::new(vcs())
                .arg("diff")
                .arg("-p")
                .current_dir(".")
                .spawn()
                .expect("vcs")
                .wait()
                .expect("failed")
                .success()
        );
    }
}

fn commit(msg: &str) -> ExitCode {
    if Command::new(vcs())
        .arg("commit")
        .arg("-m")
        .arg(msg)
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success()
    {
        ExitCode::SUCCESS
    } else {
        eprintln!("failed to run commit");
        ExitCode::FAILURE
    }
}
pub const COMMIT_MESSAGE: &str = "%type%: %summary%\n\n%body%\n";

pub struct Zen;
impl Zen {
    ///
    /// # Panics
    ///
    #[must_use]
    pub fn commit() -> ExitCode {
        if run_hooks().is_err() {
            return ExitCode::FAILURE;
        }
        loop {
            diff();
            add();
            let t = Select::new(
                "Select a type:".green().bold().to_string().as_str(),
                types(),
            )
            .with_vim_mode(true)
            .prompt()
            .expect("failed to get type");
            let summary = Text::new("Commit summary".green().bold().to_string().as_str())
                .prompt()
                .expect("failed to get summary");
            let body = Editor::new("Explain changes:")
                .prompt()
                .expect("failed to get body");

            let y = t.split('~').collect::<Vec<&str>>();
            let msg = COMMIT_MESSAGE
                .replace("%type%", y.first().expect("").trim_end())
                .replace("%summary%", summary.trim_end())
                .replace("%body%", body.as_str());

            println!("{msg}");
            if Confirm::new("Use this commit message")
                .with_default(true)
                .prompt()
                .expect("failed to get if ")
                .eq(&false)
            {
                println!("aborted commit");
                return ExitCode::SUCCESS;
            }
            if t.is_empty() || summary.is_empty() || body.is_empty() {
                continue;
            }
            return commit(msg.as_str());
        }
    }
}
