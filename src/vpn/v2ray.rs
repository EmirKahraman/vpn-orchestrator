// src/vpn/v2ray.rs
use std::process::Command;

pub fn connect() {
    println!("Connecting to V2Ray...");

    // V2Ray konfigürasyon dosyasının yolu
    let config_path = "/etc/v2ray/config.json";
    
    // V2Ray'i başlat
    let status = Command::new("v2ray")
        .arg("-config")
        .arg(config_path)
        .status()
        .expect("Failed to start V2Ray");

    if status.success() {
        println!("V2Ray connected successfully.");
    } else {
        println!("Failed to connect to V2Ray.");
    }
}