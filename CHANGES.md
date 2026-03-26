# Changelog

## 0.1.0 — 2026-03-26

- Initial project scaffold (Tauri v2 + Svelte 5)
- Basic xterm.js terminal rendering with FitAddon and resize support
- PTY backend module (portable-pty) with create/resize abstraction
- Session management module with lifecycle state tracking
- SSH command builder with support for user, port, identity file, jump host, port forwarding, agent forwarding
- Profile and PortForward data models (serde-serializable)
