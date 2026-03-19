// Plugin Manager for OpenClaw Linux Desktop App
// Handles plugin listing, enable/disable, logs, install
use std::process::Command;

pub fn show_plugin_activity(plugin: &str) {
    // Show plugin activity (stub: print activity file)
    let activity_path = format!("~/.openclaw/plugins/{}/activity.txt", plugin);
    match std::fs::read_to_string(&activity_path) {
        Ok(content) => println!("Plugin activity for {}:\n{}", plugin, content),
        Err(e) => println!("Failed to read plugin activity: {}", e),
    }
}

pub fn list_plugins() {
    // List installed plugins (stub: call npm list)
    let output = Command::new("npm")
        .arg("list")
        .arg("--depth=0")
        .arg("--prefix")
        .arg("~/.openclaw/plugins/")
        .output();
    match output {
        Ok(out) => println!("Plugins:\n{}", String::from_utf8_lossy(&out.stdout)),
        Err(e) => println!("Failed to list plugins: {}", e),
    }
}

pub fn enable_plugin(plugin: &str) {
    // Enable plugin by updating config (stub)
    let config_path = "~/.openclaw/openclaw.json";
    println!("Enabling plugin: {}", plugin);
    // In a real implementation, update config to enable plugin
    // For now, just print action
}

pub fn disable_plugin(plugin: &str) {
    // Disable plugin by updating config (stub)
    let config_path = "~/.openclaw/openclaw.json";
    println!("Disabling plugin: {}", plugin);
    // In a real implementation, update config to disable plugin
    // For now, just print action
}

pub fn install_plugin(spec: &str) {
    // Install plugin via npm spec (stub)
    println!("Installing plugin: {}", spec);
    let output = Command::new("npm")
        .arg("install")
        .arg(spec)
        .arg("--prefix")
        .arg("~/.openclaw/plugins/")
        .output();
    match output {
        Ok(out) => println!("Install output:\n{}", String::from_utf8_lossy(&out.stdout)),
        Err(e) => println!("Failed to install plugin: {}", e),
    }
}

pub fn show_plugin_logs(plugin: &str) {
    // Show plugin logs and activity (stub: print log file)
    let log_path = format!("~/.openclaw/plugins/{}/logs.txt", plugin);
    match std::fs::read_to_string(&log_path) {
        Ok(content) => println!("Plugin logs for {}:\n{}", plugin, content),
        Err(e) => println!("Failed to read plugin logs: {}", e),
    }
}
