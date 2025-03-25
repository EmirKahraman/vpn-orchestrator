// src/network/mac_spoof.rs
use std::process::Command;

pub fn change_mac(interface: &str) {
    println!("Changing MAC address for {}", interface);
    Command::new("ifconfig").arg(interface).arg("down").status().unwrap();
    Command::new("macchanger").arg("-r").arg(interface).status().unwrap();
    Command::new("ifconfig").arg(interface).arg("up").status().unwrap();
}
