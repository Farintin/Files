# Changelog

All notable changes to this project will be documented in this file.

The format is inspired by Keep a Changelog
and adheres to Semantic Versioning.

---

## [0.1.0] - 2026-02-21

### Added

- Core `AppState` abstraction for filesystem navigation
- Encapsulated selection logic with safe API boundaries
- Command-based mutation layer (`Command` enum)
- Navigation support (enter directory, go up)
- Deterministic sorting (directories first, case-insensitive)
- Filesystem abstraction for testability
- Comprehensive unit test coverage for state transitions

### Architecture

- Modular state separation (`navigation`, `selection`, `sorting`, `command`)
- Read-only public API boundaries
- Internal mutation restricted to command layer

This release establishes the foundational Rust core engine for Files.
