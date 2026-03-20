# 🧠 Prompt: Build a Linux Desktop App for OpenClaw

## Context
OpenClaw is a local-first AI agent platform that runs on user machines and interacts via messaging platforms, system tools, and a local gateway. It is cross-platform in principle, but currently has mature apps for macOS, iOS, and Android.
There is currently NO native Linux (or Windows) desktop application.
The goal is to build a Linux app with a feature set comparable to the macOS app.

---

## 🎯 Objective
Create a production-ready Linux desktop application for OpenClaw that:
- Acts as a control interface for the OpenClaw gateway
- Provides system-level integration (notifications, tray, file access)
- Supports real-time interaction with the agent
- Is stable, secure, and extensible

---

## ⚙️ Functional Requirements

### 1. Core App Features
- Chat interface to interact with OpenClaw agent
- Display agent responses, tool outputs, and logs
- Support multi-session / multi-agent switching
- Show agent status (running, idle, executing tasks)

### 2. Gateway Integration
- Connect to local OpenClaw gateway via WebSocket (localhost)
- Auto-detect running gateway instance
- Ability to start/stop/restart gateway
- Display logs and errors from gateway

### 3. System Integration
- System tray icon with quick actions:
	- Open app
	- Pause agent
	- Restart gateway
- Native Linux notifications (libnotify / DBus)
- File system access

### 4. Plugin / Tool Visibility
- List installed plugins
- Enable/disable plugins
- Show plugin logs and activity
- Install plugins via npm spec

### 5. Security Awareness (CRITICAL)
- Show warnings for shell execution
- Permission prompts for file/system access
- Optional sandboxing (Flatpak/AppArmor)

### 6. Configuration UI
- Edit ~/.openclaw/openclaw.json visually
- Validate config
- Restart gateway after changes

### 7. Logs & Debugging
- View logs from ~/.openclaw/logs/
- Filter logs
- Export logs

---

## 🧩 Non-Functional Requirements
- Linux support (Ubuntu, Arch, Fedora)
- Packaging:
	- AppImage
	- .deb
	- Flatpak

---

## 🛠 Tech Stack
Choose one:
- Native (GTK)

---

## 🎨 UX Requirements
- Fast startup (<2s)
- Keyboard-first navigation
- Dark mode (pure black background: #0f1113 or #000000)
- Use red (#ff3b30/coral) and white (#ffffff) as primary accent and text colors everywhere else
- Use gradients in shades of the same color for backgrounds, buttons, and cards wherever possible (e.g., sidebar, active nav, buttons)
- Add shadows to cards and buttons on hover for depth and feedback
- Logo must be high-resolution, with transparent background, and visually optimized for dark backgrounds
- Add subtle animations for transitions, button hovers, and panel switching (e.g., fade, slide, scale)
- Real-time streaming responses

---

## 🏗 Architecture
- UI Layer
- IPC Layer
- Gateway Client
- Plugin Manager
- Config Manager
- Logs Module

---

## 🚀 Implementation Plan
1. MVP
2. System Integration
3. Plugin + Config UI
4. Security Layer
5. Packaging


---

## UI Design Prompt (Screens)

Design a modern, dark-themed Linux desktop app for OpenClaw, matching the layout and style in the provided screenshots, with the following additional visual requirements:

- The app background must be pure black ( #0f1113 or #000000).
- All UI elements (text, icons, borders) should use white ( #ffffff) and red/coral ( #ff3b30) as primary colors.
- Use gradients in shades of the same color for backgrounds, buttons, and cards wherever possible (e.g., sidebar, active nav, buttons).
- Add shadows to cards and buttons on hover for depth and feedback.
- Logo must be high-resolution, with transparent background, and visually optimized for dark backgrounds.
- Add subtle animations for transitions, button hovers, and panel switching (e.g., fade, slide, scale).

### 1. Gateway Connect (connect.png)
On launch, show a centered, elevated card on a dark background.
Top: OpenClaw mascot/logo, title "OpenClaw", subtitle "Gateway Dashboard".
Three vertically stacked input fields:
	- WebSocket URL (pre-filled, full-width, with validation and error display)
	- Gateway Token (masked, toggle-eye icon to show/hide)
	- Password (optional, masked, with helper text)
Large, full-width "Connect" button in coral/red, rounded corners.
Status box below button for connection feedback (error in red, success in green).
"How to connect" section with numbered steps and code blocks for gateway setup and token retrieval.
Small user/status icon in top-right corner for additional actions.
All elements keyboard-accessible, visually spaced, matching the dark card style, colors, and spacing.

### 2. Main App Layout (chat.png, overview-1.png, overview-2.png)
Left sidebar: vertical navigation with sections for Chat, Control, Agent, Settings, Docs.
Sidebar includes OpenClaw logo, app name, and version indicator at the bottom.
Sidebar items: Chat, Overview, Channels, Instances, Sessions, Usage, Cron Jobs, Agents, Skills, Nodes, Config, Communications, Appearance, Automation, Docs.
Active section highlighted with coral/red accent.
Main content area:
	- **Chat view**: 
		- Top dropdowns for session and agent selection.
		- Message history with agent/tool responses, warnings, and user messages.
		- Message bubbles styled with rounded corners, agent/user avatars, and tool output blocks.
		- Input bar at bottom: text entry, send button (coral/red), attachment and emoji icons.
		- "New messages" indicator for unread messages.
	- **Overview view**:
		- Gateway Access card: WebSocket URL, Gateway Token, Password, Session Key, Language, Connect/Refresh buttons.
		- Snapshot card: status (OK/attention), uptime, tick interval, last refresh, recent sessions, skills, jobs, cost.
		- Attention card for warnings (skills with missing dependencies).
		- Event log and gateway logs in split panels.
		- Recent sessions listed as pill buttons.
Top bar: search input, theme toggle, settings, help, and user avatar.
Responsive layout: cards and panels adjust to window size, maintain spacing and alignment.
Consistent dark palette: background (#0f1113), card (#191b1d), accent coral/red, error vivid red, success green, muted gray for secondary actions.
All controls accessible, keyboard-first navigation, clear focus indicators.

### 3. General UX
Fast startup (<2s).
Real-time streaming responses in chat.
Inline validation and helper text for all inputs.
Tool output and agent status clearly surfaced.
Graceful error handling, offline-first, no root required.

---

**Implementation Notes**
- Use GTK for native Linux UI.
- All work in apps/linux directory.
- Match spacing, colors, and widget hierarchy as in screenshots.
- Provide accessibility labels and keyboard shortcuts.
- Make sidebar and main panels modular for easy extension.

---

## 📦 Deliverables
- Project scaffold
- Build instructions
- Packaging scripts

---

## ⚠️ Constraints
- Offline-first
- No root required
- Graceful error handling

## Working Directory
- All work must be done in apps/linux directory
