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
    Maven,
    Kotlin,
    Gradle,
    Swift,
    Dart,
    Elixir,
}

impl From<String> for Language {
    fn from(value: String) -> Self {

        if value.eq("Node") {
            return Language::Node;
        }
        if value.eq("Rust") {
            return Language::Rust;
        }
        if value.eq("Java") {
            return Language::Java;
        }
        if value.eq("Python") {
            return Language::Python;
        }
        if value.eq("Go") {
            return Language::Go;
        }
        if value.eq("Php") {
            return Language::Php;
        }
        if value.eq("Ruby") {
            return Language::Ruby;
        }
        if value.eq("CMake") {
            return Language::CMake;
        }
        if value.eq("CSharp") {
            return Language::CSharp;
        }
        if value.eq("Maven") {
           return Language::Maven;
        }
        if value.eq("Kotlin") {
            return Language::Kotlin;
        }
        if value.eq("Gradle") {
            return Language::Gradle;
        }
        if value.eq("Swift") {
            return Language::Swift;
        }
        if value.eq("Dart") {
            return Language::Dart;
        }
        if value.eq("Elixir") {
            return Language::Elixir;
        }
        Language::Rust
    }
}
impl Language {
    pub fn get_file(language: &Language) -> &'static str {
        match language {
            Language::Node => "package.json",
            Language::Rust => "Cargo.toml",
            Language::Java => "pom.xml",
            Language::Python => "requirements.txt",
            Language::Go => "go.mod",
            Language::Php => "composer.json",
            Language::Ruby => "Gemfile",
            Language::CMake => "CMakeLists.txt",
            Language::CSharp => "*.csproj",
            Language::Maven => "pom.xml",
            Language::Kotlin => "build.gradle.kts",
            Language::Gradle => "build.gradle",
            Language::Swift => "Package.swift",
            Language::Dart => "pubspec.yaml",
            Language::Elixir => "mix.exs",
        }
    }
}
pub const CS_PROJ: &str = "*.csproj";
pub const MAVEN_POM: &str = "pom.xml";
pub const GRADLE_BUILD: &str = "build.gradle";
pub const RUST_FILE: &str = "Cargo.toml";
pub const GO_FILE: &str = "go.mod";
pub const PHP_FILE: &str = "composer.json";
pub const NODE_FILE: &str = "package.json";
pub const CMAKE_FILE: &str = "CMakeLists.txt";
pub const ELIXIR_FILE: &str = "mix.exs";
pub const RUBY_FILE: &str = "Gemfile";
pub const DART_FILE: &str = "pubspec.yaml";
pub const KOTLIN_FILE: &str = "build.gradle.kts";
pub const SWIFT_FILE: &str = "Package.swift";
pub const PYTHON_FILE: &str = "requirements.txt";
pub const GRADLE_KTS_FILE: &str = "build.gradle.kts";

pub const LANGUAGES: [(Language, &str); 14] = [
    (Language::Rust, RUST_FILE),
    (Language::CSharp, CS_PROJ),
    (Language::Java, MAVEN_POM),
    (Language::Go, GO_FILE),
    (Language::Ruby, RUBY_FILE),
    (Language::Dart, DART_FILE),
    (Language::Kotlin, KOTLIN_FILE),
    (Language::Swift, SWIFT_FILE),
    (Language::Php, PHP_FILE),
    (Language::Node, NODE_FILE),
    (Language::CMake, CMAKE_FILE),
    (Language::Elixir, ELIXIR_FILE),
    (Language::Gradle, GRADLE_KTS_FILE),
    (Language::Python, PYTHON_FILE),
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
            Self::CSharp => write!(f, "CSharp"),
            Self::Maven => write!(f, "Maven"),
            Self::Kotlin => write!(f, "Kotlin"),
            Self::Gradle => write!(f, "Gradle"),
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

impl Hook {
    pub fn get(language: Language) -> Vec<Hook> {
        let mut hooks: Vec<Hook> = vec![];
        match language {
            Language::Maven => {
                hooks.push(Self {
                    language: Language::Maven,
                    description: "Checking for outdated dependencies.",
                    success: "No outdated dependencies found",
                    failure: "Outdated dependencies found",
                    file: "outdated.log",
                    command: "mvn dependency:tree",
                });
                hooks.push(Self {
                    language: Language::Maven,
                    description: "Checking for security vulnerabilities.",
                    success: "No vulnerabilities found",
                    failure: "Vulnerabilities found",
                    file: "audit.log",
                    command: "mvn dependency-check:check",
                });
                hooks.push(Self {
                    language: Language::Maven,
                    description: "Running tests for your Maven project.",
                    success: "Tests passed",
                    failure: "Tests failed",
                    file: "test.log",
                    command: "mvn test",
                });
                hooks.push(Self {
                    language: Language::Maven,
                    description: "Checking for outdated packages in your project.",
                    success: "No outdated packages found",
                    failure: "Outdated packages found",
                    file: "outdated.log",
                    command: "mvn versions:display-dependency-updates",
                });
            }
            Language::Gradle => {
                hooks.push(Self {
                    language: Language::Gradle,
                    description: "Checking for outdated dependencies.",
                    success: "No outdated dependencies found",
                    failure: "Outdated dependencies found",
                    file: "outdated.log",
                    command: "gradle dependencyUpdates",
                });
                hooks.push(Self {
                    language: Language::Gradle,
                    description: "Checking for security vulnerabilities.",
                    success: "No vulnerabilities found",
                    failure: "Vulnerabilities found",
                    file: "audit.log",
                    command: "gradle dependencyCheckAnalyze",
                });
                hooks.push(Self {
                    language: Language::Gradle,
                    description: "Running tests for your Gradle project.",
                    success: "Tests passed",
                    failure: "Tests failed",
                    file: "test.log",
                    command: "gradle test",
                });
            }
            Language::Node => {
                hooks.push(Self {
                    language: Language::Node,
                    description: "Checking for outdated packages in your project.",
                    success: "No outdated packages found",
                    failure: "Outdated packages found",
                    file: "outdated.log",
                    command: "npm outdated",
                });
                hooks.push(Self {
                    language: Language::Node,
                    description: "Testings your project.",
                    success: "Tests passed",
                    failure: "Tests failed",
                    file: "test.log",
                    command: "npm run test",
                });
                hooks.push(Self {
                    language: Language::Node,
                    description: "Auditing your project.",
                    success: "No vulnerabilities founded",
                    failure: "Vulnerabilities founded",
                    file: "audit.log",
                    command: "npm audit",
                });
            }
            Language::Rust => {
                hooks.push(Self {
                    language: Language::Rust,
                    description: "Checking the configuration",
                    success: "Project is valid",
                    failure: "Project not valid",
                    file: "project.log",
                    command: "cargo verify-project",
                });
                hooks.push(Self {
                    language: Language::Rust,
                    description: "Checking build capability",
                    success: "Can build the project",
                    failure: "Cargo check detect failure",
                    file: "check.log",
                    command: "cargo check",
                });
                hooks.push(Self {
                    language: Language::Rust,
                    description: "Checking for security vulnerabilities",
                    success: "No vulnerabilities found",
                    failure: "Vulnerabilities found",
                    file: "audit.log",
                    command: "cargo audit",
                });
                hooks.push(Self {
                    language: Language::Rust,
                    description: "Checks for formatting issues in your Rust code.",
                    file: "fmt.log",
                    success: "Respect the code format standard",
                    failure: "Not respect code format standard",
                    command: "cargo fmt --check",
                });
                hooks.push(Self {
                    language: Language::Rust,
                    description: "Checks for linting issues and suggests code improvements.",
                    success: "No vulnerabilities found",
                    failure: "Vulnerabilities found",
                    file: "clippy.log",
                    command: "cargo clippy -- -D clippy::all -W warnings -D clippy::pedantic -D clippy::nursery -A clippy::multiple_crate_versions",
                });
                hooks.push(Self {
                    language: Language::Rust,
                    description: "Testing your project.",
                    success: "Tests passed",
                    failure: "Tests failed",
                    file: "test.log",
                    command: "cargo test --no-fail-fast",
                });
                hooks.push(Self {
                    language: Language::Rust,
                    description: "Generating documentation for your project.",
                    success: "Documentation generated",
                    failure: "Failed to generate documentation",
                    file: "doc.log",
                    command: "cargo doc --no-deps --document-private-items",
                });
                hooks.push(Self {
                    language: Language::Rust,
                    description: "Checking for outdated packages in your project.",
                    success: "No outdated packages found",
                    failure: "Outdated packages found",
                    file: "outdated.log",
                    command: "cargo outdated",
                });
            }
            Language::Java => {
                hooks.push(Self {
                    language: Language::Java,
                    description: "Checking for outdated packages in your project.",
                    success: "No outdated packages found",
                    failure: "Outdated packages found",
                    file: "outdated.log",
                    command: "mvn versions:display-dependency-updates",
                });
            }
            Language::Python => {
                hooks.push(Self {
                    language: Language::Python,
                    description: "Checking for outdated packages in your project.",
                    success: "No outdated packages found",
                    failure: "Outdated packages found",
                    file: "outdated.log",
                    command: "pip list --outdated",
                });
                hooks.push(Self {
                    language: Language::Python,
                    description: "Checking for security vulnerabilities.",
                    success: "No vulnerabilities found",
                    failure: "Vulnerabilities found",
                    file: "audit.log",
                    command: "pip audit",
                });
            }
            Language::Go => {
                hooks.push(Self {
                    language: Language::Go,
                    description: "Testing your project.",
                    success: "Tests passed",
                    failure: "Test failed",
                    file: "test.log",
                    command: "go test -v",
                });
                hooks.push(Self {
                    language: Language::Go,
                    description: "Checking for outdated packages.",
                    success: "No outdated packages found",
                    failure: "Outdated packages found",
                    file: "outdated.log",
                    command: "go list -u -m -json all",
                });
                hooks.push(Self {
                    language: Language::Go,
                    description: "Checking for security vulnerabilities.",
                    success: "No vulnerabilities found",
                    failure: "Vulnerabilities found",
                    file: "audit.log",
                    command: "go list -u -m -json all",
                });
            }
            Language::Php => {
                hooks.push(Self {
                    language: Language::Php,
                    description: "Checking platform requirements.",
                    success: "All requirements are met",
                    failure: "Missing requirements found",
                    file: "reqs.log",
                    command: "composer check-platform-reqs",
                });
                hooks.push(Self {
                    language: Language::Php,
                    description: "Checking for security vulnerabilities.",
                    success: "No vulnerabilities found",
                    failure: "Vulnerabilities found",
                    file: "audit.log",
                    command: "composer audit",
                });
                hooks.push(Self {
                    language: Language::Php,
                    description: "Checking outdated packages.",
                    success: "No outdated packages found",
                    failure: "Outdated packages found",
                    file: "outdated.log",
                    command: "composer outdated",
                });
                hooks.push(Self {
                    language: Language::Php,
                    description: "Running tests for your PHP project.",
                    success: "Tests passed",
                    failure: "Tests failed",
                    file: "test.log",
                    command: "composer run test",
                });
            }
            Language::Ruby => {
                hooks.push(Self {
                    language: Language::Ruby,
                    description: "Checking for outdated gems.",
                    success: "No outdated gems found",
                    failure: "Outdated gems found",
                    file: "outdated.log",
                    command: "bundle outdated",
                });
                hooks.push(Self {
                    language: Language::Ruby,
                    description: "Checking for security vulnerabilities.",
                    success: "No vulnerabilities found",
                    failure: "Vulnerabilities found",
                    file: "audit.log",
                    command: "bundle audit",
                });
                hooks.push(Self {
                    language: Language::Ruby,
                    description: "Running tests for your Ruby project.",
                    success: "Tests passed",
                    failure: "Tests failed",
                    file: "test.log",
                    command: "bundle exec rspec",
                });
            }
            Language::CMake => {
                hooks.push(Self {
                    language: Language::CMake,
                    description: "Validating CMake configuration files.",
                    success: "CMake configuration is valid",
                    failure: "CMake configuration validation failed",
                    file: "cmake.log",
                    command: "cmake .",
                });
                hooks.push(Self {
                    language: Language::CMake,
                    description: "Validating CMake configuration files.",
                    success: "Build success",
                    failure: "Build failed",
                    file: "make.log",
                    command: "make",
                });
                hooks.push(Self {
                    language: Language::CMake,
                    description: "Validating CMake configuration files.",
                    success: "Test success",
                    failure: "test failures",
                    file: "test.log",
                    command: "make test",
                });
            }
            Language::CSharp => {
                hooks.push(Self {
                    language: Language::CSharp,
                    description: "Checking for code formatting",
                    success: "Code formatting is correct",
                    failure: "Code formatting issues found",
                    file: "format.log",
                    command: "dotnet format --verify-no-changes",
                });
                hooks.push(Self {
                    language: Language::CSharp,
                    description: "Running unit tests",
                    success: "All tests passed",
                    failure: "Some tests failed",
                    file: "test.log",
                    command: "dotnet test",
                });
                hooks.push(Self {
                    language: Language::CSharp,
                    description: "Building the project",
                    success: "Build successful",
                    failure: "Build failed",
                    file: "build.log",
                    command: "dotnet build",
                });
                hooks.push(Self {
                    language: Language::CSharp,
                    description: "Checking for dependency updates",
                    success: "Dependencies are up to date",
                    failure: "Dependency updates available",
                    file: "deps.log",
                    command: "dotnet restore",
                });
                hooks.push(Self {
                    language: Language::CSharp,
                    description: "Checking for security vulnerabilities",
                    success: "No vulnerabilities found",
                    failure: "Vulnerabilities found",
                    file: "audit.log",
                    command: "dotnet audit",
                });
            }
            Language::Kotlin => {
                hooks.push(Self {
                    language: Language::Kotlin,
                    description: "Checking for code formatting",
                    success: "Code formatting is correct",
                    failure: "Code formatting issues found",
                    file: "format.log",
                    command: "ktlint --reporter=plain",
                });
                hooks.push(Self {
                    language: Language::Kotlin,
                    description: "Running unit tests",
                    success: "All tests passed",
                    failure: "Some tests failed",
                    file: "test.log",
                    command: "kotlinc -script test.kts",
                });
            }
            Language::Swift => {
                hooks.push(Self {
                    language: Language::Swift,
                    description: "Checking for code formatting",
                    success: "Code formatting is correct",
                    failure: "Code formatting issues found",
                    file: "format.log",
                    command: "swiftformat --lint .",
                });
                hooks.push(Self {
                    language: Language::Swift,
                    description: "Running unit tests",
                    success: "All tests passed",
                    failure: "Some tests failed",
                    file: "test.log",
                    command: "swift test",
                });
                hooks.push(Self {
                    language: Language::Swift,
                    description: "Checking for security vulnerabilities",
                    success: "No vulnerabilities found",
                    failure: "Vulnerabilities found",
                    file: "audit.log",
                    command: "swift package audit",
                });
                hooks.push(Self {
                    language: Language::Swift,
                    description: "Building the project",
                    success: "Build successful",
                    failure: "Build failed",
                    file: "build.log",
                    command: "swift build",
                });
                hooks.push(Self {
                    language: Language::Swift,
                    description: "Running integration tests",
                    success: "All integration tests passed",
                    failure: "Some integration tests failed",
                    file: "integration.log",
                    command: "swift test --parallel",
                });
            }
            Language::Dart => {
                hooks.push(Self {
                    language: Language::Dart,
                    description: "Checking for code formatting",
                    success: "Code formatting is correct",
                    failure: "Code formatting issues found",
                    file: "format.log",
                    command: "dart format --set-exit-if-changed",
                });
                hooks.push(Self {
                    language: Language::Dart,
                    description: "Running unit tests",
                    success: "All tests passed",
                    failure: "Some tests failed",
                    file: "test.log",
                    command: "dart test",
                });
                hooks.push(Self {
                    language: Language::Dart,
                    description: "Running integration tests",
                    success: "All integration tests passed",
                    failure: "Some integration tests failed",
                    file: "integration.log",
                    command: "dart test --integration",
                });
                hooks.push(Self {
                    language: Language::Dart,
                    description: "Checking for security vulnerabilities",
                    success: "No vulnerabilities found",
                    failure: "Vulnerabilities found",
                    file: "audit.log",
                    command: "dart pub audit",
                });
                hooks.push(Self {
                    language: Language::Dart,
                    description: "Building the project",
                    success: "Build successful",
                    failure: "Build failed",
                    file: "build.log",
                    command: "dart compile exe bin/main.dart",
                });
            }
            Language::Elixir => {
                hooks.push(Self {
                    language: Language::Elixir,
                    description: "Checking for code formatting",
                    success: "Code formatting is correct",
                    failure: "Code formatting issues found",
                    file: "format.log",
                    command: "mix format --check-formatted",
                });
                hooks.push(Self {
                    language: Language::Elixir,
                    description: "Running unit tests",
                    success: "All tests passed",
                    failure: "Some tests failed",
                    file: "test.log",
                    command: "mix test",
                });
                hooks.push(Self {
                    language: Language::Elixir,
                    description: "Running integration tests",
                    success: "All integration tests passed",
                    failure: "Some integration tests failed",
                    file: "integration.log",
                    command: "mix test --integration",
                });
                hooks.push(Self {
                    language: Language::Elixir,
                    description: "Checking for security vulnerabilities",
                    success: "No vulnerabilities found",
                    failure: "Vulnerabilities found",
                    file: "audit.log",
                    command: "mix audit",
                });
                hooks.push(Self {
                    language: Language::Elixir,
                    description: "Building the project",
                    success: "Build successful",
                    failure: "Build failed",
                    file: "build.log",
                    command: "mix compile",
                });
            }
        }
        hooks
    }
}
