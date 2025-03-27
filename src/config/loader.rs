//! Configuration Loading and Validation
//! 
//! Provides a robust system for loading, validating, 
//! and managing application configurations

use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, File, Environment};
use std::path::{Path, PathBuf};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct OrchestratorConfig {
    // VPN Protocol Configurations
    #[validate(length(min = 1, message = "At least one protocol must be configured"))]
    pub protocols: Vec<ProtocolConfig>,

    // Network Settings
    pub network: NetworkSettings,

    // Security Configuration
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProtocolConfig {
    pub name: String,
    pub server: String,
    pub port: u16,
    pub encryption: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NetworkSettings {
    pub ping_timeout_ms: u64,
    pub connection_retry_attempts: u8,
    pub bypass_techniques: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SecurityConfig {
    pub authentication_method: String,
    pub encryption_level: String,
}

impl OrchestratorConfig {
    /// Load configuration from multiple sources
    pub fn load(config_path: &Path) -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Default configuration
            .add_source(File::from(config_path))
            
            // Environment variable overrides
            .add_source(Environment::with_prefix("VPN_ORCHESTRATOR"))
            
            // Build the configuration
            .build()?;
        
        // Deserialize and validate configuration
        let mut orchestrator_config: Self = config.try_deserialize()?;
        
        // Perform additional validation
        orchestrator_config.validate()?;
        
        Ok(orchestrator_config)
    }

    /// Custom validation beyond default serde/validator checks
    fn validate(&self) -> Result<(), ConfigError> {
        // Additional complex validation logic
        if self.protocols.is_empty() {
            return Err(ConfigError::Message("No VPN protocols configured".to_string()));
        }

        // Validate each protocol
        for protocol in &self.protocols {
            if protocol.server.is_empty() {
                return Err(ConfigError::Message(
                    format!("Invalid server for protocol: {}", protocol.name)
                ));
            }
        }

        Ok(())
    }

    /// Get configuration for a specific protocol
    pub fn get_protocol_config(&self, protocol_name: &str) -> Option<&ProtocolConfig> {
        self.protocols.iter()
            .find(|p| p.name == protocol_name)
    }
}

// Default configuration generation
impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            protocols: vec![
                ProtocolConfig {
                    name: "wireguard".to_string(),
                    server: "wg.example.com".to_string(),
                    port: 51820,
                    encryption: "ChaCha20-Poly1305".to_string(),
                }
            ],
            network: NetworkSettings {
                ping_timeout_ms: 1000,
                connection_retry_attempts: 3,
                bypass_techniques: vec![
                    "dpi_obfuscation".to_string(),
                    "port_randomization".to_string(),
                ],
            },
            security: SecurityConfig {
                authentication_method: "certificate".to_string(),
                encryption_level: "high".to_string(),
            },
        }
    }
}