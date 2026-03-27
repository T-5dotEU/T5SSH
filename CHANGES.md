# Changelog

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
- Profile persistence: save/load/delete profiles to ~/.config/ultrassh/profiles.json
- Profile list dialog with saved profiles, click to connect, delete
- Tab "+" button opens connection dialog, "☰" button opens saved profiles
- IPC commands: save_profile, load_profiles, delete_profile
- Auto-focus terminal after connect (password input works immediately)
- Profiles button highlighted in tab bar for better discoverability
- Saved profiles listed in connection dialog with quick-connect on click
- Right-click context menu on profiles (Edit / Delete)
- Profile editing loads data into form with visual indicator
- Profile click now loads into form (instead of instant connect), with scroll to Connect button
- Window geometry (position & size) saved to ~/.config/ultrassh/settings.json and restored on startup
