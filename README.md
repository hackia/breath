# Breath

A Rust tool for streamlining the process of committing code to Git and Mercurial repositories.

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Crates.io](https://img.shields.io/crates/v/breath.svg)](https://crates.io/crates/breath)

A command-line tool for streamlining the process of committing Rust code to Git and Mercurial repositories. It automates
checks like
formatting, tests, documentation, dependency audit and provides interactive commit message creation.

## Features

- Interactive commit message creation with a conventional commit format
- Pre-commit checks:
    - Code compilation check using `cargo check`
    - Code formatting verification with `cargo fmt`
    - Test suite execution via `cargo test`
    - Lint checks using `cargo clippy`
    - Documentation generation with `cargo doc`
    - Dependencies audit through `cargo audit`
- User-friendly terminal interface with progress indicators
- Version control workflow automation:
    - Staged files preview
    - Commit creation
    - Optional push to remote
- Clean error reporting with logs stored in `.breathes` directory
- Support for multiple version control systems:
    - Git support via `--features git`
    - Mercurial support via `--features hg`
    - Default hooks run with default features

## Installation

### Git support

```shell
cargo install breath --features git
```

### Mercurial support

```shell
cargo install breath --features hg
```

### Hooks support

```shell
cargo install breath
```

## Usage

### Interactive commit message creation

```shell
breath
```

### pre-commit hook

```sh
#!/bin/sh

breath && exit 0 || exit 1
```

## Coming soon

* Support for other version control systems

Best regards [hackia](https://github.com/hackia)