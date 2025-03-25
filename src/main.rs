// src/main.rs
mod vpn;
mod network;
mod config;

use vpn::VpnManager;
use config::Config;

fn main() {
    let config = Config::load_from_url("https://example.com/vpn-config.json");
    let mut vpn_manager = VpnManager::new(config);
    vpn_manager.start();
}