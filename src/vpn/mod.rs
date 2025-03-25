// src/vpn/mod.rs
pub mod wireguard;
pub mod openvpn;
pub mod shadowsocks;
pub mod v2ray;
pub mod l2tp;

use crate::config::Config;

pub struct VpnManager {
    config: Config,
}

impl VpnManager {
    pub fn new(config: Config) -> Self {
        VpnManager { config }
    }

    pub fn start(&mut self) {
        println!("VPN Manager started with config: {:?}", self.config);

        // VPN türüne göre bağlantıyı başlat
        match self.config.default_vpn.as_str() {
            "wireguard" => wireguard::connect(),
            "openvpn" => openvpn::connect(),
            "shadowsocks" => shadowsocks::connect(),
            "v2ray" => v2ray::connect(),
            "l2tp" => l2tp::connect(),
            _ => println!("Unknown VPN type, using default"),
        }
    }
}
