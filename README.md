# Files

Files is a Rust-powered native file manager.

This project explores clean systems design, strong state encapsulation,
and modular architecture in Rust — evolving from a pure core engine
into a fully integrated desktop application.

---

## Current Status

**v0.1.0 — Core Engine Foundation Complete**

The Rust core provides:

- Filesystem abstraction layer
- Deterministic directory sorting
- Encapsulated navigation state
- Command-driven mutation model
- Fully tested state transitions

---

**In Progress — v0.2.0 (TUI Adapter)**

- ratatui-based terminal interface
- Arrow-key navigation
- Directory traversal (Enter / Backspace)
- Real filesystem integration
- Core-command integration validated

The interface layer is intentionally thin and delegates all behavior to `files-core`.

---

## Architecture

The project is structured in vertical layers:

- **files-core** — Pure Rust engine (complete)
- **files-tui** — Terminal interface adapter (next)
- **files-desktop** — Tauri-based native UI (planned)

### Design Principles

- Strong API boundaries
- Behavior-driven testing
- Command-based state mutation
- Minimal UI coupling
- Vertical iteration over horizontal abstraction

---

## Roadmap

### Next

- Minimal terminal UI adapter (`files-tui`)
- Keyboard-driven navigation
- End-to-end vertical slice validation

### Later

- Desktop UI integration (Tauri)
- Extended file operations (rename, delete)
- Filtering and search
- Cross-platform packaging

---

## Philosophy

Files is built incrementally through vertical slices:

1. Core engine
2. Interface adapter
3. Native desktop integration

Each layer is independently testable and replaceable.

---

Work in progress.
