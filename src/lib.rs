//! tollway: a small, self-hosted gateway that keeps AI agents running when providers fail.
//!
//! The binary in `main.rs` is a thin wrapper; everything testable lives in this library.

pub mod app;
pub mod cli;
pub mod config;
pub mod error;
pub mod llm;

/// The crate version, as reported by `tollway --version`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
