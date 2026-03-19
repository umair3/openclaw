// Config Manager for OpenClaw Linux Desktop App
// Handles editing and validating ~/.openclaw/openclaw.json
use std::fs;
use crate::gateway_client;

pub fn edit_config() {
    // Visual config editor (stub: print config)
    let config_path = "~/.openclaw/openclaw.json";
    match fs::read_to_string(config_path) {
        Ok(content) => println!("Current config:\n{}", content),
        Err(e) => println!("Failed to read config: {}", e),
    }
    // Visual editor UI (basic GTK window)
    use gtk4::prelude::*;
    use gtk4::{Application, ApplicationWindow, TextView, Button, Box, Orientation};
    let app = Application::builder()
        .application_id("ai.openclaw.config-editor")
        .build();
    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Edit OpenClaw Config")
            .default_width(600)
            .default_height(400)
            .build();
        let vbox = Box::new(Orientation::Vertical, 8);
        let text_view = TextView::new();
        // Load config content
        let config_path = "~/.openclaw/openclaw.json";
        if let Ok(content) = fs::read_to_string(config_path) {
            text_view.buffer().set_text(&content);
        }
        let save_btn = Button::with_label("Save");
        let save_path = config_path.to_string();
        save_btn.connect_clicked(move |_| {
            let buffer = text_view.buffer();
            if let Some(text) = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false) {
                if let Err(e) = fs::write(&save_path, text.as_str()) {
                    println!("Failed to save config: {}", e);
                } else {
                    println!("Config saved.");
                }
            }
        });
        vbox.append(&text_view);
        vbox.append(&save_btn);
        window.set_child(Some(&vbox));
        window.show();
    });
    app.run();
}

pub fn validate_config() {
    // Validate config file (stub)
    println!("Validating config...");
    // Real validation logic: check if config is valid JSON
    let config_path = "~/.openclaw/openclaw.json";
    match fs::read_to_string(config_path) {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(_) => println!("Config is valid JSON."),
                Err(e) => println!("Config validation error: {}", e),
            }
        },
        Err(e) => println!("Failed to read config: {}", e),
    }
}

pub fn restart_gateway_after_config() {
    // Restart gateway after config changes
    println!("Restarting gateway after config update...");
    gateway_client::restart_gateway();
}
