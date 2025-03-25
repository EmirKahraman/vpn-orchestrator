// src/vpn/openvpn.rs
use std::process::Command;

pub fn connect() {
    println!("Connecting to OpenVPN...");

    // OpenVPN konfigürasyon dosyasının yolu
    let config_path = "/etc/openvpn/client.ovpn";
    
    // OpenVPN'i başlat
    let status = Command::new("openvpn")
        .arg("--config")
        .arg(config_path)
        .status()
        .expect("Failed to start OpenVPN");

    if status.success() {
        println!("OpenVPN connected successfully.");
    } else {
        println!("Failed to connect to OpenVPN.");
    }
}