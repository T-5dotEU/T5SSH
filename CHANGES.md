# Changelog

## 1.9.0-dev — 2026-03-28

### Features

- Terminal mouse support: set TERM=xterm-256color for SSH sessions, enabling mouse tracking in remote applications (mc, htop, etc.)
- Confirmed: mouse support working on Linux
- Windows test pending: build via GitHub Actions CI

## 1.8.0 — 2026-03-28

- Release: all 1.7.x-dev fixes confirmed on Windows and Linux

## 1.7.3-dev — 2026-03-28

### Bugfixes

- Fixed window height growing on each restart: save inner_size instead of outer_size (excludes title bar decoration)

## 1.7.2-dev — 2026-03-28

### Bugfixes

- Fixed UI font: system sans-serif stack (Segoe UI on Windows) instead of serif default
- Fixed window geometry restore: use physical pixels to match save (broken with HiDPI/Windows scaling)

## 1.7.1-dev — 2026-03-28

### Bugfixes

- Default dark theme on first start (no more missing color scheme)
- Fixed initial terminal width mismatch: PTY size synced after session creation
- Confirmed: SSH login and terminal output working on Windows

## 1.7.0-dev — 2026-03-27

### Bugfixes

- Fixed race condition: event listeners now registered before session creation (SSH output was lost on Windows)
- Fixed Windows SSH_ASKPASS script: now only responds to password prompts (was replying to all prompts)
- Terminal shows "Connecting..." feedback during connection setup

## 1.6.0 — 2026-03-27

- Release version bump (no code changes)

## 1.5.3-dev — 2026-03-27

### CI/CD

- GitHub Actions: Node.js 20 → 24 (deprecation fix)

## 1.5.2-dev — 2026-03-27

### UI/UX

- Middle-click on tab closes the tab
- Dark background in app.html to prevent white flash on load
- Fixed window title (was "Tauri + SvelteKit App", now "T5SSH")

## 1.5.1-dev — 2026-03-27

### CI/CD

- GitHub Actions workflow for .deb, .rpm and Windows builds

### Code Quality

- Fixed all 135 svelte-check errors: JSDoc type annotations for all JS/Svelte files
- Fixed 2 Clippy warnings: removed unused import, inlined let binding
- Removed unused CSS selector in SettingsDialog
- Result: 0 errors, 0 warnings across both Rust (clippy) and Svelte (svelte-check)

## 1.5.0-dev — 2026-03-27

### Branding

- Renamed project from UltraSSH to T5SSH (package.json, tauri.conf.json, Cargo.toml, window title, app identifier)

## 1.4.0 — 2026-03-27

### UI/UX Improvements

- Settings dialog: font dropdown (8 monospace fonts), font size, color scheme (Dark/Classic), scrollback lines
- Settings are persisted to settings.json and applied live to running terminals
- Shared color scheme module (colorSchemes.js) — single source of truth for terminal themes
- Double-click profile = direct connect
- Last tab close: confirmation dialog before quitting
- Swapped Connect / Save&Connect button positions
- Eye icon for password toggle: red only when password visible
- Unsaved changes confirmation when switching/loading profiles
- Settings button (⚙) in start dialog for quick access
- Identity file field: native file picker button (📂) via Tauri dialog plugin
- Removed green profile banner (cleaner UI)
- Global dark scrollbars (16px wide, WebKit-styled)
- Dark native widgets via `color-scheme: dark` (selects, inputs, checkboxes)

### Backend

- Cross-platform password storage: keyring crate with linux-native, apple-native, windows-native features
- Cross-platform SSH_ASKPASS: Unix shell script + Windows .cmd batch script
- Tauri dialog plugin (tauri-plugin-dialog) for native file picker
- TerminalSettings: font_family, font_size, color_scheme, scrollback_lines (default 10000)
- Settings IPC: get_settings, update_settings commands

### Bug Fixes

- Color schemes were duplicated in SettingsDialog and Terminal with different values — extracted to shared module
- Window geometry save no longer overwrites terminal settings

## 1.2.1 — 2026-03-27

- Fix: SSH_ASKPASS script now deleted after session ends (was deleted too early before SSH could read it)
- Fix: Stored passwords loaded from keyring when switching profiles (new get_password IPC command)
- Fix: Password show/hide toggle now uses direct DOM manipulation (Svelte reactive type binding workaround)
- UX: Green success message after profile save with 2.5s auto-dismiss

## 1.2.0 — 2026-03-27

- Password manager: store SSH passwords securely in the system keyring (libsecret/GNOME Keyring)
- Password field in ConnectionDialog with show/hide toggle
- "Save Password" checkbox — stores password in system keyring, never in profiles.json
- 🔑 indicator on profiles with stored passwords
- SSH_ASKPASS mechanism for password delivery (SSH-native, only responds to password prompts)
- Askpass script uses SSH_ASKPASS_REQUIRE=prefer — host key verification still works interactively
- IPC commands: store_password, has_password, delete_password
- Deleting a profile also cleans up its stored password from the keyring
- keyring 3 crate with linux-native feature (libsecret backend)

## 1.0.1 — 2026-03-27

- Security: Config directory permissions set to 0700, files to 0600 (profiles.json, settings.json)
- Security: StrictHostKeyChecking changed from accept-new to ask (interactive host key verification)
- Security: ConnectTimeout=30 added to prevent indefinite connection hangs
- Security: Removed user@host from log output (session ID only)
- Security: Removed unused tauri-plugin-opener (reduced attack surface)

## 1.0.0 — 2026-03-27

- Consistent button color scheme across all components:
  - Primary (blue): Connect/Go actions
  - Success (green): Add/Save actions, + tab button
  - Cancel (red outline): Cancel buttons
  - Danger (red): Delete actions
  - Secondary (gray): Neutral actions
- Hide Cancel button in connection dialog when no tabs are open
- Fix: ProfileList horizontal scrollbar removed (overflow-x: hidden)

## 0.2.1 — 2026-03-27

- Fix: Ctrl+Shift+C/V now handled directly in xterm key handler (previously not working)
- Fix: Profile delete now requires confirmation dialog
- Fix: Zombie processes — child.wait() after child.kill() in all session cleanup paths
- Fix: Removed unused SessionState variants (Creating, Connecting) and unused import
- Fix: Mutex .unwrap() replaced with proper error handling in IPC commands
- Fix: .expect() on window lookup replaced with graceful match
- Fix: Atomic file writes for settings.json and profiles.json (write to .tmp, then rename)
- Fix: Deduplicated window geometry save code into helper function
- Fix: Context menu viewport bounds check (no off-screen rendering)
- Fix: Clipboard readText() .catch() for denied permissions
- Fix: terminalRefs cleaned up on tab close (memory leak)
- Fix: invoke('quit_app') error handling
- ProfileList dialog: added "+ New Connection" button
- Cargo check: 0 warnings, 0 errors

## 0.2.0 — 2026-03-27

- Copy/Paste: Ctrl+Shift+C / Ctrl+Shift+V (like KDE Konsole)
- Reconnect button (↻) on disconnected tabs to re-establish SSH session in-place
- Close confirmation: closing a tab with an active session requires confirmation
- Close confirmation: closing the app window with active sessions requires confirmation
- SSH crash/disconnect handling: session marked as disconnected with reconnect option
- Last tab closed → app quits (instead of showing connection dialog)
- IPC commands: close_all_sessions, quit_app
- Rust: drain_all() on SessionManager for clean shutdown
- Window close interception via CloseRequested event + frontend confirmation

## 0.1.0 — 2026-03-26

- Initial project scaffold (Tauri v2 + Svelte 5)
- Basic xterm.js terminal rendering with FitAddon and resize support
- PTY backend module (portable-pty) with create/resize abstraction
- Session management module with lifecycle state tracking
- SSH command builder with support for user, port, identity file, jump host, port forwarding, agent forwarding
- Profile and PortForward data models (serde-serializable)
- Tauri IPC commands: create_session, send_input, resize_session, close_session, list_sessions
- Async PTY output reader with session:output, session:exit, session:error events
- Tracing (structured logging) initialization
- Frontend API layer (session.js) wrapping Tauri invoke/listen calls
- Terminal component wired to backend: SSH output, keyboard input, resize sync
- End-to-end SSH connection via PTY working
- Tab management: TabStore with Svelte 5 runes ($state) for reactive tab tracking
- TabBar component with add/close/switch tabs, active and disconnected styling
- Multi-terminal composition: each tab runs its own Terminal instance with session isolation
- Visibility-based tab switching (hidden tabs keep PTY sessions alive)
- Fix: terminal sizing now uses ResizeObserver and requestAnimationFrame for accurate initial dimensions
- Connection dialog: host, port, username, identity file, jump host, port forwards, agent forwarding
- Profile persistence: save/load/delete profiles to ~/.config/t5ssh/profiles.json
- Profile list dialog with saved profiles, click to connect, delete
- Tab "+" button opens connection dialog, "☰" button opens saved profiles
- IPC commands: save_profile, load_profiles, delete_profile
- Auto-focus terminal after connect (password input works immediately)
- Profiles button highlighted in tab bar for better discoverability
- Saved profiles listed in connection dialog with quick-connect on click
- Right-click context menu on profiles (Edit / Delete)
- Profile editing loads data into form with visual indicator
- Profile click now loads into form (instead of instant connect), with scroll to Connect button
- Window geometry (position & size) saved to ~/.config/t5ssh/settings.json and restored on startup
- Profile list (☰) click loads profile into ConnectionDialog instead of direct connect
- Sticky "ready to connect" banner with ▶ Connect button pinned at top of dialog when profile loaded
- Right-click context menu (Edit / Delete) in profile list dialog, consistent with connection dialog
