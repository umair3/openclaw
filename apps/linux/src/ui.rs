// UI Layer for OpenClaw Linux Desktop App


































































use notify_rust::Notification;
use gtk4::{Application, ApplicationWindow, Box, Orientation, Entry, Button, Label, ScrolledWindow, ComboBoxText, ListBox, TextView};
use gtk4::prelude::*;

// Handles main window, chat interface, tray, notifications
pub fn launch_ui(app: &Application) {
	// Main window
	let window = ApplicationWindow::builder()
		.application(app)
		.title("OpenClaw Linux Agent")
		.default_width(1100)
		.default_height(700)
		.build();

	// Main layout: horizontal box
	let hbox = Box::new(Orientation::Horizontal, 12);

	// Sidebar: session/agent switcher, plugin list, agent status
	let sidebar = Box::new(Orientation::Vertical, 8);
	let session_label = Label::new(Some("Sessions / Agents"));
	let session_switcher = ComboBoxText::new();
	session_switcher.append_text("Default Agent");
	session_switcher.append_text("Agent 2");
	let status_label = Label::new(Some("Status: Running"));

	let plugin_label = Label::new(Some("Plugins"));
	let plugin_list = ListBox::new();
	plugin_list.append(&Label::new(Some("plugin-a")));
	plugin_list.append(&Label::new(Some("plugin-b")));

	sidebar.append(&session_label);
	sidebar.append(&session_switcher);
	sidebar.append(&status_label);
	sidebar.append(&plugin_label);
	sidebar.append(&plugin_list);

	// Main panel: chat interface, agent responses, logs
	let main_panel = Box::new(Orientation::Vertical, 8);

	// Top bar: gateway controls, tray actions, notifications
	let top_bar = Box::new(Orientation::Horizontal, 8);
	let gateway_start = Button::with_label("Start Gateway");
	let gateway_stop = Button::with_label("Stop Gateway");
	let gateway_restart = Button::with_label("Restart Gateway");
	let tray_btn = Button::with_label("Tray Actions");
	let notify_btn = Button::with_label("Notify");

	top_bar.append(&gateway_start);
	top_bar.append(&gateway_stop);
	top_bar.append(&gateway_restart);
	top_bar.append(&tray_btn);
	top_bar.append(&notify_btn);

	main_panel.append(&top_bar);

	// Chat area
	let chat_log = TextView::new();
	chat_log.set_editable(false);
	chat_log.set_cursor_visible(false);
	chat_log.buffer().set_text("Welcome to OpenClaw!\nStreaming responses will appear here.");
	let chat_scrolled = ScrolledWindow::new();
	chat_scrolled.set_child(Some(&chat_log));

	let entry = Entry::new();
	let send_btn = Button::with_label("Send");

	let chat_box = Box::new(Orientation::Horizontal, 8);
	chat_box.append(&entry);
	chat_box.append(&send_btn);

	main_panel.append(&chat_scrolled);
	main_panel.append(&chat_box);

	// Logs & Debugging area
	let logs_label = Label::new(Some("Logs & Debugging"));
	let logs_view = TextView::new();
	logs_view.set_editable(false);
	logs_view.buffer().set_text("Log output will appear here.");
	let logs_scrolled = ScrolledWindow::new();
	logs_scrolled.set_child(Some(&logs_view));

	let filter_btn = Button::with_label("Filter Logs");
	let export_btn = Button::with_label("Export Logs");

	let logs_box = Box::new(Orientation::Horizontal, 8);
	logs_box.append(&filter_btn);
	logs_box.append(&export_btn);

	main_panel.append(&logs_label);
	main_panel.append(&logs_scrolled);
	main_panel.append(&logs_box);

	// Plugin manager area
	let plugin_mgr_label = Label::new(Some("Plugin Manager"));
	let install_entry = Entry::new();
	install_entry.set_placeholder_text(Some("npm spec (e.g. @openclaw/plugin)"));
	let install_btn = Button::with_label("Install Plugin");

	let enable_btn = Button::with_label("Enable Plugin");
	let disable_btn = Button::with_label("Disable Plugin");
	let plugin_log_btn = Button::with_label("Show Plugin Logs");

	let plugin_mgr_box = Box::new(Orientation::Horizontal, 8);
	plugin_mgr_box.append(&install_entry);
	plugin_mgr_box.append(&install_btn);
	plugin_mgr_box.append(&enable_btn);
	plugin_mgr_box.append(&disable_btn);
	plugin_mgr_box.append(&plugin_log_btn);

	main_panel.append(&plugin_mgr_label);
	main_panel.append(&plugin_mgr_box);

	// Config editor area
	let config_label = Label::new(Some("Configuration"));
	let config_edit_btn = Button::with_label("Edit Config");
	let config_validate_btn = Button::with_label("Validate Config");
	let config_restart_btn = Button::with_label("Restart Gateway After Config");

	let config_box = Box::new(Orientation::Horizontal, 8);
	config_box.append(&config_edit_btn);
	config_box.append(&config_validate_btn);
	config_box.append(&config_restart_btn);

	main_panel.append(&config_label);
	main_panel.append(&config_box);

	// Security awareness area
	let security_label = Label::new(Some("Security Awareness"));
	let shell_warn_btn = Button::with_label("Show Shell Warning");
	let file_perm_btn = Button::with_label("Request File Permission");
	let sandbox_btn = Button::with_label("Enable Sandbox");

	let security_box = Box::new(Orientation::Horizontal, 8);
	security_box.append(&shell_warn_btn);
	security_box.append(&file_perm_btn);
	security_box.append(&sandbox_btn);

	main_panel.append(&security_label);
	main_panel.append(&security_box);

	// Keyboard navigation and dark mode scaffold
	gtk4::prelude::GtkWindowExt::set_focus(&window, Some(&entry)); // Keyboard-first
	// For dark mode styling, use CSS provider (not set_css_name)

	// Assemble layout
	hbox.append(&sidebar);
	hbox.append(&main_panel);
	window.set_child(Some(&hbox));

	Notification::new()
		.summary("OpenClaw Agent Ready")
		.body("The agent is running.")
		.show().ok();

	window.show();
}
