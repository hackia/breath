use std::fmt::{Display, Formatter};
use tabled::Tabled;

#[derive(Clone, Hash, Eq, PartialEq, Tabled)]
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
    All,
}

pub const RUST_FILE: &str = "Cargo.toml";
pub const GO_FILE: &str = "go.mod";
pub const PHP_FILE: &str = "composer.json";
pub const NODE_FILE: &str = "package.json";
pub const CMAKE_FILE: &str = "CMakeLists.txt";
pub const LANGUAGES: [(Language, &str); 5] = [
    (Language::Rust, RUST_FILE),
    (Language::Go, GO_FILE),
    (Language::Php, PHP_FILE),
    (Language::Node, NODE_FILE),
    (Language::CMake, CMAKE_FILE),
];
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
            Self::All => write!(f, "All"),
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
        description: "Checking for code formatting",
        success: "Code formatting is correct",
        failure: "Code formatting issues found",
        file: "format.log",
        command: "dotnet format --verify-no-changes",
    },
    Hook {
        language: Language::CSharp,
        description: "Building your project.",
        success: "Build successful",
        failure: "Build failed",
        file: "build.log",
        command: "dotnet build",
    },
    Hook {
        language: Language::CSharp,
        description: "Running unit tests.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "dotnet test",
    },
    Hook {
        language: Language::CSharp,
        description: "Performing static code analysis.",
        success: "No issues found",
        failure: "Code analysis issues found",
        file: "analyze.log",
        command: "dotnet analyze",
    },
    Hook {
        language: Language::CSharp,
        description: "Checking nuget package dependencies.",
        success: "Dependencies are up to date",
        failure: "Dependency issues found",
        file: "deps.log",
        command: "dotnet restore",
    },
];
pub const JAVA_HOOKS: [Hook; 3] = [
    Hook {
        language: Language::Java,
        description: "Building your Java project.",
        success: "Build successful",
        failure: "Build failed",
        file: "build.log",
        command: "java clean compile",
    },
    Hook {
        language: Language::Java,
        description: "Running unit tests.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "test",
    },
    Hook {
        language: Language::Java,
        description: "Checking for dependency updates.",
        success: "Dependencies are up to date",
        failure: "Dependency updates available",
        file: "deps.log",
        command: "versions:display-dependency-updates",
    },
];
pub const GO_HOOKS: [Hook; 5] = [
    Hook {
        language: Language::Go,
        description: "Checking code format.",
        success: "Code formatting is correct",
        failure: "Code formatting issues found",
        file: "gofmt.log",
        command: "go fmt",
    },
    Hook {
        language: Language::Go,
        description: "Running unit tests.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "go test -v",
    },
    Hook {
        language: Language::Go,
        description: "Performs static code analysis.",
        success: "No issues found",
        failure: "Static analysis issues found",
        file: "lint.log",
        command: "go vet",
    },
    Hook {
        language: Language::Go,
        description: "Checking dependency management.",
        success: "Dependencies are properly managed",
        failure: "Dependency issues found",
        file: "deps.log",
        command: "go mod tidy",
    },
    Hook {
        language: Language::Go,
        description: "Builds the Go project.",
        success: "Build successful",
        failure: "Build failed",
        file: "build.log",
        command: "go build",
    },
];
pub const CMAKE_HOOKS: [Hook; 3] = [
    Hook {
        language: Language::CMake,
        description: "Validates CMake configuration files.",
        success: "CMake configuration is valid",
        failure: "CMake configuration validation failed",
        file: "cmake.log",
        command: "cmake .",
    },
    Hook {
        language: Language::CMake,
        description: "Validates CMake configuration files.",
        success: "Build success",
        failure: "Build failed",
        file: "make.log",
        command: "make",
    },
    Hook {
        language: Language::CMake,
        description: "Validates CMake configuration files.",
        success: "Test success",
        failure: "test failures",
        file: "test.log",
        command: "make test",
    },
];

pub const PHP_HOOKS: [Hook; 6] = [
    Hook {
        language: Language::Php,
        description: "Installing the project.",
        success: "Installed successfully on your system",
        failure: "Failed to install in your system",
        file: "install.log",
        command: "composer install",
    },
    Hook {
        language: Language::Php,
        description: "Checking our php extensions.",
        success: "No missing extensions on your system",
        failure: "Mising extension in your system",
        file: "check-platform-reqs.log",
        command: "composer check-platform-reqs",
    },
    Hook {
        language: Language::Php,
        description: "Checking the composer file.",
        success: "Composer file is valid",
        failure: "Composer file is not valid",
        command: "composer validate",
        file: "validate.log",
    },
    Hook {
        language: Language::Php,
        description: "Checking for security vulnerabilities.",
        success: "No vulnerabilities has been founded",
        failure: "Vulnerabilities has been founded",
        command: "composer audit",
        file: "audit.log",
    },
    Hook {
        language: Language::Php,
        description: "Checking outdated packages.",
        success: "No outdated packages found",
        failure: "Outdated packages founded",
        command: "composer outdated",
        file: "outdated.log",
    },
    Hook {
        language: Language::Php,
        description: "Runs tests for your PHP project.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "composer run test",
    },
];

pub const NODE_HOOKS: [Hook; 3] = [
    Hook {
        language: Language::Node,
        description: "Checks for outdated packages in your project.",
        success: "No outdated packages found",
        failure: "Outdated packages found",
        command: "npm outdated",
        file: "outdated.log",
    },
    Hook {
        language: Language::Node,
        description: "Testings your project.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "npm run test",
    },
    Hook {
        language: Language::Node,
        description: "Auditing your project.",
        success: "No vulnerabilities founded",
        failure: "Vulnerabilities founded",
        file: "audit.log",
        command: "npm audit",
    },
];

pub const RUST_HOOKS: [Hook; 7] = [
    Hook {
        language: Language::Rust,
        description: "Checking the configuration",
        success: "Project is valid",
        failure: "Project not valid",
        file: "project.log",
        command: "cargo verify-project",
    },
    Hook {
        language: Language::Rust,
        description: "Checking build capability",
        success: "Can build the project",
        failure: "Cargo check detect failure",
        file: "check.log",
        command: "cargo check",
    },
    Hook {
        language: Language::Rust,
        description: "Checking for security vulnerabilities",
        success: "No vulnerabilities found",
        failure: "Vulnerabilities found",
        file: "audit.log",
        command: "cargo audit",
    },
    Hook {
        language: Language::Rust,
        description: "Checks for formatting issues in your Rust code.",
        file: "fmt.log",
        success: "Respect the code format standard",
        failure: "Not respect code format standard",
        command: "cargo fmt --check",
    },
    Hook {
        language: Language::Rust,
        description: "Checks for linting issues and suggests code improvements.",
        success: "No issues found",
        failure: "Lint founded some warnings",
        file: "clippy.log",
        command: "cargo clippy -- -D clippy::all -W warnings -D clippy::pedantic -D clippy::nursery -A clippy::multiple_crate_versions",
    },
    Hook {
        language: Language::Rust,
        description: "Testing your project.",
        success: "Tests passed",
        failure: "Tests failed",
        file: "test.log",
        command: "cargo test --no-fail-fast",
    },
    Hook {
        language: Language::Rust,
        description: "Checks for outdated packages in your Rust project.",
        success: "No outdated packages found",
        failure: "Outdated packages found",
        command: "cargo outdated",
        file: "outdated.log",
    },
];
