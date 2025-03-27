//! VPN Orchestrator - Main Application Entry Point
//! 
//! This is the central hub that coordinates various components 
//! of the VPN management system.

use anyhow::Result;
use clap::Parser;
use tracing::{info, error};

mod config;
mod protocols;
mod network;
mod security;
mod managers;
mod utils;
mod cli;

use config::OrchestratorConfig;
use managers::vpn_manager::VPNManager;
use network::monitor::NetworkMonitor;
use cli::commands::CliArgs;
use utils::logging;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let cli_args = CliArgs::parse();

    // Initialize logging
    logging::setup_logger(&cli_args.log_level)?;

    // Load configuration
    let config = match OrchestratorConfig::load(&cli_args.config_path) {
        Ok(cfg) => {
            info!("Configuration loaded successfully");
            cfg
        },
        Err(e) => {
            error!("Configuration load failed: {}", e);
            return Err(anyhow::anyhow!("Failed to load configuration"));
        }
    };

    // Initialize security context
    let security_context = security::SecurityManager::new(&config)?;

    // Create VPN Manager
    let mut vpn_manager = VPNManager::new(config.clone(), security_context);

    // Initialize network monitor
    let network_monitor = NetworkMonitor::new(&config);

    // Dynamic protocol registration
    vpn_manager.register_protocols()?;

    // Attempt to establish VPN connection
    match vpn_manager.connect_best_server().await {
        Ok(_) => info!("Successfully established VPN connection"),
        Err(e) => {
            error!("Failed to establish VPN connection: {}", e);
            return Err(anyhow::anyhow!("VPN connection error"));
        }
    }

    // Start network monitoring
    network_monitor.start_monitoring().await;

    // Wait for termination signal
    tokio::signal::ctrl_c().await?;

    // Graceful shutdown
    vpn_manager.disconnect_all().await?;
    network_monitor.stop().await;

    info!("VPN Orchestrator shutting down gracefully");

    Ok(())
}