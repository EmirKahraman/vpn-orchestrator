//! VPN Connection Management
//! 
//! Coordinates multiple VPN protocols, manages connections,
//! and implements intelligent server selection

use std::collections::HashMap;
use anyhow::{Result, Context};
use tokio::sync::Mutex;
use tracing::{info, warn, error};

use crate::config::OrchestratorConfig;
use crate::protocols::traits::{VPNProtocol, ConnectionStatus};
use crate::security::SecurityManager;
use crate::protocols::{
    wireguard::WireGuardProtocol,
    openvpn::OpenVPNProtocol,
    // Import other protocol implementations
};

pub struct VPNManager {
    // Registered VPN protocols
    protocols: Mutex<HashMap<String, Box<dyn VPNProtocol>>>,
    
    // Current active connection
    current_connection: Mutex<Option<String>>,
    
    // Application configuration
    config: OrchestratorConfig,
    
    // Security context
    security_manager: SecurityManager,
}

impl VPNManager {
    pub fn new(
        config: OrchestratorConfig, 
        security_manager: SecurityManager
    ) -> Self {
        Self {
            protocols: Mutex::new(HashMap::new()),
            current_connection: Mutex::new(None),
            config,
            security_manager,
        }
    }

    /// Dynamically register available VPN protocols
    pub fn register_protocols(&mut self) -> Result<()> {
        let mut protocols = self.protocols.blocking_lock();
        
        // Register WireGuard
        protocols.insert(
            "wireguard".to_string(), 
            Box::new(WireGuardProtocol::new(&self.config))
        );

        // Register OpenVPN
        protocols.insert(
            "openvpn".to_string(), 
            Box::new(OpenVPNProtocol::new(&self.config))
        );

        Ok(())
    }

    /// Intelligent server selection algorithm
    pub async fn connect_best_server(&mut self) -> Result<()> {
        let protocols = self.protocols.lock().await;
        
        // Prioritization strategy
        let prioritized_protocols = vec!["wireguard", "openvpn"];
        
        for protocol_name in prioritized_protocols {
            if let Some(protocol) = protocols.get(protocol_name) {
                match protocol.connect().await {
                    Ok(ConnectionStatus::Connected) => {
                        // Update current connection
                        let mut current_conn = self.current_connection.lock().await;
                        *current_conn = Some(protocol_name.to_string());
                        
                        info!("Connected via {}", protocol_name);
                        return Ok(());
                    },
                    Ok(ConnectionStatus::Error(msg)) => {
                        warn!("Connection failed for {}: {}", protocol_name, msg);
                    },
                    _ => continue
                }
            }
        }

        error!("Failed to establish VPN connection");
        Err(anyhow::anyhow!("No viable VPN connection found"))
    }

    /// Disconnect all active VPN connections
    pub async fn disconnect_all(&mut self) -> Result<()> {
        let protocols = self.protocols.lock().await;
        
        for (name, protocol) in protocols.iter() {
            if let Err(e) = protocol.disconnect().await {
                error!("Error disconnecting {}: {}", name, e);
            }
        }

        // Clear current connection
        let mut current_conn = self.current_connection.lock().await;
        *current_conn = None;

        Ok(())
    }
}