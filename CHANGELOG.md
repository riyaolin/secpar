# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-03-01

### Added

- **Interactive selection menus** — `sec get`, `sec describe`, `sec delete`, `par get`, and `par delete` now accept an optional `--name` flag; when omitted, an interactive selection menu is presented populated from the live AWS resource list.
- **Spinner progress indicators** — all AWS API calls are now wrapped with an indicatif braille spinner so the terminal provides feedback during network operations.
- **Confirmation prompt on delete** — `sec delete` and `par delete` require explicit confirmation before proceeding (defaults to "No").
- **Formatted table output** — `sec list` and `par list` now render results as UTF-8 tables with columns NAME/ARN/LAST CHANGED and NAME/TYPE/LAST MODIFIED respectively.
- **`src/ui.rs`** — new module exposing `new_spinner`, `confirm_delete`, `select_from_list`, `build_secrets_table`, and `build_parameters_table`.
- **`SecParError::Interactive`** — new error variant for terminal interaction failures.
- **Crate-level `//!` documentation** in `src/lib.rs` covering overview, credential resolution order, and CLI examples.
- **Rustdoc** on all public functions across `src/cli/sec.rs`, `src/cli/par.rs`, `src/util.rs`, `src/specs/mod.rs`, and `src/errors.rs`.
- **Unit tests** in `src/ui.rs` (table builders, empty-list guard) and `src/util.rs` (YAML parsing, missing file).
- New dependencies: `indicatif 0.18`, `dialoguer 0.12`, `comfy-table 7.2`.
- New dev-dependency: `tempfile 3`.

### Changed

- `list_secrets` return type changed from `Result<(), SecParError>` to `Result<Vec<(String, String, String)>, SecParError>` — separates data retrieval from rendering.
- `list_parameters` return type changed from `Result<(), SecParError>` to `Result<Vec<(String, String, String)>, SecParError>`.
- `--name` is now optional on `sec get`, `sec describe`, `sec delete`, `par get`, and `par delete`.
- Version bumped from `0.1.2` → `0.2.0`.

## [0.1.2] - 2024-xx-xx

### Changed

- Updated to AWS SDK GA version.

## [0.1.1] - 2024-xx-xx

### Changed

- Removed manually compiled binary; pending GitHub Actions automated build/release.

## [0.1.0] - 2024-xx-xx

### Added

- Initial release: `sec` and `par` subcommands for AWS Secrets Manager and Parameter Store.
- Global `--region` and `--profile` options.
- `par apply` for bulk-loading parameters from a YAML spec file.
