# Files – Architecture Overview (v0)

Files is a Rust-powered native desktop file manager.

The project is structured around a core-first philosophy:
All business logic lives in Rust.
The UI is a thin rendering layer.

---

## Design Principles

1. Logic first, UI second.
2. Clear separation of concerns.
3. Explicit error modeling.
4. Extensible architecture (AI/indexing modules can plug in later).
5. Native-feeling desktop behavior.

---

## High-Level Structure

Files is divided into two major layers:

### 1. Core (Rust Library)

Responsible for:

- Filesystem traversal
- Metadata extraction
- Sorting and filtering
- Selection logic
- Application state modeling
- Preview generation
- Future indexing modules

The core must remain framework-agnostic.

### 2. Desktop Interface (Tauri)

Responsible for:

- Window management
- IPC boundary
- Rendering state
- Handling OS integrations (drag & drop, shortcuts)

---

## Core Module Layout (Planned)

core/
├── fs/ # Filesystem engine
├── state/ # Centralized app state
├── selection/ # Native-style selection logic
├── preview/ # Safe file preview handling
├── errors.rs # Custom error types
└── lib.rs # Public API surface

---

## Future Extensibility

The architecture should allow:

- File indexing module
- Local AI integration
- Caching layer
- Plugin system

Without rewriting the core.

---

## Initial Scope (v0)

- Directory traversal
- FileEntry modeling
- Sorting (directories first)
- Structured error handling
- Unit tests

No AI.
No cloud.
No monetization features.

Foundation only.
