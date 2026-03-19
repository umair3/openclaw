// OpenClaw Linux Desktop App (GTK)
// Entry point for the native Linux app
mod ui;
mod gateway_client;
mod ipc_layer;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

fn main() {
    // Initialize GTK application
    let app = Application::builder()
        .application_id("ai.openclaw.linux")
        .build();
    app.connect_activate(|app| {
        // Launch UI
        ui::launch_ui(app);
        // Set up IPC
        ipc_layer::setup_ipc();
        // Connect to gateway
        gateway_client::connect_gateway();
    });
    println!("OpenClaw Linux Desktop App starting...");
    app.run();
}
