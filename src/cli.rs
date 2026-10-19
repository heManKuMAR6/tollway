//! The `tollway` command-line interface.

use clap::{Parser, Subcommand};

/// tollway: keeps AI agents running when providers fail.
#[derive(Debug, Parser)]
#[command(name = "tollway", version, about)]
pub struct Cli {
    /// The command to run.
    #[command(subcommand)]
    pub command: Command,
}

/// Top-level commands.
#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Command {
    /// Run the gateway.
    Serve {
        /// Address to bind.
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
        /// Port to listen on.
        #[arg(long, default_value_t = 8080)]
        port: u16,
    },
}
