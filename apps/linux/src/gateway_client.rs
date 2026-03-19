// Gateway Client for OpenClaw Linux Desktop App
// Handles WebSocket connection to local gateway and gateway control
use websocket::{ClientBuilder, Message};
use std::process::Command;

pub fn connect_gateway() {
    // Connect to OpenClaw gateway via WebSocket
    let ws_url = "ws://127.0.0.1:18789";
    match ClientBuilder::new(ws_url).expect("Invalid WebSocket URL").connect_insecure() {
        Ok(mut client) => {
            println!("Connected to OpenClaw gateway.");
            // Example: send ping
            let _ = client.send_message(&Message::text("ping"));
        },
        Err(e) => {
            println!("Failed to connect to gateway: {}", e);
        }
    }
}

pub fn start_gateway() {
    // Start gateway process (stub)
    println!("Starting gateway...");
    let output = Command::new("openclaw")
        .arg("gateway")
        .arg("run")
        .arg("--bind")
        .arg("loopback")
        .arg("--port")
        .arg("18789")
        .arg("--force")
        .output();
    match output {
        Ok(out) => println!("Gateway start output:\n{}", String::from_utf8_lossy(&out.stdout)),
        Err(e) => println!("Failed to start gateway: {}", e),
    }
}

pub fn restart_gateway() {
    // Restart gateway process
    println!("Restarting gateway...");
    stop_gateway();
    start_gateway();
}

pub fn stop_gateway() {
    // Stop gateway process
    println!("Stopping gateway...");
    let output = Command::new("pkill")
        .arg("-9")
        .arg("-f")
        .arg("openclaw-gateway")
        .output();
    match output {
        Ok(out) => println!("Gateway stop output:\n{}", String::from_utf8_lossy(&out.stdout)),
        Err(e) => println!("Failed to stop gateway: {}", e),
    }
}
