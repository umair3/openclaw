// Logs Module for OpenClaw Linux Desktop App
// Handles viewing, filtering, exporting logs from ~/.openclaw/logs
use std::fs;

pub fn view_logs() {
    // View logs (stub: print log files)
    let log_dir = "~/.openclaw/logs/";
    match fs::read_dir(log_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(e) = entry {
                    println!("Log file: {}", e.path().display());
                }
            }
        },
        Err(e) => println!("Failed to read log directory: {}", e),
    }
}

pub fn filter_logs() {
    // Filter logs (stub)
    println!("Filtering logs...");
    // Real filtering logic: filter logs containing 'error'
    let log_dir = "~/.openclaw/logs/";
    match fs::read_dir(log_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(e) = entry {
                    if let Ok(content) = fs::read_to_string(e.path()) {
                        for line in content.lines() {
                            if line.contains("error") {
                                println!("{}: {}", e.path().display(), line);
                            }
                        }
                    }
                }
            }
        },
        Err(e) => println!("Failed to read log directory: {}", e),
    }
}

pub fn export_logs() {
    // Export logs (stub)
    println!("Exporting logs...");
    // Real export logic: zip all logs to logs_export.zip
    use std::process::Command;
    let log_dir = "~/.openclaw/logs/";
    let output = Command::new("zip")
        .arg("-r")
        .arg("logs_export.zip")
        .arg(log_dir)
        .output();
    match output {
        Ok(out) => println!("Export output:\n{}", String::from_utf8_lossy(&out.stdout)),
        Err(e) => println!("Failed to export logs: {}", e),
    }
}
