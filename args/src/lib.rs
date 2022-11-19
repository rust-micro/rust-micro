use clap::{Parser, Subcommand};
use shellexpand;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rust-micro")]
#[command(version = "1.0")]
#[command(author = "Peter Heiss <peter.heiss@uni-muenster.de>")]
#[command(about = "A cli for creating microservices in rust.")]
pub struct CliArgs {
    #[cfg(target_os = "linux")]
    /// Sets a custom config file.
    #[arg(short, long, value_name = "FILE", value_parser = parse_pathbuf, default_value = "~/.config/micro/config.toml")]
    pub config_file: PathBuf,

    #[cfg(target_os = "windows")]
    /// Sets a custom config file.
    #[arg(short, long, value_name = "FILE", value_parser = parse_pathbuf, default_value = "~\\micro\\config.toml")]
    pub config_file: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Parses the given str to PathBuf and expands the tilde.
fn parse_pathbuf(s: &str) -> Result<PathBuf, String> {
    Ok(PathBuf::from(shellexpand::tilde(s).to_string()))
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create the config file, if it is not already present in currently used config directory.
    /// The config file will be created with default values.
    /// If you want to change the default values, you can edit the config file.
    /// See --config-file for more information about config file location.
    Init,
    /// Create a new microservice
    New {
        /// The name of the new microservice
        name: String,
    },
}

pub fn get_cli_arguments() -> CliArgs {
    let cli = CliArgs::parse();
    cli
}
