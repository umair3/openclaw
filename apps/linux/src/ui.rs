// UI Layer for OpenClaw Linux Desktop App


































































use notify_rust::Notification;
use gtk4::{Application, ApplicationWindow, Box, Orientation, Entry, Button, Label, ScrolledWindow};
use gtk4::prelude::*;

// Handles main window, chat interface, tray, notifications
pub fn launch_ui(app: &Application) {
	// Main window
	let window = ApplicationWindow::builder()
		.application(app)
		.title("OpenClaw Linux Agent")
		.default_width(800)
		.default_height(600)
		.build();

	// Chat interface
	let vbox = Box::new(Orientation::Vertical, 8);
	let chat_log = Label::new(Some("Welcome to OpenClaw!"));
	let entry = Entry::new();
	let send_btn = Button::with_label("Send");
	let scrolled = ScrolledWindow::new();
	scrolled.set_child(Some(&chat_log));
	vbox.append(&scrolled);
	vbox.append(&entry);
	vbox.append(&send_btn);
	window.set_child(Some(&vbox));

	// Tray icon and quick actions (placeholder)
	// TODO: Implement tray/quick actions using libappindicator or supported GTK4 patterns

	Notification::new()
		.summary("OpenClaw Agent Ready")
		.body("The agent is running.")
		.show().ok();

	window.show();
}
