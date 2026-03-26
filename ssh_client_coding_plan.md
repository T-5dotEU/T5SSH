# SSH Client Coding Plan

## 1. Zielbild

Desktop-App mit Tabs, die jeweils eine SSH-Session darstellen,
implementiert als Rust-basierter Session-Orchestrator mit Web-UI.

## 2. Tech Stack

Backend: - Rust - Tokio - portable-pty - tracing

Frontend: - Tauri - Svelte - xterm.js

SSH: - OpenSSH subprocess

## 3. Architektur

Svelte UI → xterm.js → Tauri IPC → Rust Backend → PTY → ssh Prozess

## 4. Kernkonzepte

-   1 Tab = 1 Session
-   Backend ist Source of Truth
-   SSH wird nicht selbst implementiert

## 5. Projektstruktur

src-tauri/src: - main.rs - session/ - pty/ - ssh/ - ipc/ - models/

Frontend: - lib/terminal - lib/tabs - lib/api

## 6. Backend Module

Session: - Lifecycle & State

PTY: - create, read/write, resize

SSH: - Command Builder

IPC: - create_session - send_input - resize - close_session -
list_sessions

Models: - Profile - Forwarding

## 7. IPC Design

Commands: - create_session - send_input - resize - close_session

Events: - session:output - session:exit - session:error

## 8. Frontend

-   Tabs verwalten Sessions
-   xterm.js pro Tab
-   API Layer via invoke/listen

## 9. SSH Features

-   Port Forwarding (-L)
-   Jump Hosts (-J)
-   Agent Forwarding
-   \~/.ssh/config Support

## 10. Lifecycle

CREATE → CONNECTING → CONNECTED → CLOSED

## 11. Technische Details

-   async IO
-   PTY resize
-   raw bytes handling

## 12. Logging

-   Rust: tracing
-   Frontend: console
-   optional Debug Panel

## 13. Persistenz

-   Profiles (JSON)

## 14. UX Prinzipien

-   minimalistisch
-   defaults first

## 15. Entwicklungsphasen

0 Setup 1 Terminal 2 PTY 3 SSH 4 Tabs 5 Features 6 Stabilisierung

## 16. Coding Tasks

1 Scaffold 2 Terminal 3 PTY 4 SSH 5 Session Manager 6 IPC 7 Tabs 8
Profiles

## 17. Erfolgsfaktoren

-   kein eigener SSH Stack
-   kein eigener Terminal
-   klare IPC
-   Logging

## 18. Ergebnis

-   schlank
-   stabil
-   OpenSSH Power
