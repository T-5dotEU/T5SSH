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
