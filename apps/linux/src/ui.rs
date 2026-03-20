// UI Layer for OpenClaw Linux Desktop App

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, ComboBoxText, Entry, Label, ListBox, Orientation,
    ScrolledWindow, TextView,
};
use notify_rust::Notification;
use std::cell::RefCell;
use std::rc::Rc;

// Handles main window, chat interface, tray, notifications
pub fn launch_ui(app: &Application) {
    // Load custom CSS for black background, red/white colors, gradients, and animations
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(
        r#"
            window, .main-bg {
                background: #000000;
            }
            .sidebar-bg {
                background: linear-gradient(180deg, #18191a 0%, #232526 100%);
                padding: 24px 0 24px 0;
                margin: 0 24px 0 0;
                min-width: 320px;
                max-width: 340px;
            }
            .sidebar-nav-btn {
                color: #fff;
                background: transparent;
                border-radius: 8px;
                margin: 8px 16px;
                padding: 12px 24px;
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
            .sidebar-section-label {
                font-weight: bold;
                font-size: 14px;
                letter-spacing: 1px;
                margin: 16px 0 8px 0;
                color: #f2f2f2;
            }
            .cmd-box {
                background: #232526;
                border-radius: 8px;
                border: 1px solid #ff3b30;
                padding: 8px 16px;
                margin: 8px 0;
                box-shadow: 0 2px 8px #ff3b3040;
                align-items: center;
            }
            .play-btn {
                background: linear-gradient(90deg, #ff3b30 0%, #b22222 100%);
                color: #fff;
                border-radius: 8px;
                font-weight: bold;
                margin-left: 16px;
                padding: 2px 8px;
                font-size: 12px;
                min-width: 32px;
                min-height: 24px;
                max-width: 32px;
                max-height: 24px;
                box-shadow: 0 2px 8px #ff3b3040;
                transition: background 200ms, box-shadow 200ms;
            }
            .play-btn:hover {
                background: linear-gradient(90deg, #b22222 0%, #ff3b30 100%);
                box-shadow: 0 4px 16px #ff3b30cc;
            }
		"#,
    );
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
    window.fullscreen();
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
    // How to connect section
    let how_to_connect_box = Box::new(Orientation::Vertical, 8);
    how_to_connect_box.set_halign(gtk4::Align::Start);
    let how_heading = Label::new(Some("How to connect"));
    how_heading.set_css_classes(&["helper-section-heading"]);
    how_heading.set_markup("<b>How to connect</b>");
    how_heading.set_halign(gtk4::Align::Start);
    how_to_connect_box.append(&how_heading);

    let step1 = Label::new(Some("1. Start the gateway on your host machine:"));
    step1.set_css_classes(&["helper-section-point"]);
    step1.set_halign(gtk4::Align::Start);
    how_to_connect_box.append(&step1);
    let cmd1_box = Box::new(Orientation::Horizontal, 8);
    cmd1_box.set_css_classes(&["cmd-box"]);
    cmd1_box.set_hexpand(true);
    let cmd1 = Label::new(Some("openclaw gateway run"));
    cmd1.set_css_classes(&["helper-section-cmd"]);
    cmd1.set_markup("<tt>openclaw gateway run</tt>");
    cmd1.set_halign(gtk4::Align::Start);
    let play_btn1_spacer = gtk4::Box::new(Orientation::Horizontal, 0);
    play_btn1_spacer.set_hexpand(true);
    let play_btn1 = Button::with_label("▶");
    play_btn1.set_css_classes(&["play-btn"]);
    play_btn1.set_hexpand(false);
    play_btn1.set_valign(gtk4::Align::Center);
    play_btn1.set_width_request(32);
    play_btn1.set_height_request(24);
    cmd1_box.append(&cmd1);
    cmd1_box.append(&play_btn1_spacer);
    cmd1_box.append(&play_btn1);
    how_to_connect_box.append(&cmd1_box);

    let step2 = Label::new(Some("2. Get a tokenized dashboard URL:"));
    step2.set_css_classes(&["helper-section-point"]);
    step2.set_halign(gtk4::Align::Start);
    how_to_connect_box.append(&step2);
    let cmd2_box = Box::new(Orientation::Horizontal, 8);
    cmd2_box.set_css_classes(&["cmd-box"]);
    cmd2_box.set_hexpand(true);
    let cmd2 = Label::new(Some("openclaw dashboard --no-open"));
    cmd2.set_css_classes(&["helper-section-cmd"]);
    cmd2.set_markup("<tt>openclaw dashboard --no-open</tt>");
    cmd2.set_halign(gtk4::Align::Start);
    let play_btn2_spacer = gtk4::Box::new(Orientation::Horizontal, 0);
    play_btn2_spacer.set_hexpand(true);
    let play_btn2 = Button::with_label("▶");
    play_btn2.set_css_classes(&["play-btn"]);
    play_btn2.set_hexpand(false);
    play_btn2.set_valign(gtk4::Align::Center);
    play_btn2.set_width_request(32);
    play_btn2.set_height_request(24);
    cmd2_box.append(&cmd2);
    cmd2_box.append(&play_btn2_spacer);
    cmd2_box.append(&play_btn2);
    how_to_connect_box.append(&cmd2_box);

    let step3 = Label::new(Some(
        "3. Paste the WebSocket URL and token above, or open the tokenized URL directly.",
    ));
    step3.set_css_classes(&["helper-section-point"]);
    step3.set_halign(gtk4::Align::Start);
    how_to_connect_box.append(&step3);

    let docs_link = Label::new(Some(
        "<span color='red'><a href='https://docs.openclaw.ai/gateway'>Read the docs →</a></span>",
    ));
    docs_link.set_css_classes(&["helper-section-link"]);
    docs_link.set_halign(gtk4::Align::Start);
    docs_link.set_markup(
        "<span color='red'><a href='https://docs.openclaw.ai/gateway'>Read the docs →</a></span>",
    );
    how_to_connect_box.append(&docs_link);
    let gateway_card = Box::new(Orientation::Vertical, 16);
    gateway_card.set_css_classes(&["card-bg"]);
    gateway_card.set_halign(gtk4::Align::Center);
    gateway_card.set_valign(gtk4::Align::Center);
    gateway_card.set_width_request(700); // Double the width
    gateway_card.append(&mascot_logo);
    gateway_card.append(&gateway_title);
    gateway_card.append(&gateway_subtitle);
    gateway_card.append(&ws_url_entry);
    gateway_card.append(&token_entry);
    gateway_card.append(&password_entry);
    gateway_card.append(&connect_btn);
    gateway_card.append(&status_box);
    gateway_card.append(&how_to_connect_box);
    // Center Gateway Card
    let gateway_center = Box::new(Orientation::Vertical, 0);
    gateway_center.set_halign(gtk4::Align::Center);
    gateway_center.set_valign(gtk4::Align::Center);
    gateway_center.append(&gateway_card);
    // Sidebar Navigation (modular, highlight active)
    let sidebar = Box::new(Orientation::Vertical, 16);
    sidebar.set_css_classes(&["sidebar-bg"]);
    let logo = gtk4::Image::from_file(mascot_logo_path);
    logo.set_pixel_size(48);
    let app_name = Label::new(Some("OpenClaw"));
    app_name.set_css_classes(&["sidebar-app-name"]);
    sidebar.append(&logo);
    sidebar.append(&app_name);

    // Scrollable section container
    let scrollable_container = gtk4::ScrolledWindow::new();
    scrollable_container.set_vexpand(true);
    let scroll_content = Box::new(Orientation::Vertical, 12);

    // Section: CHAT
    let chat_section = Box::new(Orientation::Vertical, 8);
    let chat_label = Label::new(Some("CHAT"));
    chat_label.set_css_classes(&["sidebar-section-label"]);
    chat_section.append(&chat_label);
    let chat_btn = Button::with_label("Chat");
    chat_btn.set_css_classes(&["sidebar-nav-btn", "sidebar-nav-active"]);
    chat_section.append(&chat_btn);
    scroll_content.append(&chat_section);

    // Section: CONTROL
    let control_section = Box::new(Orientation::Vertical, 8);
    let control_label = Label::new(Some("CONTROL"));
    control_label.set_css_classes(&["sidebar-section-label"]);
    control_section.append(&control_label);
    let control_items = vec![
        "Overview",
        "Channels",
        "Instances",
        "Sessions",
        "Usage",
        "Cron Jobs",
    ];
    for item in control_items {
        let btn = Button::with_label(item);
        btn.set_css_classes(&["sidebar-nav-btn"]);
        control_section.append(&btn);
    }
    scroll_content.append(&control_section);

    // Section: AGENT
    let agent_section = Box::new(Orientation::Vertical, 8);
    let agent_label = Label::new(Some("AGENT"));
    agent_label.set_css_classes(&["sidebar-section-label"]);
    agent_section.append(&agent_label);
    let agent_items = vec!["Agents", "Skills", "Nodes"];
    for item in agent_items {
        let btn = Button::with_label(item);
        btn.set_css_classes(&["sidebar-nav-btn"]);
        agent_section.append(&btn);
    }
    scroll_content.append(&agent_section);

    // Section: SETTINGS
    let settings_section = Box::new(Orientation::Vertical, 8);
    let settings_label = Label::new(Some("SETTINGS"));
    settings_label.set_css_classes(&["sidebar-section-label"]);
    settings_section.append(&settings_label);
    let settings_items = vec![
        "Config",
        "Communications",
        "Appearance",
        "Automation",
        "Infrastructure",
        "AI & Agents",
        "Debug",
        "Logs",
    ];
    for item in settings_items {
        let btn = Button::with_label(item);
        btn.set_css_classes(&["sidebar-nav-btn"]);
        settings_section.append(&btn);
    }
    scroll_content.append(&settings_section);

    scrollable_container.set_child(Some(&scroll_content));
    sidebar.append(&scrollable_container);

    // Separator line
    let separator = gtk4::Box::new(Orientation::Horizontal, 0);
    separator.set_css_classes(&["sidebar-separator"]);
    separator.set_height_request(2);
    sidebar.append(&separator);

    // Fixed bottom section: Docs and Version
    let bottom_section = Box::new(Orientation::Vertical, 8);
    let docs_btn = Button::with_label("Docs");
    docs_btn.set_css_classes(&["sidebar-nav-btn"]);
    let version_label = Label::new(Some("Version: v2026.3.20"));
    version_label.set_css_classes(&["sidebar-version"]);
    bottom_section.append(&docs_btn);
    bottom_section.append(&version_label);
    sidebar.append(&bottom_section);
    // State management for screen transitions
    let show_gateway_card = Rc::new(RefCell::new(true));
    let show_gateway_card_clone = show_gateway_card.clone();

    // Connect button handler
    let window_clone = window.clone();
    let sidebar_clone = sidebar.clone();
    let chat_section_clone = chat_section.clone();
    connect_btn.connect_clicked(move |_| {
        *show_gateway_card_clone.borrow_mut() = false;
        let hbox = Box::new(Orientation::Horizontal, 0);
        hbox.append(&sidebar_clone);
        hbox.append(&chat_section_clone);
        window_clone.set_child(Some(&hbox));
    });

    // Layout logic
    if *show_gateway_card.borrow() {
        // Show only gateway card, centered
        window.set_child(Some(&gateway_center));
    } else {
        // Show sidebar and main panels (Chat by default)
        let hbox = Box::new(Orientation::Horizontal, 0);
        hbox.append(&sidebar);
        hbox.append(&chat_section); // Use chat_section as placeholder for chat panel
        window.set_child(Some(&hbox));
    }
    window.show();
}
