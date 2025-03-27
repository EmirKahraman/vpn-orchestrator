//! CLI Arguments and Command Handling
//! 
//! Provides a flexible command-line interface for 
//! interacting with the VPN Orchestrator

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to custom configuration file
    #[arg(short, long, default_value = "config/default.toml")]
    pub config_path: PathBuf,

    /// Logging verbosity level
    #[arg(short, long, default_value = "info")]
    pub log_level: String,

    /// List available VPN protocols
    #[arg(long)]
    pub list_protocols: bool,

    /// Connect to a specific VPN protocol
    #[arg(long)]
    pub connect_protocol: Option<String>,

    /// Perform network diagnostics
    #[arg(long)]
    pub diagnostics: bool,
}

impl CliArgs {
    /// Additional validation for CLI arguments
    pub fn validate(&self) -> Result<(), String> {
        // Add custom validation logic
        if !self.config_path.exists() {
            return Err(format!("Configuration file not found: {:?}", self.config_path));
        }

        // Validate log level
        match self.log_level.to_lowercase().as_str() {
            "error" | "warn" | "info" | "debug" | "trace" => Ok(()),
            _ => Err(format!("Invalid log level: {}", self.log_level)),
        }
    }
}