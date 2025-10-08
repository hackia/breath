# Breath

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Crates.io](https://img.shields.io/crates/v/breath.svg)](https://crates.io/crates/breath)

A fast, interactive CLI that streamlines committing code to Git or Mercurial repositories.
It can also run language-aware checks before you commit (format, lint, tests, security, deps) and stores
logs per language under a .breathes directory.

VCS detection: Breath auto-detects the repository type at runtime by checking for .git or .hg.
No Cargo features are required to switch between Git and Mercurial.

| Category                      | Commit Type                | Mnemonic                                                                                           | Description                                              | Example                                                                               |
|-------------------------------|----------------------------|----------------------------------------------------------------------------------------------------|----------------------------------------------------------|---------------------------------------------------------------------------------------|
| Core Changes                  | Star                       | Shiny Technology Added or Refined                                                                  | New feature or enhancement                               | Star(Auth): Implement two-factor authentication                                       |
|                               | Comet                      | Code or Module Error Terminated                                                                    | Bug fix or error resolution                              | Comet(UI): Fix responsive layout issue on mobile devices                              |
|                               | Nebula                     | New Efficient Better Understandable Logic Achieved                                                 | Code refactoring                                         | Nebula(Backend): Refactor user management module for improved maintainability         |
|                               | Pulsar                     | Powerful Upgrade, Less Sluggish, Agile Response                                                    | Performance improvement                                  | Pulsar(Database): Optimize queries for faster response times                          |
|                               | Quasar                     | Quick Adjustments for Superior Accuracy and Readability                                            | Documentation or clarity improvement                     | Quasar(API): Update documentation with new endpoint parameters                        |
| Maintenance & Infrastructure  | Asteroid Belt              | Adjustments, Sweeps, Tidy-ups, Elimination, Reordering of Items, Decrease Bloat                    | Code cleanup and maintenance                             | Asteroid Belt: Remove unused CSS and optimize images                                  |
|                               | Solar Flare                | Securing Our Logic Against Regressions, Failures, and Latencies Actively, Rigorously Ensured       | Adding or updating tests (unit, integration, end-to-end) | Solar Flare(Payments): Add unit tests for payment processing module                   |
|                               | Dwarf Planet               | Details Warranted Attention, Refined Further, Polished Little Aspects Neatly Enhanced              | Tiny Minor but essential updates or fixes                | Dwarf Planet: Update project dependencies to latest versions                          |
|                               | Terraform                  | Technology Engineering Resources Readily Automated, Foundation of Reliable Management              | Infrastructure changes                                   | Terraform(AWS): Provision new EC2 instance for staging environment                    |
| Project Events                | Black Hole                 | Big Legacy Aspects Consumed, Killing Heavy, Old Loads Entirely                                     | Removing large chunks of code or features                | Black Hole: Remove deprecated user profile module                                     |
|                               | Wormhole                   | Weaving or Reconnecting Modules, Hitching onto Linked Elements                                     | Merging branches or connecting code parts                | Wormhole: Merge feature/new-dashboard into develop branch                             |
|                               | Big Bang                   | Birth of Initial Greatness, Beginning All New Growth                                               | Initial commit of a project or major feature             | Big Bang: Initial project setup and scaffolding                                       |
|                               | Launch                     | Lifting Application Upward, New Code Entering Production                                           | Deploying to production or releasing a version           | Launch(v1.2): Release new version with user profile customization                     |
| Communication & Collaboration | Lightspeed                 | Lightening Speed Enhancements                                                                      | Significant performance improvements                     | Lightspeed(Frontend): Implement lazy loading for images                               |
|                               | Mission Control            | Managing Changes, Issues, Scope, Teamwork, and Release On Time                                     | Project management changes                               | Mission Control: Update project roadmap and assign tasks for Q3                       |
|                               | Spacewalk                  | Swift Work Above Limits, Keeping All Systems Extra Safe                                            | Urgent hotfixes or critical production updates           | Spacewalk(Security): Patch critical vulnerability in authentication module            |
|                               | Moon Landing               | Major Leaps Over Night, New Doors and Incredible Achievements                                      | Completing major milestones or goals                     | Moon Landing: Successfully launch beta version to select users                        |
|                               | First Contact              | Forge Initial Connections, Open New Territories                                                    | Establishing initial connections or integrations         | First Contact(API): Integrate with new payment provider's API                         |
|                               | Interstellar Communication | Informing, Sharing, Teaching, Educating, & Learning Lucidly & Clearly                              | Improving documentation or communication                 | Interstellar Communication: Update wiki with troubleshooting guide for common errors  |
| Celestial Events              | Solar Eclipse              | Sun Escapes, Legacy Code Lurks                                                                     | Temporarily masking functionality                        | Solar Eclipse(Feature): Temporarily disable new onboarding flow for testing           |
|                               | Supernova                  | Sudden Unbelievable Performance Revolution, New Version Arrives                                    | Major, transformative change or improvement              | Supernova(Architecture): Migrate to microservices architecture                        |
|                               | Meteor Shower              | Many Edits, Tiny Overall Result, Overhaul Routines                                                 | Series of small changes or fixes                         | Meteor Shower: Fix multiple typos in error messages                                   |
|                               | Cosmic Dawn                | Creating Original, Simple, Minimal Initial Draft                                                   | Initial implementation of a feature                      | Cosmic Dawn(Search): Initial implementation of basic search functionality             |
|                               | Solar Storm                | Sudden Transformations Occur Rapidly, Modifications                                                | Rapid, impactful changes                                 | Solar Storm(Refactor): Overhaul data processing pipeline for improved performance     |
|                               | Lunar Transit              | Little Update, Now Adjustments Require Testing                                                     | Minor, temporary change                                  | Lunar Transit(Config): Temporarily adjust logging level for debugging                 |
|                               | Perihelion                 | Perfect Ending, Refined, Improved, High Efficiency, Low Obstacles, Near Goal                       | Significant milestone or feature completion              | Perihelion: Successfully complete user acceptance testing for new dashboard           |
|                               | Aphelion                   | Away From Perfection, High Effort, Long Overhaul, Intense Overhaul, Obstacles                      | Refactor, dependency update, or architecture change      | Aphelion: Upgrade to React 18 and refactor components                                 |
| Celestial Objects             | White Dwarf                | Writing, Improving, Detailed Documentation For All                                                 | Improving code comments or documentation                 | White Dwarf(API): Add detailed documentation for new endpoints                        |
|                               | Red Giant                  | Refactoring, Enhancing, Growing, Increasing, Adding New Things                                     | Expanding a feature or functionality                     | Red Giant(Payments): Add support for Apple Pay and Google Pay                         |
|                               | Neutron Star               | New Efficient Utility, Tweaks, Robust Optimization, Nimble Solution                                | Optimizing code for performance                          | Neutron Star(Search): Optimize search algorithm for faster results                    |
|                               | Binary Star                | Bringing In New And Revised, Yielding Integrated Results                                           | Merging features or components                           | Binary Star: Merge user authentication and authorization modules                      |
|                               | Brown Dwarf                | Barely Developed, Requires Work, Ongoing Development For Future                                    | Undeveloped feature with potential                       | Brown Dwarf(Social): Initial prototype for social sharing feature                     |
|                               | Quark Star                 | Questionable, Unstable, Anticipated Results, Risky, Keen Experiment                                | Experimental or speculative change                       | Quark Star(AI): Experiment with integrating GPT-3 for content generation              |
|                               | Rogue Planet               | Refactoring Or Generating Operations, Unique Path, Leaping Ahead                                   | Independent change unrelated to the main codebase        | Rogue Planet: Create standalone script for data migration                             |
|                               | Stellar Nursery            | Starting To Enhance, Laying Layers, Launching New Requirements                                     | Creating new components                                  | Stellar Nursery(UI): Add new component library for design system                      |
|                               | Planetary Nebula           | Pruning, Leaving, Abandoning, Nostalgic Era, Totally Removed                                       | Removal or deprecation of a component                    | Planetary Nebula: Remove legacy image carousel component                              |
|                               | Globular Cluster           | Gathering, Linking, Operations, Bringing Unity, Lots of Adjustments, All Related                   | Collection of related changes                            | Globular Cluster(Refactor): Refactor multiple API endpoints for consistency           |
|                               | Void                       | Vanished, Obliterated, Irrelevant, Deleted                                                         | Removal of a module, component, or feature               | Void: Remove unused user settings module                                              |
| Astronomical Concepts         | Gravity                    | Glitch Resolution, Adjusting Versions, Integrating, Troubleshooting Yielding                       | Resolving merge conflicts or dependencies                | Gravity: Resolve merge conflicts in feature/new-navigation branch                     |
|                               | Dark Matter                | Debugging And Resolving Mysterious Attributes, Tricky issues Removed                               | Fixing unknown or mysterious bugs                        | Dark Matter: Fix intermittent crash on user login                                     |
|                               | Time Dilation              | Time Is Dilated, Improvements Leverage Agility, Time-Saving                                        | Improving code performance or reducing execution time    | Time Dilation(Backend): Optimize image processing algorithm for faster response       |
|                               | Spacetime                  | Scheduling, Planning, Adjusting Calendar Events, Coordinating Time                                 | Changes to date, time, or scheduling                     | Spacetime(API): Fix timezone handling for event timestamps                            |
|                               | Gravitational Lensing      | Gravity Redirects Light, Altering Information Pathways                                             | Altering data or information flow                        | Gravitational Lensing(Data): Refactor data pipeline for improved throughput           |
|                               | Cosmic String              | Connecting Our Sections, Merging Together, Interlinking New Groups                                 | Connecting code parts                                    | Cosmic String(API): Connect user service with authentication middleware               |
|                               | Quantum Fluctuation        | Quick Unpredictable Adjustments, Noticed Tiny Unexpected Modification                              | Small, random change                                     | Quantum Fluctuation: Fix typo in error message                                        |
|                               | Hawking Radiation          | Hastily And Willingly Killing Redundancies, Ageing Dead-ends, Tidying In Order, Obliterating Noise | Removing technical debt                                  | Hawking Radiation: Remove unused CSS classes and refactor styles                      |
|                               | Quantum Entanglement       | Quantum Effects Never Tangled, Greater Efficiency, Linked Adjustments                              | Establishing close relationships between code parts      | Quantum Entanglement(API): Tightly couple user profile and order history endpoints    |
|                               | Gravitational Redshift     | Gravity Reduces Efficiency, Degraded Speed, Shift Happens                                          | Slowing down or reducing code performance                | Gravitational Redshift(UI): Disable unnecessary animations for low-end devices        |
| Space Exploration             | Space Probe                | Surveying, Planning, Analysing, Checking Every Nook                                                | Testing new features or technologies                     | Space Probe(AI): Experiment with ChatGPT integration for customer support             |
|                               | Space Station              | Setting Up The Area, Testing In Orbit, Optimising New                                              | Creating or improving environments                       | Space Station(DevOps): Set up new development environment with Docker                 |
|                               | Rocket Launch              | Releasing Our Code, Keenly Entering The Production                                                 | Deploying to production                                  | Rocket Launch(v1.5): Deploy new version to production with enhanced security features |
|                               | Spacewalk                  | Swift Patches And Lookout Work, Keeping Systems Extra safe                                         | Urgent production hotfixes                               | Spacewalk(Database): Fix critical database connection issue causing downtime          |
|                               | Space Elevator             | Streamlined Access, Providing Easy Vertical On boarding, Lifting Entries                           | Making code base more accessible                         | Space Elevator(API): Add new public API endpoints for third-party integrations        |

## Overview

Breath helps you:
- review the current diff and status;
- compose a guided commit message (templates with type, scope, summary);
- optionally push to remotes;
- run pre-commit hooks for common ecosystems when their config files are detected (Rust, Node.js, PHP, Go, Java, .NET/C#, CMake, etc.).

Under the hood it uses:
- Rust + Cargo as the build system;
- crossterm and inquire for the TUI/interactive prompts;
- simple process spawning to run the underlying tools for each language.

## Stack and entry points
- Language: Rust (edition 2024)
- Package manager/build tool: Cargo
- Binary crate name: breath
- Entry point: src/main.rs
- VCS: auto-detected at runtime by checking for .git or .hg
- Subcommands: config, health, commit, push, pull, status, log, diff

## Requirements
- Rust toolchain with edition 2024 support (Rust 1.82+ recommended)
- One of (depending on your repository):
  - Git (if using a Git repo)
  - Mercurial/hg (if using a Mercurial repo)
- Optional tools used by hooks (only needed if the corresponding project files exist):
  - Rust: rustfmt, clippy (rustup components), cargo-audit, cargo-outdated
  - Node.js: npm
  - PHP: composer
  - Go: go toolchain
  - Java: Maven (mvn)
  - .NET/C#: dotnet
  - CMake: cmake and make

If a language’s tool is not installed but its marker file exists (e.g., Cargo.toml), the related hook will fail. Install the tool or remove/ignore the marker.

## Installation

From crates.io:

```sh
cargo install breath
```

From source (in this repo):

```sh
cargo build --release
```

## Usage

Breath works the same in Git and Mercurial repositories. It auto-detects the VCS.

- Interactive commit flow:

```sh
breath commit
# or simply `breath` to run the default interactive flow (commit in detected VCS)
```

- Health checks (run language-aware hooks if their marker files exist):

```sh
breath health
```

- Configure your Git or Mercurial setup:

```sh
breath config git  # prompts for user.name, user.email, core.editor
breath config hg   # prints instructions to edit hg config
```

- Convenience passthroughs to the underlying VCS:

```sh
breath status
breath log
breath diff
breath push
breath pull
```

Example pre-commit hook:

```sh
#!/bin/sh
# .git/hooks/pre-commit (make sure it is executable)

breath && exit 0 || exit 1
```

## Hooks and logs

Breath runs language-specific hooks based on the presence of well-known files in your repository:
- Rust: Cargo.toml → runs cargo fmt --check, clippy, test --no-fail-fast, audit, outdated
- Node.js: package.json → runs npm outdated, npm run test
- PHP: composer.json → runs composer outdated, composer security-check, php-cs-fixer (dry run), composer run test
- Go: go.mod → runs go fmt -x, go vet ./..., go test ./..., go mod tidy, go build
- Java: build.gradle or Maven project (see hooks.rs) → runs mvn clean compile, test, spotbugs, dependency updates
- C#: *.csproj → dotnet format --verify-no-changes, build, test, analyze, restore
- CMake: CMakeLists.txt → cmake build . && make . && make test

Outputs are stored under .breathes/<Language>/{stdout,stderr}/<hook>.log. On failure, Breath prints the captured logs for quick diagnosis.

## Scripts and common commands
- Build: cargo build --release
- Run: cargo run
- Install locally from source: cargo install --path .
- Show help: breath --help

## Environment variables
- None required at this time.
- TODO: Document any future env vars to control timeouts, spinner behavior, or disabling certain hooks.

## Tests
- There are currently no automated tests in this repository.
- To run tests when they are added: cargo test
- TODO: Add unit/integration tests for the interactive flow and hook runner.

## Project structure

```
.
├── Cargo.toml           # crate metadata (name, version, license, deps)
├── src
│   ├── main.rs          # CLI entry point and subcommands; auto-detects Git/Hg
│   ├── git.rs           # Git workflow: diff, commit, optional push
│   ├── hg.rs            # Mercurial workflow: diff, commit, optional push
│   ├── utils.rs         # hooks runner, spinner UI, COMMIT_MESSAGE template
│   ├── hooks.rs         # per-language hook definitions
│   └── commit.rs        # commit type catalogue for the interactive menu
├── README.md
└── target/              # build artifacts (ignored)
```

## License

AGPL-3.0. See the license badge above and Cargo.toml.

## Acknowledgements

Created by [hackia](https://github.com/hackia). Contributions and feedback are welcome.
