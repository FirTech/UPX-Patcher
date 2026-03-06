# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2025-03-06

### Added

- **Backup option**: Added `--backup` / `-b` flag to create `.bak` backup file before patching
- **Selective patching**: Added options to skip specific patching steps:
    - `--no-section` - Skip section name patching (UPX0/UPX1/UPX2)
    - `--no-version` - Skip version block confusing
    - `--no-dos` - Skip DOS stub message replacement
    - `--no-entry` - Skip entry point patching
- **Help and version**: Added `--help` / `-h` and `--version` / `-V` options using clap framework

### Changed

- **CLI framework**: Migrated from manual `env::args()` parsing to `clap::Parser` for better CLI experience
- **Code cleanup**: Removed unused `launched_from_explorer()` function and related dependencies
- **Imports cleanup**: Removed redundant imports and unused dependencies (sysinfo)

### Fixed

- **Typos**: Fixed typo "standart" → "standard" in log message

## [1.0.1] - 2025-12-11

### Removed

- Change WinAPI from ExitProcess to CopyContext (Incompatible with Windows XP)

## [1.0.0] - 2025-08-17
