// src/vpn/l2tp.rs
use std::process::Command;

pub fn connect() {
    println!("Connecting to L2TP/IPSec...");

    // L2TP/IPSec başlatma işlemi
    let status = Command::new("ipsec")
        .arg("up")
        .arg("l2tp-connection")
        .status()
        .expect("Failed to start L2TP/IPSec");

    if status.success() {
        println!("L2TP/IPSec connected successfully.");
    } else {
        println!("Failed to connect to L2TP/IPSec.");
    }
}