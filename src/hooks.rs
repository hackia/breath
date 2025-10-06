use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum Language {
    Node,
    Rust,
    Java,
    Python,
    Go,
    Php,
    Ruby,
    CMake,
    CSharp,
    Kotlin,
    Swift,
    Dart,
    Elixir,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Node => write!(f, "Node"),
            Self::Rust => write!(f, "Rust"),
            Self::Java => write!(f, "Java"),
            Self::Python => write!(f, "Python"),
            Self::Go => write!(f, "Go"),
            Self::Php => write!(f, "Php"),
            Self::Ruby => write!(f, "Ruby"),
            Self::CMake => write!(f, "CMake"),
            Self::CSharp => write!(f, "Csharp"),
            Self::Kotlin => write!(f, "Kotlin"),
            Self::Swift => write!(f, "Swift"),
            Self::Dart => write!(f, "Dart"),
            Self::Elixir => write!(f, "Elixir"),
        }
    }
}
#[derive(Clone)]
pub struct Hook {
    pub language: Language,
    pub description: &'static str,
    pub success: &'static str,
    pub failure: &'static str,
    pub file: &'static str,
    pub command: &'static str,
}

pub const CSHARP_HOOKS: [Hook; 5] = [
    Hook {
        language: Language::CSharp,
        description: "Checks for code formatting in your C# project.",
        success: "Code formatting is correct",
        failure: "Code formatting issues found",
        file: "format.log",
        command: "format --verify-no-changes",
    },
    Hook {
        language: Language::CSharp,
        description: "Builds the C# project.",
        success: "Build successful",
        failure: "Build failed",
        file: "build.log",
        command: "build",
    },
    Hook {
        language: Language::CSharp,
        description: "Runs unit tests for your C# project.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "test",
    },
    Hook {
        language: Language::CSharp,
        description: "Performs static code analysis.",
        success: "No issues found",
        failure: "Code analysis issues found",
        file: "analyze.log",
        command: "analyze",
    },
    Hook {
        language: Language::CSharp,
        description: "Checks NuGet package dependencies.",
        success: "Dependencies are up to date",
        failure: "Dependency issues found",
        file: "deps.log",
        command: "restore",
    },
];
pub const JAVA_HOOKS: [Hook; 5] = [
    Hook {
        language: Language::Java,
        description: "Checks code formatting with Google Java Format.",
        success: "Code formatting is correct",
        failure: "Code formatting issues found",
        file: "format.log",
        command: "google-java-format --dry-run",
    },
    Hook {
        language: Language::Java,
        description: "Builds the Java project with Maven.",
        success: "Build successful",
        failure: "Build failed",
        file: "build.log",
        command: "clean compile",
    },
    Hook {
        language: Language::Java,
        description: "Runs unit tests with JUnit.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "test",
    },
    Hook {
        language: Language::Java,
        description: "Analyzes code with SpotBugs.",
        success: "No issues found",
        failure: "Code analysis issues found",
        file: "analyze.log",
        command: "spotbugs:check",
    },
    Hook {
        language: Language::Java,
        description: "Checks for dependency updates.",
        success: "Dependencies are up to date",
        failure: "Dependency updates available",
        file: "deps.log",
        command: "versions:display-dependency-updates",
    },
];
pub const GO_HOOKS: [Hook; 5] = [
    Hook {
        language: Language::Go,
        description: "Checks for code formatting in your Go project.",
        success: "Code formatting is correct",
        failure: "Code formatting issues found",
        file: "gofmt.log",
        command: "fmt -x",
    },
    Hook {
        language: Language::Go,
        description: "Runs unit tests for your Go project.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "test ./...",
    },
    Hook {
        language: Language::Go,
        description: "Performs static code analysis.",
        success: "No issues found",
        failure: "Static analysis issues found",
        file: "lint.log",
        command: "vet ./...",
    },
    Hook {
        language: Language::Go,
        description: "Checks dependency management.",
        success: "Dependencies are properly managed",
        failure: "Dependency issues found",
        file: "deps.log",
        command: "mod tidy",
    },
    Hook {
        language: Language::Go,
        description: "Builds the Go project.",
        success: "Build successful",
        failure: "Build failed",
        file: "build.log",
        command: "build",
    },
];
pub const CMAKE_HOOKS: [Hook; 1] = [Hook {
    language: Language::CMake,
    description: "Validates CMake configuration files.",
    success: "CMake configuration is valid",
    failure: "CMake configuration validation failed",
    file: "cmake-validate.log",
    command: "cmake . && make && make test",
}];

pub const PHP_HOOKS: [Hook; 4] = [
    Hook {
        language: Language::Php,
        description: "Checks for outdated packages in your PHP project.",
        success: "No outdated packages found",
        failure: "Outdated packages found",
        command: "outdated",
        file: "outdated.log",
    },
    Hook {
        language: Language::Php,
        description: "Checks for security vulnerabilities in your PHP dependencies.",
        success: "No security vulnerabilities found",
        failure: "Security vulnerabilities found",
        command: "security-check",
        file: "security.log",
    },
    Hook {
        language: Language::Php,
        description: "Checks for formatting issues in your PHP code.",
        success: "No formatting issues found",
        failure: "Formatting issues found",
        command: "php-cs-fixer fix --dry-run --diff",
        file: "php-cs-fixer.log",
    },
    Hook {
        language: Language::Php,
        description: "Runs tests for your PHP project.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "run test",
    },
];

pub const NODE_HOOKS: [Hook; 2] = [
    Hook {
        language: Language::Node,
        description: "Checks for outdated packages in your Node.js project.",
        success: "No outdated packages found",
        failure: "Outdated packages found",
        command: "outdated",
        file: "outdated.log",
    },
    Hook {
        language: Language::Node,
        description: "Runs tests for your Node.js project.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "run test",
    },
];

pub const RUST_HOOKS: [Hook; 5] = [
    Hook {
        language: Language::Rust,
        description: "Checks for security vulnerabilities in your Rust dependencies.",
        success: "no vulnerabilities found",
        failure: "vulnerabilities found",
        file: "audit.log",
        command: "audit",
    },
    Hook {
        language: Language::Rust,
        description: "Checks for formatting issues in your Rust code.",
        file: "fmt.log",
        success: "format respect standard",
        failure: "format not respect standard",
        command: "fmt --check",
    },
    Hook {
        language: Language::Rust,
        description: "Checks for linting issues and suggests code improvements.",
        success: "No issues found",
        failure: "Clippy checks found issues",
        file: "clippy.log",
        command: "clippy -- -D clippy::all -W warnings -D clippy::pedantic -D clippy::nursery -A clippy::multiple_crate_versions -W clippy::cargo",
    },
    Hook {
        language: Language::Rust,
        description: "Runs tests for your Rust project.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "test --no-fail-fast",
    },
    Hook {
        language: Language::Rust,
        description: "Checks for outdated packages in your Rust project.",
        success: "No outdated packages found",
        failure: "Outdated packages found",
        command: "outdated",
        file: "outdated.log",
    },
];
