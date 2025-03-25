// src/vpn/shadowsocks.rs
use std::process::Command;

pub fn connect() {
    println!("Connecting to Shadowsocks...");

    // Shadowsocks konfigürasyon dosyasının yolu
    let config_path = "/etc/shadowsocks/config.json";
    
    // Shadowsocks'ı başlat
    let status = Command::new("ss-local")
        .arg("-c")
        .arg(config_path)
        .status()
        .expect("Failed to start Shadowsocks");

    if status.success() {
        println!("Shadowsocks connected successfully.");
    } else {
        println!("Failed to connect to Shadowsocks.");
    }
}