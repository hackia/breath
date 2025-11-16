<div align="center">

![breath](https://raw.githubusercontent.com/otechdo/zuu/refs/heads/main/zuu.png)

[![Breath](https://github.com/hackia/breath/actions/workflows/breath.yml/badge.svg)](https://github.com/hackia/breath/actions/workflows/breath.yml) ![Version](https://img.shields.io/crates/v/breath.svg) ![License](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)

**Windows** / **Linux** / **Bsd** /  **Mac OS X**

</div>

# What is Breath?

Breath is an advanced developer companion tool meticulously crafted to streamline the process of creating structured and
high-quality commit messages while simultaneously upholding stringent code health standards. This innovative solution is
engineered for seamless integration into a developer's daily workflow, extending its utility far beyond that of a
rudimentary commit helper. Breath acts as an intelligent assistant, guiding developers through the creation of clear,
concise, and informative commit messages that adhere to predefined organizational or project-specific guidelines.

Beyond its core function of commit message generation, Breath plays a crucial role in maintaining overall code quality.
It actively analyzes code changes against established best practices, identifying potential issues before they are
committed to the codebase. This proactive approach helps prevent the introduction of technical debt and ensures that the
codebase remains robust and maintainable over time. By automating adherence to code health standards, Breath empowers
development teams to consistently deliver high-quality software. Its comprehensive feature set makes it an indispensable
asset for individual developers and large teams alike, fostering a culture of disciplined development and continuous
improvement.

Core Features:

1. VCS Agnostic Compatibility: Breath offers exceptional flexibility by automatically detecting the version control
   system in use within your project. It intelligently identifies whether your repository utilizes Git or Mercurial (hg)
   by checking for the presence of .git or .hg directories at the project root. This ensures that Breath can adapt to
   diverse development environments without requiring manual configuration.

2. Seamless VCS Command Proxying: To enhance convenience and maintain a unified interface, Breath provides a set of
   intuitive commands that proxy directly to the underlying version control system. Commands such as breath status,
   breath log, breath diff, breath push, and breath pull are accurately translated and executed, allowing developers to
   manage their VCS operations through a consistent Breath interface.

3. Guided Interactive Commits: Moving beyond the simplicity of a commit -m command, breath commit initiates an
   interactive wizard designed to guide the user through the creation of well-structured commit messages. This wizard
   leverages the inquire crate to present clear and concise prompts, soliciting structured input from the user. This
   includes critical elements such as a designated commit "type" and a comprehensive "summary," ensuring that each
   commit is descriptive and informative.

4. Expressive, Space-Themed Typology: Breath introduces a unique and memorable space-themed commit typology to foster
   more descriptive and organized commit histories. This innovative approach assigns distinct "types" to commits, such
   as "Star" for new features, "Comet" for bug fixes, and "Nebula" for refactoring efforts. This expressive typology not
   only aids in classifying commit intent but also makes for a more engaging and easily navigable project history.

5. Automatic Pre-Commit Health Checks: The breath health command serves as a critical quality gate, meticulously
   integrated into the commit flow to ensure code integrity. Upon execution, or as part of the commit process, Breath
   automatically detects the project's technology stack. This is achieved by scanning for characteristic marker files (
   e.g., Cargo.toml for Rust, package.json for Node.js, etc.). Based on this detection, Breath then runs a series of
   predefined hooks, which can include crucial tasks such as code formatting, linting, comprehensive testing, and
   thorough security audits. The outputs of these hooks are meticulously logged, providing invaluable information for
   debugging and ensuring that any potential issues are identified and addressed before code is committed. This robust
   pre-commit health check system significantly elevates code quality and reduces the likelihood of introducing
   regressions.

## Why Choose Breath?

In a crowded landscape of developer tools, Breath distinguishes itself as an all-encompassing solution that elevates
both individual developer efficiency and overall team productivity.
Here's why Breath is the indispensable companion your development workflow needs:

- Enforce Code Quality Proactively: Unlike tools that merely report issues after they've been introduced, Breath acts as
  a proactive guardian. Its automatic pre-commit health checks catch formatting errors, linting violations, and even
  security vulnerabilities before code ever reaches your repository. This significantly reduces technical debt and
  ensures a consistently high-quality codebase.

- Standardize Commit Messages Effortlessly: Inconsistent commit messages can quickly turn a project's history into an
  unnavigable mess. Breath's guided interactive commit wizard ensures that every commit adheres to predefined,
  structured guidelines, making your project history clear, searchable, and informative for all team members.

- Boost Developer Productivity: By automating routine tasks like code checks and simplifying the commit process, Breath
  frees up developers to focus on what they do best: writing great code. The intuitive VCS command proxying and
  interactive commit flow reduce mental overhead and streamline daily operations.

- Adaptable to Any Project: With its VCS-agnostic compatibility, Breath seamlessly integrates into projects using either
  Git or Mercurial without any additional configuration. Its intelligent detection of technology stacks ensures that the
  right health checks are applied, making it a versatile tool for diverse development environments.

- Foster a Culture of Discipline: Breath promotes disciplined development practices by making it easy to adhere to best
  practices and project standards. This consistency across the team leads to more robust, maintainable, and ultimately,
  more successful software projects.

- Engaging and Intuitive Experience: The unique space-themed commit typology and interactive wizard not only make the
  commit process more engaging but also contribute to a more understandable and easily navigable project history,
  transforming a mundane task into a more intuitive experience.

## How to install breath

From crates.io:

```shell
cargo install breath broot eza
```

## Usage

```shell
breath status
breath commit
breath zen
breath health
breath log
breath diff
breath push
breath pull
```

## Example pre-commit hook

```sh
#!/usr/bin/env bash
unset GIT_DIR
breath health && exit 0 || exit 1
```

## Language supported

| language   | supported   |
|------------|-------------|
| Javascript | yes         |
| Typescript | yes         |
| Python     | yes         |
| Go         | yes         |
| Java       | yes         |
| C          | yes         |
| D          | yes         |
| C++        | yes         |
| C#         | yes         |
| Ruby       | yes         |
| Swift      | yes         |
| Kotlin     | yes         |
| Scala      | in progress |
| Rust       | yes         |
| Dart       | yes         |
| Elixir     | yes         |
| Erlang     | in progress |
| Haskell    | yes         |
| Lua        | in progress |
| OCaml      | in progress |
| Perl       | in progress |
| R          | in progress |
| Scala      | in progress |
| Swift      | yes         |
| Php        | yes         |    

## Commit message format

```text
<type> ~ <summary>

    Why changes?
    
        * <details> 
    
    Breaking Changes:
    
        * <breaking changes>
    
    What changes?
    
        * <what details>
    
    Who changes?
    
        <author> ~ <roles>
        
    Benefits:
    
        * <benefice>
    
    Notes:
    
        * <team message>
    
    Resolves
        
        Fixes #<issues>
```

## Commit message example

```text
Nebula ~ update commit display logic

    Why changes?
    
        * The previous implementation of `Display` for the `Commit` struct was becoming hard to maintain.
    
        * The order of sections was confusing  (e.g., `When changes` contained implementation details).
    
        * Adding new sections like `Benefits` or `Breaking Changes` required significant code modification.
    
        * This refactoring addresses these issues based on team feedback.
    
    Breaking Changes:
        
        * None.
        
        * This change only affects the internal display logic and the final text output format.
        
        * The command-line interface and core functionality remain unchanged.
        
    What changes?
    
        * Refactored the `impl Display for Commit` block in `src/commit.rs`.
        
        * Introduced helper functions to conditionally render sections only if they contain meaningful content.
        
        * Renamed the section previously titled `When changes` to `What changes` to accurately reflect its content.
        
        * Added new sections `Benefits` and `Breaking Changes` to the output format.
        
        * Updated the display logic for `Who changes` to remove leading `@` symbols for roles, improving readability.
    
    Who changes?
        
        * @hackia ~ Developer
    
    Benefits:
        
        * This change makes it easier to understand the commit type and its purpose.
        
        * It also makes it easier to remember the commit type `Star` than `Feature`.
        
        * It also makes it easier to understand the commit type `Star` than `Feature`.
        
    Notes:
    
        * This change implements the structure discussed in our recent sync about improving commit message quality.
        
        * Please review the new format and provide feedback.
    
    Resolves
    
        Fixes #15 
```

## Why cosmic types and not just categories?

The idea is to make it easier to understand the commit type and its purpose.

For example, `Star` is a commit type that adds a new feature or improves an existing one.

It's also easier to remember the commit type `Star` than `Feature`.

## Commit typology

| Commit Type                | Mnemonic                                                                                           | Description                                              |
|----------------------------|----------------------------------------------------------------------------------------------------|----------------------------------------------------------|
| Star                       | Shiny Technology Added or Refined                                                                  | New feature or enhancement                               |
| Comet                      | Code or Module Error Terminated                                                                    | Bug fix or error resolution                              |
| Nebula                     | New Efficient Better Understandable Logic Achieved                                                 | Code refactoring                                         |
| Pulsar                     | Powerful Upgrade, Less Sluggish, Agile Response                                                    | Performance improvement                                  |
| Quasar                     | Quick Adjustments for Superior Accuracy and Readability                                            | Documentation or clarity improvement                     |
| Asteroid Belt              | Adjustments, Sweeps, Tidy-ups, Elimination, Reordering of Items, Decrease Bloat                    | Code cleanup and maintenance                             |
| Solar Flare                | Securing Our Logic Against Regressions, Failures, and Latencies Actively, Rigorously Ensured       | Adding or updating tests (unit, integration, end-to-end) |
| Dwarf Planet               | Details Warranted Attention, Refined Further, Polished Little Aspects Neatly Enhanced              | Tiny Minor but essential updates or fixes                |
| Terraform                  | Technology Engineering Resources Readily Automated, Foundation of Reliable Management              | Infrastructure changes                                   |
| Black Hole                 | Big Legacy Aspects Consumed, Killing Heavy, Old Loads Entirely                                     | Removing large chunks of code or features                |
| Wormhole                   | Weaving or Reconnecting Modules, Hitching onto Linked Elements                                     | Merging branches or connecting code parts                |
| Big Bang                   | Birth of Initial Greatness, Beginning All New Growth                                               | Initial commit of a project or major feature             |
| Launch                     | Lifting Application Upward, New Code Entering Production                                           | Deploying to production or releasing a version           |
| Lightspeed                 | Lightening Speed Enhancements                                                                      | Significant performance improvements                     |
| Mission Control            | Managing Changes, Issues, Scope, Teamwork, and Release On Time                                     | Project management changes                               |
| Spacewalk                  | Swift Work Above Limits, Keeping All Systems Extra Safe                                            | Urgent hotfixes or critical production updates           |
| Moon Landing               | Major Leaps Over Night, New Doors and Incredible Achievements                                      | Completing major milestones or goals                     |
| First Contact              | Forge Initial Connections, Open New Territories                                                    | Establishing initial connections or integrations         |
| Interstellar Communication | Informing, Sharing, Teaching, Educating, & Learning Lucidly & Clearly                              | Improving documentation or communication                 |
| Solar Eclipse              | Sun Escapes, Legacy Code Lurks                                                                     | Temporarily masking functionality                        |
| Supernova                  | Sudden Unbelievable Performance Revolution, New Version Arrives                                    | Major, transformative change or improvement              |
| Meteor Shower              | Many Edits, Tiny Overall Result, Overhaul Routines                                                 | Series of small changes or fixes                         |
| Cosmic Dawn                | Creating Original, Simple, Minimal Initial Draft                                                   | Initial implementation of a feature                      |
| Solar Storm                | Sudden Transformations Occur Rapidly, Modifications                                                | Rapid, impactful changes                                 |
| Lunar Transit              | Little Update, Now Adjustments Require Testing                                                     | Minor, temporary change                                  |
| Perihelion                 | Perfect Ending, Refined, Improved, High Efficiency, Low Obstacles, Near Goal                       | Significant milestone or feature completion              |
| Aphelion                   | Away From Perfection, High Effort, Long Overhaul, Intense Overhaul, Obstacles                      | Refactor, dependency update, or architecture change      |
| White Dwarf                | Writing, Improving, Detailed Documentation For All                                                 | Improving code comments or documentation                 |
| Red Giant                  | Refactoring, Enhancing, Growing, Increasing, Adding New Things                                     | Expanding a feature or functionality                     |
| Neutron Star               | New Efficient Utility, Tweaks, Robust Optimization, Nimble Solution                                | Optimizing code for performance                          |
| Binary Star                | Bringing In New And Revised, Yielding Integrated Results                                           | Merging features or components                           |
| Brown Dwarf                | Barely Developed, Requires Work, Ongoing Development For Future                                    | Undeveloped feature with potential                       |
| Quark Star                 | Questionable, Unstable, Anticipated Results, Risky, Keen Experiment                                | Experimental or speculative change                       |
| Rogue Planet               | Refactoring Or Generating Operations, Unique Path, Leaping Ahead                                   | Independent change unrelated to the main codebase        |
| Stellar Nursery            | Starting To Enhance, Laying Layers, Launching New Requirements                                     | Creating new components                                  |
| Planetary Nebula           | Pruning, Leaving, Abandoning, Nostalgic Era, Totally Removed                                       | Removal or deprecation of a component                    |
| Globular Cluster           | Gathering, Linking, Operations, Bringing Unity, Lots of Adjustments, All Related                   | Collection of related changes                            |
| Void                       | Vanished, Obliterated, Irrelevant, Deleted                                                         | Removal of a module, component, or feature               |
| Gravity                    | Glitch Resolution, Adjusting Versions, Integrating, Troubleshooting Yielding                       | Resolving merge conflicts or dependencies                |
| Dark Matter                | Debugging And Resolving Mysterious Attributes, Tricky issues Removed                               | Fixing unknown or mysterious bugs                        |
| Time Dilation              | Time Is Dilated, Improvements Leverage Agility, Time-Saving                                        | Improving code performance or reducing execution time    |
| Spacetime                  | Scheduling, Planning, Adjusting Calendar Events, Coordinating Time                                 | Changes to date, time, or scheduling                     |
| Gravitational Lensing      | Gravity Redirects Light, Altering Information Pathways                                             | Altering data or information flow                        |
| Cosmic String              | Connecting Our Sections, Merging Together, Interlinking New Groups                                 | Connecting code parts                                    |
| Quantum Fluctuation        | Quick Unpredictable Adjustments, Noticed Tiny Unexpected Modification                              | Small, random change                                     |
| Hawking Radiation          | Hastily And Willingly Killing Redundancies, Ageing Dead-ends, Tidying In Order, Obliterating Noise | Removing technical debt                                  |
| Quantum Entanglement       | Quantum Effects Never Tangled, Greater Efficiency, Linked Adjustments                              | Establishing close relationships between code parts      |
| Gravitational Redshift     | Gravity Reduces Efficiency, Degraded Speed, Shift Happens                                          | Slowing down or reducing code performance                |
| Space Probe                | Surveying, Planning, Analysing, Checking Every Nook                                                | Testing new features or technologies                     |
| Space Station              | Setting Up The Area, Testing In Orbit, Optimising New                                              | Creating or improving environments                       |
| Rocket Launch              | Releasing Our Code, Keenly Entering The Production                                                 | Deploying to production                                  |
| Spacewalk                  | Swift Patches And Lookout Work, Keeping Systems Extra safe                                         | Urgent production hotfixes                               |
| Space Elevator             | Streamlined Access, Providing Easy Vertical On boarding, Lifting Entries                           | Making code base more accessible                         |
