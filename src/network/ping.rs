// src/network/ping.rs
use std::process::Command;

pub fn get_avg_ping(server: &str) -> Option<f64> {
    let output = Command::new("ping")
        .arg("-c").arg("3")
        .arg(server)
        .output()
        .expect("Failed to execute ping");
    
    if !output.status.success() {
        return None;
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("rtt min/avg/max/mdev") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() > 1 {
                let values: Vec<&str> = parts[1].trim().split('/').collect();
                if values.len() > 1 {
                    return values[1].parse::<f64>().ok();
                }
            }
        }
    }
    None
}