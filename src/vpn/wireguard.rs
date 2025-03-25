// src/vpn/wireguard.rs
use std::process::Command;

pub fn connect() {
    println!("Connecting to WireGuard VPN...");

    // WireGuard konfigürasyon dosyasının yolu
    let config_path = "/etc/wireguard/wg0.conf";
    
    // WireGuard'ı başlat
    let status = Command::new("wg-quick")
        .arg("up")
        .arg(config_path)
        .status()
        .expect("Failed to start WireGuard");

    if status.success() {
        println!("WireGuard connected successfully.");
    } else {
        println!("Failed to connect to WireGuard.");
    }
}