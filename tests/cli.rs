//! Command-line parsing.

use clap::Parser;
use clap::error::ErrorKind;
use tollway::cli::{Cli, Command};

#[test]
fn serve_defaults() {
    let cli = Cli::try_parse_from(["tollway", "serve"]).unwrap();
    assert_eq!(
        cli.command,
        Command::Serve {
            host: "0.0.0.0".into(),
            port: 8080
        }
    );
}

#[test]
fn serve_with_port() {
    let cli = Cli::try_parse_from(["tollway", "serve", "--port", "9000"]).unwrap();
    assert!(matches!(cli.command, Command::Serve { port: 9000, .. }));
}

#[test]
fn version_flag() {
    let err = Cli::try_parse_from(["tollway", "--version"]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert!(err.to_string().contains(tollway::VERSION));
}

#[test]
fn command_is_required() {
    assert!(Cli::try_parse_from(["tollway"]).is_err());
}
