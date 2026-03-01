//! # Lycanthrope
//!
//! Async TCP connection hijacker — Rust/C hybrid.
//!
//! ## Architecture
//! - **C core** (`csrc/`): packet construction, checksums, raw sockets
//! - **Rust layer** (`src/`): async orchestration, TUI, CLI
//! - **FFI bridge** (`src/ffi/`): safe wrappers over C functions
//!
//! ## Platform Support
//! - Linux (full)
//! - macOS (full)
//! - Android/Termux (full, requires root)
//! - Windows (partial, requires npcap + admin)

pub mod args;
pub mod errors;
pub mod ffi;
pub mod net;
pub mod tui;