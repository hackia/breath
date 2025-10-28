# What is Breath?

Breath is a development companion tool designed to assist with creating structured and high-quality commit messages
while ensuring code health. It aims to integrate seamlessly into the developer's workflow, offering features beyond a
simple commit helper.

Core Features:

VCS Agnostic: Breath automatically detects whether your project uses Git or Mercurial (hg) by checking for .git or .hg
directories.

Convenience commands like breath status, log, diff, push, and pull are correctly proxied to the underlying version
control system.

Guided Interactive Commits: Instead of a simple commit -m, running breath commit launches an interactive wizard that
prompts the user for structured input, including a commit "type" and "summary".

It uses the inquire crate for these prompts.

Expressive Typology: Breath employs a unique, space-themed commit typology (e.g., Star for features, Comet for fixes,
Nebula for refactoring) to help create more descriptive and organized commit histories.

Automatic Pre-Commit Health Checks: The breath health command (which is also integrated into the commit flow) acts as a
quality gate.

It detects the project's technology stack (Rust, Node.js, PHP, Go, etc.) by looking for marker files (Cargo.toml,
package.json, etc.) and runs predefined hooks like formatting, linting, testing, and security audits. Hook outputs are
logged for debugging.

Automatic Pre-Commit Health Checks: The breath health command (which is also integrated into the commit flow) acts as a
quality gate.

It detects the project's technology stack (Rust, Node.js, PHP, Go, etc.) by looking for marker files (Cargo.toml,
package.json, etc.) and runs predefined hooks like formatting, linting, testing, and security audits. Hook outputs are
logged for debugging.

<div align="center">

![breath](https://raw.githubusercontent.com/otechdo/zuu/refs/heads/main/zuu.png)

![Version](https://img.shields.io/crates/v/breath.svg) ![License](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)

</div>

## Installation

From crates.io:

```shell
cargo install breath broot
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

breath health && exit 0 || exit 1
```

## Language supported

- Rust
- Python
- Go
- Java
- C
- C++
- C#
- JavaScript
- TypeScript
- PHP
- Shell
- Ruby
- Swift
- Kotlin
- Scala
- Rust
- Dart
- Elixir
- Erlang
- Haskell
- Lua
- OCaml
- Perl
- R
- Scala
- Swift
- Visual Basic
- Visual Basic .NET
- Visual C++

## Commit message format

```text
<type> ~ <summary>

Why changes?

    <details> 

Breaking Changes:

    <breaking changes>

What changes?

    <what details>

Who changes?

    <author> ~ <roles>
    
Benefits:

    <benefice>

Notes:

    <team message>

Resolves
    
    Fixes #<issues>
    ...
```


## Commit message example

```text
Nebula ~ update commit display logic

Why changes?

    The previous implementation of `Display` for the `Commit` struct was becoming hard to maintain.

    The order of sections was confusing  (e.g., `When changes` contained implementation details).

    Adding new sections like `Benefits` or `Breaking Changes` required significant code modification.

    This refactoring addresses these issues based on team feedback.

Breaking Changes:
    
    None.
    
    This change only affects the internal display logic and the final text output format.
    
    The command-line interface and core functionality remain unchanged.
    
What changes?

    Refactored the `impl Display for Commit` block in `src/commit.rs`.
    
    Introduced helper functions to conditionally render sections only if they contain meaningful content.
    
    Renamed the section previously titled `When changes` to `What changes` to accurately reflect its content.
    
    Added new sections `Benefits` and `Breaking Changes` to the output format.
    
    Updated the display logic for `Who changes` to remove leading `@` symbols for roles, improving readability.

Who changes?
    
    hackia ~ Developer

Benefits:
    
    This change makes it easier to understand the commit type and its purpose.
    
    It also makes it easier to remember the commit type `Star` than `Feature`.
    
    It also makes it easier to understand the commit type `Star` than `Feature`.
    
Notes:

    This change implements the structure discussed in our recent sync about improving commit message quality.
    
    Please review the new format and provide feedback.

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
