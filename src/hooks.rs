use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum Language {
    Node,
    Rust,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Node => write!(f, "Node"),
            Language::Rust => write!(f, "Rust"),
        }
    }
}
#[derive(Clone)]
pub struct Hooks {
    pub language: Language,
    pub description: &'static str,
    pub success: &'static str,
    pub failure: &'static str,
    pub file: &'static str,
    pub command: &'static str,
}

pub const NODE_HOOKS: [Hooks; 2] = [
    Hooks {
        language: Language::Node,
        description: "Checks for outdated packages in your Node.js project.",
        success: "No outdated packages found",
        failure: "Outdated packages found",
        command: "outdated",
        file: "outdated.log",
    },
    Hooks {
        language: Language::Node,
        description: "Runs tests for your Node.js project.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "run test",
    },
];

pub const RUST_HOOKS: [Hooks; 5] = [
    Hooks {
        language: Language::Rust,
        description: "Checks for security vulnerabilities in your Rust dependencies.",
        success: "no vulnerabilities found",
        failure: "vulnerabilities found",
        file: "audit.log",
        command: "audit",
    },
    Hooks {
        language: Language::Rust,
        description: "Checks for formatting issues in your Rust code.",
        file: "fmt.log",
        success: "format respect standard",
        failure: "format not respect standard",
        command: "fmt --check",
    },
    Hooks {
        language: Language::Rust,
        description: "Checks for linting issues and suggests code improvements.",
        success: "No issues found",
        failure: "Clippy checks found issues",
        file: "clippy.log",
        command: "clippy -- -D clippy::all -D warnings",
    },
    Hooks {
        language: Language::Rust,
        description: "Runs tests for your Rust project.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "test --no-fail-fast",
    },
    Hooks {
        language: Language::Rust,
        description: "Checks for outdated packages in your Rust project.",
        success: "No outdated packages found",
        failure: "Outdated packages found",
        command: "outdated",
        file: "outdated.log",
    },
];
