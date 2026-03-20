
// UI Layer for OpenClaw Linux Desktop App


































































use notify_rust::Notification;
use gtk4::{Application, ApplicationWindow, Box, Orientation, Entry, Button, Label, ScrolledWindow, ComboBoxText, ListBox, TextView};
use gtk4::prelude::*;

// Handles main window, chat interface, tray, notifications
pub fn launch_ui(app: &Application) {
		// Load custom CSS for black background, red/white colors, gradients, and animations
		let provider = gtk4::CssProvider::new();
		provider.load_from_data(r#"
			window, .main-bg {
				background: #000000;
			}
			.sidebar-bg {
				background: linear-gradient(180deg, #18191a 0%, #232526 100%);
				padding: 24px 0 24px 0;
				margin: 0 24px 0 0;
			}
			.sidebar-nav-btn {
				color: #fff;
				background: transparent;
				border-radius: 8px;
				margin: 8px 0;
				padding: 12px 20px;
				transition: background 200ms, color 200ms, box-shadow 200ms;
			}
			.sidebar-nav-btn:hover {
				background: linear-gradient(90deg, #ff3b30 0%, #b22222 100%);
				color: #fff;
				box-shadow: 0 2px 12px #ff3b3080;
			}
			.sidebar-nav-active {
				background: linear-gradient(90deg, #ff3b30 0%, #b22222 100%);
				color: #fff;
				border-left: 4px solid #ff3b30;
				box-shadow: 0 2px 12px #ff3b3080;
				transition: background 200ms, color 200ms, box-shadow 200ms;
			}
			.sidebar-app-name, .sidebar-version {
				color: #fff;
				margin: 12px 0;
			}
			.card-bg {
				background: linear-gradient(180deg, #191b1d 0%, #232526 100%);
				border-radius: 18px;
				box-shadow: 0 4px 32px rgba(0,0,0,0.4);
				color: #fff;
				margin: 24px;
				padding: 32px 32px 24px 32px;
				transition: box-shadow 300ms;
			}
			.card-bg:hover {
				box-shadow: 0 8px 48px #ff3b30cc;
			}
			.coral-btn {
				background: linear-gradient(90deg, #ff3b30 0%, #b22222 100%);
				color: #fff;
				border-radius: 12px;
				font-weight: bold;
				margin: 16px 0 0 0;
				padding: 14px 0;
				transition: background 200ms, box-shadow 200ms;
			}
			.coral-btn:hover {
				background: linear-gradient(90deg, #b22222 0%, #ff3b30 100%);
				box-shadow: 0 2px 12px #ff3b30cc;
			}
			.status-box {
				color: #fff;
				background: #191b1d;
				border-radius: 8px;
				margin: 16px 0 0 0;
				padding: 10px 16px;
			}
			.helper-section {
				color: #fff;
				margin: 16px 0 0 0;
				padding: 8px 0 0 0;
			}
			entry, .input-field {
				padding: 12px 16px;
				border-radius: 8px;
				margin: 8px 0;
			}
		"#);
		gtk4::StyleContext::add_provider_for_display(
			&gtk4::gdk::Display::default().unwrap(),
			&provider,
			gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
		);
	// Main window
	let window = ApplicationWindow::builder()
		.application(app)
		.title("OpenClaw Linux Desktop App")
		.default_width(1100)
		.default_height(700)
		.build();
	// Gateway Connect Card
	 // Load mascot_logo from path relative to executable for portability
	 use std::env;
	 use std::path::{Path, PathBuf};
	 let exe_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
	 let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new("."));
	 let asset_path = exe_dir.join("assets/touch-icon.png");
	 let mascot_logo_path = asset_path.to_str().unwrap_or("assets/touch-icon.png");
	 println!("Mascot logo path: {}", mascot_logo_path);
	 let mascot_logo = gtk4::Image::from_file(mascot_logo_path);
	let gateway_title = Label::new(Some("OpenClaw"));
	gateway_title.set_css_classes(&["gateway-title"]);
	let gateway_subtitle = Label::new(Some("Gateway Dashboard"));
	gateway_subtitle.set_css_classes(&["gateway-subtitle"]);
	let ws_url_entry = Entry::new();
	ws_url_entry.set_placeholder_text(Some("WebSocket URL"));
	let token_entry = Entry::new();
	token_entry.set_placeholder_text(Some("Gateway Token"));
	token_entry.set_visibility(false);
	let password_entry = Entry::new();
	password_entry.set_placeholder_text(Some("Password (optional)"));
	password_entry.set_visibility(false);
	let connect_btn = Button::with_label("Connect");
	connect_btn.set_css_classes(&["coral-btn"]);
	let status_box = Label::new(Some("Status: Not connected"));
	status_box.set_css_classes(&["status-box"]);
	let how_to_connect = Label::new(Some("How to connect:\n1. Start gateway\n2. Retrieve token\n3. Enter details above"));
	how_to_connect.set_css_classes(&["helper-section"]);
	let gateway_card = Box::new(Orientation::Vertical, 16);
	gateway_card.set_css_classes(&["card-bg"]);
	gateway_card.append(&mascot_logo);
	gateway_card.append(&gateway_title);
	gateway_card.append(&gateway_subtitle);
	gateway_card.append(&ws_url_entry);
	gateway_card.append(&token_entry);
	gateway_card.append(&password_entry);
	gateway_card.append(&connect_btn);
	gateway_card.append(&status_box);
	gateway_card.append(&how_to_connect);
	// Center Gateway Card
	let gateway_center = Box::new(Orientation::Vertical, 0);
	gateway_center.set_halign(gtk4::Align::Center);
	gateway_center.set_valign(gtk4::Align::Center);
	gateway_center.append(&gateway_card);
	// Sidebar Navigation (modular, highlight active)
	let sidebar = Box::new(Orientation::Vertical, 16);
	sidebar.set_css_classes(&["sidebar-bg"]);
	let logo = gtk4::Image::from_file("assets/linux-logo.png"); // High-res, transparent logo for dark BG
	let app_name = Label::new(Some("OpenClaw"));
	app_name.set_css_classes(&["sidebar-app-name"]);
	let version = Label::new(Some("v2026.3.20"));
	version.set_css_classes(&["sidebar-version"]);
	sidebar.append(&logo);
	sidebar.append(&app_name);

	// Navigation items (all required sections)
	let nav_items = vec![
		"Chat", "Overview", "Channels", "Instances", "Sessions", "Usage", "Cron Jobs", "Agents", "Skills", "Nodes", "Config", "Communications", "Appearance", "Automation", "Docs"
	];
	for (i, item) in nav_items.iter().enumerate() {
		let nav_btn = Button::with_label(item);
		nav_btn.set_css_classes(&["sidebar-nav-btn"]);
		// Highlight the first item (Chat) as active for now
		if i == 0 {
			nav_btn.add_css_class("sidebar-nav-active");
		}
		sidebar.append(&nav_btn);
	}
	// Spacer to push version to bottom
	let sidebar_spacer = gtk4::Box::new(Orientation::Vertical, 0);
	sidebar_spacer.set_vexpand(true);
	sidebar.append(&sidebar_spacer);
	sidebar.append(&version);
	// Layout: sidebar + main panel
	let hbox = Box::new(Orientation::Horizontal, 0);
	hbox.append(&sidebar);
	hbox.append(&gateway_center);
	window.set_child(Some(&hbox));
	window.show();
}
