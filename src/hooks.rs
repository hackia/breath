use std::fmt::{Display, Formatter};
use tabled::Tabled;
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
pub const TYPESCRIPT_FILE: &str = "tsconfig.json";
pub const HASKELL_FILE: &str = "*.cabal";
pub const D_FILE: &str = "dub.*";

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Tabled)]
pub enum Language {
    Unknown,
    Javascript,
    Typescript,
    Haskell,
    D,
    Rust,
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
        if value.eq("Javascript") {
            return Self::Javascript;
        }
        if value.eq("Typescript") {
            return Self::Typescript;
        }
        if value.eq("Rust") {
            return Self::Rust;
        }
        if value.eq("Python") {
            return Self::Python;
        }
        if value.eq("Go") {
            return Self::Go;
        }
        if value.eq("Php") {
            return Self::Php;
        }
        if value.eq("Ruby") {
            return Self::Ruby;
        }
        if value.eq("CMake") {
            return Self::CMake;
        }
        if value.eq("CSharp") {
            return Self::CSharp;
        }
        if value.eq("Maven") {
            return Self::Maven;
        }
        if value.eq("Kotlin") {
            return Self::Kotlin;
        }
        if value.eq("Gradle") {
            return Self::Gradle;
        }
        if value.eq("Swift") {
            return Self::Swift;
        }
        if value.eq("Dart") {
            return Self::Dart;
        }
        if value.eq("Elixir") {
            return Self::Elixir;
        }
        if value.eq("D") {
            return Self::D;
        }
        if value.eq("Haskell") {
            return Self::Haskell;
        }
        Self::Unknown
    }
}

impl Language {
    #[must_use]
    pub const fn get_file(language: Self) -> &'static str {
        match language {
            Self::Javascript => NODE_FILE,
            Self::Typescript => TYPESCRIPT_FILE,
            Self::Haskell => HASKELL_FILE,
            Self::Rust => RUST_FILE,
            Self::Python => PYTHON_FILE,
            Self::Go => GO_FILE,
            Self::Php => PHP_FILE,
            Self::Ruby => RUBY_FILE,
            Self::CMake => CMAKE_FILE,
            Self::CSharp => CS_PROJ,
            Self::Maven => MAVEN_POM,
            Self::Kotlin => KOTLIN_FILE,
            Self::Gradle => GRADLE_BUILD,
            Self::Swift => SWIFT_FILE,
            Self::Dart => DART_FILE,
            Self::Elixir => ELIXIR_FILE,
            Self::D => D_FILE,
            Self::Unknown => "",
        }
    }
}

pub const LANGUAGES: [(Language, &str); 17] = [
    (Language::Rust, RUST_FILE),
    (Language::Typescript, TYPESCRIPT_FILE),
    (Language::Haskell, HASKELL_FILE),
    (Language::D, D_FILE),
    (Language::Javascript, NODE_FILE),
    (Language::CSharp, CS_PROJ),
    (Language::Maven, MAVEN_POM),
    (Language::Go, GO_FILE),
    (Language::Ruby, RUBY_FILE),
    (Language::Dart, DART_FILE),
    (Language::Gradle, GRADLE_BUILD),
    (Language::Kotlin, KOTLIN_FILE),
    (Language::Swift, SWIFT_FILE),
    (Language::Php, PHP_FILE),
    (Language::CMake, CMAKE_FILE),
    (Language::Elixir, ELIXIR_FILE),
    (Language::Python, PYTHON_FILE),
];
impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Javascript => write!(f, "Javascript"),
            Self::Typescript => write!(f, "Typescript"),
            Self::Rust => write!(f, "Rust"),
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
            Self::D => write!(f, "D"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Haskell => write!(f, "Haskell"),
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
    pub fn d(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::D,
            description: "Building your project",
            success: "Build successful",
            failure: "Build failed",
            file: "build.log",
            command: "dub build",
        });
        hooks.push(Self {
            language: Language::D,
            description: "Testing your project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "dub test",
        });
        hooks.push(Self {
            language: Language::D,
            description: "Checking format",
            success: "Format check passed",
            failure: "Format check failed",
            file: "format.log",
            command: "dub format --check",
        });
        hooks.push(Self {
            language: Language::D,
            description: "Checking for outdated packages",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "dub outdated",
        });
        hooks.push(Self {
            language: Language::D,
            description: "Generating documentation for your project",
            success: "Documentation generated successfully",
            failure: "Failed to generate documentation",
            file: "docs.log",
            command: "dub docs",
        });
    }
    pub fn haskell(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Haskell,
            description: "Checking for outdated packages in your project",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "cabal outdated",
        });
        hooks.push(Self {
            language: Language::Haskell,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "cabal audit",
        });
        hooks.push(Self {
            language: Language::Haskell,
            description: "Running tests for your Haskell project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "cabal test",
        });
    }
    pub fn typescript(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Typescript,
            description: "Testing your project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "npm run test",
        });
        hooks.push(Self {
            language: Language::Typescript,
            description: "Auditing your project",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "npm audit",
        });
        hooks.push(Self {
            language: Language::Typescript,
            description: "Checking for outdated packages in your project",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "npm outdated",
        });
    }
    pub fn maven(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Maven,
            description: "Checking for outdated dependencies",
            success: "No outdated dependencies found",
            failure: "Outdated dependencies found",
            file: "outdated.log",
            command: "mvn dependency:tree",
        });
        hooks.push(Self {
            language: Language::Maven,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "mvn dependency-check:check",
        });
        hooks.push(Self {
            language: Language::Maven,
            description: "Running tests for your Maven project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "mvn test",
        });
        hooks.push(Self {
            language: Language::Maven,
            description: "Checking for outdated packages in your project",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "mvn versions:display-dependency-updates",
        });
    }
    pub fn gradle(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Gradle,
            description: "Checking for outdated dependencies",
            success: "No outdated dependencies found",
            failure: "Outdated dependencies found",
            file: "outdated.log",
            command: "gradle dependencyUpdates",
        });
        hooks.push(Self {
            language: Language::Gradle,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "gradle dependencyCheckAnalyze",
        });
        hooks.push(Self {
            language: Language::Gradle,
            description: "Running tests for your Gradle project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "gradle test",
        });
    }

    pub fn javascript(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Javascript,
            description: "Checking for outdated packages in your project",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "npm outdated",
        });
        hooks.push(Self {
            language: Language::Javascript,
            description: "Testing your project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "npm run test",
        });
        hooks.push(Self {
            language: Language::Javascript,
            description: "Auditing your project",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "npm audit",
        });
    }
    pub fn rust(hooks: &mut Vec<Self>) {
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
            description: "Checks for formatting issues in your Rust code",
            file: "fmt.log",
            success: "Code format standard respected",
            failure: "Code format standard not respected",
            command: "cargo fmt --check",
        });
        hooks.push(Self {
            language: Language::Rust,
            description: "Checks for linting issues and suggests code improvements",
            success: "No warning founded",
            failure: "Warnings founded",
            file: "clippy.log",
            command: "cargo clippy -- -D clippy::all -W warnings -D clippy::pedantic -D clippy::nursery -A clippy::multiple_crate_versions",
        });
        hooks.push(Self {
            language: Language::Rust,
            description: "Testing your project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "cargo test --no-fail-fast",
        });
        hooks.push(Self {
            language: Language::Rust,
            description: "Generating documentation for your project",
            success: "Documentation generated",
            failure: "Failed to generate documentation",
            file: "doc.log",
            command: "cargo doc --no-deps --document-private-items",
        });
        hooks.push(Self {
            language: Language::Rust,
            description: "Checking for outdated packages in your project",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "cargo outdated",
        });
    }

    pub fn python(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Python,
            description: "Checking for outdated packages in your project",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "pip list --outdated",
        });
        hooks.push(Self {
            language: Language::Python,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "pip audit",
        });
    }
    pub fn go(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Go,
            description: "Testing your project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "go test -v",
        });
        hooks.push(Self {
            language: Language::Go,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "go list -u -m -json all",
        });
    }
    pub fn php(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Php,
            description: "Checking platform requirements",
            success: "All requirements are met",
            failure: "Missing requirements found",
            file: "reqs.log",
            command: "composer check-platform-reqs",
        });
        hooks.push(Self {
            language: Language::Php,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "composer audit",
        });
        hooks.push(Self {
            language: Language::Php,
            description: "Checking outdated packages",
            success: "No outdated packages found",
            failure: "Outdated packages found",
            file: "outdated.log",
            command: "composer outdated",
        });
        hooks.push(Self {
            language: Language::Php,
            description: "Running tests for your PHP project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "composer run test",
        });
    }

    pub fn ruby(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Ruby,
            description: "Checking for outdated gems",
            success: "No outdated gems found",
            failure: "Outdated gems found",
            file: "outdated.log",
            command: "bundle outdated",
        });
        hooks.push(Self {
            language: Language::Ruby,
            description: "Checking for security vulnerabilities",
            success: "No vulnerabilities found",
            failure: "Vulnerabilities found",
            file: "audit.log",
            command: "bundle audit",
        });
        hooks.push(Self {
            language: Language::Ruby,
            description: "Running tests for your Ruby project",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "bundle exec rspec",
        });
    }

    pub fn cmake(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::CMake,
            description: "Generate Makefile",
            success: "Makefile generation success.",
            failure: "Makefile generation failed",
            file: "cmake.log",
            command: "cmake .",
        });
        hooks.push(Self {
            language: Language::CMake,
            description: "Building",
            success: "Build success",
            failure: "Build failed",
            file: "make.log",
            command: "make",
        });
        hooks.push(Self {
            language: Language::CMake,
            description: "Testing",
            success: "Tests passed",
            failure: "Tests failed",
            file: "test.log",
            command: "make test",
        });
    }

    pub fn csharp(hooks: &mut Vec<Self>) {
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

    pub fn swift(hooks: &mut Vec<Self>) {
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
    pub fn dart(hooks: &mut Vec<Self>) {
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
    pub fn elixir(hooks: &mut Vec<Self>) {
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
            description: "Generating documentation",
            success: "Documentation generated successfully",
            failure: "Documentation generation failed",
            file: "docs.log",
            command: "mix docs",
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

    fn kotlin(hooks: &mut Vec<Self>) {
        hooks.push(Self {
            language: Language::Kotlin,
            description: "",
            success: "",
            failure: "",
            file: "",
            command: "",
        });
    }
    #[must_use]
    pub fn get(language: Language) -> Vec<Self> {
        let mut hooks: Vec<Self> = vec![];
        match language {
            Language::Unknown => {}
            Language::Typescript => Self::typescript(&mut hooks),
            Language::D => Self::d(&mut hooks),
            Language::Haskell => Self::haskell(&mut hooks),
            Language::Maven => Self::maven(&mut hooks),
            Language::Gradle => Self::gradle(&mut hooks),
            Language::Javascript => Self::javascript(&mut hooks),
            Language::Rust => Self::rust(&mut hooks),
            Language::Python => Self::python(&mut hooks),
            Language::Go => Self::go(&mut hooks),
            Language::Php => Self::php(&mut hooks),
            Language::Ruby => Self::ruby(&mut hooks),
            Language::CMake => Self::cmake(&mut hooks),
            Language::CSharp => Self::csharp(&mut hooks),
            Language::Swift => Self::swift(&mut hooks),
            Language::Dart => Self::dart(&mut hooks),
            Language::Elixir => Self::elixir(&mut hooks),
            Language::Kotlin => Self::kotlin(&mut hooks),
        }
        hooks
    }
}
