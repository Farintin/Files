# Files – Architecture Overview (v0.1)

Files is a Rust-powered file manager built on a core-first architecture.

All domain logic lives in Rust.
Interfaces (terminal or desktop) act as thin adapters.

---

## Architectural Philosophy

Files is developed using vertical iteration:

1. Build a clean, testable core engine.
2. Add a thin interface adapter.
3. Integrate into a native desktop application.

The core remains framework-agnostic and UI-independent.

---

## Design Principles

1. Logic first, UI second.
2. Strong API boundaries.
3. Explicit error modeling.
4. Command-driven state mutation.
5. Testable and deterministic behavior.

---

## Current Structure (v0.1.0)

### files-core (Complete)

Responsible for:

- Filesystem abstraction
- Directory traversal
- FileEntry modeling
- Deterministic sorting (directories first, case-insensitive)
- Encapsulated selection logic
- Navigation state modeling
- Command-based mutation layer
- Unit-tested state transitions

The core exposes a read-only API and centralizes all mutation logic.

---

## Planned Layers

### files-tui (Next)

Terminal interface adapter:

- Renders state
- Maps keyboard input to `Command`
- Delegates behavior to `files-core`

Thin layer. No business logic.

---

### files-desktop (Planned)

Tauri-based desktop interface:

- Window management
- IPC boundary
- Rendering
- OS integration (shortcuts, drag & drop)

---

## Extensibility Strategy

Future features should integrate without rewriting the core:

- Extended file operations (rename, delete)
- Filtering and search
- Indexing layer
- Caching
- Optional AI-assisted features

All additions must preserve core isolation.

---

## Scope of v0.1.0

- Navigation
- Selection
- Sorting
- Command dispatch
- Filesystem abstraction
- Test coverage

Foundation only.
