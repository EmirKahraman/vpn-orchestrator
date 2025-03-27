//! VPN Protocol Abstraction
//! 
//! Defines a common interface for different VPN protocols
//! to ensure consistent behavior and easy extensibility

use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Represents the current status of a VPN connection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

/// Detailed connection metrics for performance tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionMetrics {
    pub latency_ms: u32,
    pub bandwidth_mbps: f64,
    pub packet_loss_percentage: f32,
    pub server_location: String,
}

/// Core trait defining the interface for VPN protocols
#[async_trait]
pub trait VPNProtocol: Send + Sync {
    /// Get the name of the protocol
    fn name(&self) -> &str;

    /// Validate the protocol's configuration
    fn validate_config(&self) -> Result<()>;

    /// Establish a VPN connection
    async fn connect(&self) -> Result<ConnectionStatus>;

    /// Disconnect from the current VPN
    async fn disconnect(&self) -> Result<()>;

    /// Get current connection status
    async fn status(&self) -> ConnectionStatus;

    /// Retrieve current connection metrics
    async fn get_metrics(&self) -> Result<ConnectionMetrics>;
}

/// Protocol-specific configuration structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProtocolConfiguration {
    pub server: String,
    pub port: u16,
    pub username: Option<String>,
    pub encryption_method: String,
}