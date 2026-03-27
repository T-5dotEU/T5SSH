use crate::pty::{create_pty, resize_pty};
use crate::session::{Session, SessionInfo, SessionManager, SessionState};
use crate::ssh::{build_ssh_command, SshProfile};
use serde::Serialize;
use std::io::{Read, Write};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use tauri::{AppHandle, Emitter, State};
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Clone, Serialize)]
struct SessionOutput {
    session_id: String,
    data: Vec<u8>,
}

#[derive(Clone, Serialize)]
struct SessionEvent {
    session_id: String,
}

#[tauri::command]
pub async fn create_session(
    app: AppHandle,
    state: State<'_, SessionManager>,
    profile: SshProfile,
    rows: Option<u16>,
    cols: Option<u16>,
    password: Option<String>,
    profile_name: Option<String>,
) -> Result<String, String> {
    let session_id = Uuid::new_v4().to_string();
    let rows = rows.unwrap_or(24);
    let cols = cols.unwrap_or(80);

    let label = if let Some(ref user) = profile.user {
        format!("{}@{}", user, profile.host)
    } else {
        profile.host.clone()
    };

    info!(session_id = %session_id, "Creating SSH session");

    let mut command = build_ssh_command(&profile);

    // Resolve password: explicit password takes priority, then keyring lookup
    let resolved_password = if password.is_some() {
        password
    } else if let Some(ref pname) = profile_name {
        crate::secrets::get_password(pname).unwrap_or(None)
    } else {
        None
    };

    // Set up SSH_ASKPASS if we have a password
    let askpass_path = if let Some(ref pw) = resolved_password {
        let tmp_dir = std::env::temp_dir();

        #[cfg(unix)]
        let askpass_file = tmp_dir.join(format!("t5ssh-askpass-{}", session_id));
        #[cfg(windows)]
        let askpass_file = tmp_dir.join(format!("t5ssh-askpass-{}.cmd", session_id));

        // Script only responds to password prompts; exits 1 for anything else
        // (e.g. host key verification) so SSH falls back to the PTY terminal.
        #[cfg(unix)]
        let script = concat!(
            "#!/bin/sh\n",
            "case \"$1\" in\n",
            "  *assword*) printf '%s' \"$T5SSH_PASSWORD\" ;;\n",
            "  *) exit 1 ;;\n",
            "esac\n",
        );
        #[cfg(windows)]
        let script = "@echo off\r\necho %T5SSH_PASSWORD%\r\n";

        std::fs::write(&askpass_file, script)
            .map_err(|e| format!("Failed to write askpass script: {e}"))?;
        #[cfg(unix)]
        std::fs::set_permissions(&askpass_file, std::fs::Permissions::from_mode(0o700))
            .map_err(|e| format!("Failed to set askpass permissions: {e}"))?;

        command.env("SSH_ASKPASS", askpass_file.to_string_lossy().as_ref());
        command.env("SSH_ASKPASS_REQUIRE", "prefer");
        command.env("T5SSH_PASSWORD", pw);
        // DISPLAY must be set for SSH_ASKPASS to work on Unix
        #[cfg(unix)]
        if std::env::var("DISPLAY").is_err() {
            command.env("DISPLAY", ":0");
        }
        Some(askpass_file)
    } else {
        None
    };

    let (pty_handle, master) = create_pty(command, rows, cols)?;

    let session = Session {
        id: session_id.clone(),
        state: SessionState::Connected,
        pty_handle,
        master,
        label,
    };

    state.insert(session);

    // Spawn a thread to read PTY output and emit events
    let output_session_id = session_id.clone();
    let exit_session_id = session_id.clone();
    let sessions = state.sessions.clone();

    // Take the reader out of the session for the background thread
    let reader = {
        let mut sessions_lock = sessions.lock()
            .map_err(|e| format!("Failed to lock sessions: {}", e))?;
        let session = sessions_lock.get_mut(&output_session_id)
            .ok_or_else(|| format!("Session {} not found after insert", output_session_id))?;
        // We need to move the reader out — replace with a dummy
        let reader = std::mem::replace(
            &mut session.pty_handle.master_reader,
            Box::new(std::io::empty()),
        );
        reader
    };

    let app_clone = app.clone();
    std::thread::spawn(move || {
        read_pty_output(reader, &output_session_id, &app_clone);

        // Clean up askpass script after session ends
        if let Some(ref path) = askpass_path {
            let _ = std::fs::remove_file(path);
        }

        // Session exited
        info!(session_id = %exit_session_id, "SSH session exited");
        let _ = app_clone.emit("session:exit", SessionEvent {
            session_id: exit_session_id.clone(),
        });

        // Update session state
        if let Ok(mut sessions_lock) = sessions.lock() {
            if let Some(session) = sessions_lock.get_mut(&exit_session_id) {
                session.state = SessionState::Closed;
            }
        }
    });

    Ok(session_id)
}

fn read_pty_output(mut reader: Box<dyn Read + Send>, session_id: &str, app: &AppHandle) {
    let mut buf = [0u8; 4096];
    loop {
        match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let data = buf[..n].to_vec();
                let _ = app.emit(
                    "session:output",
                    SessionOutput {
                        session_id: session_id.to_string(),
                        data,
                    },
                );
            }
            Err(e) => {
                error!(session_id = %session_id, error = %e, "PTY read error");
                let _ = app.emit(
                    "session:error",
                    SessionEvent {
                        session_id: session_id.to_string(),
                    },
                );
                break;
            }
        }
    }
}

#[tauri::command]
pub async fn send_input(
    state: State<'_, SessionManager>,
    session_id: String,
    data: Vec<u8>,
) -> Result<(), String> {
    let mut sessions = state.sessions.lock()
        .map_err(|e| format!("Failed to lock sessions: {}", e))?;
    let session = sessions
        .get_mut(&session_id)
        .ok_or_else(|| format!("Session {} not found", session_id))?;

    session
        .pty_handle
        .master_writer
        .write_all(&data)
        .map_err(|e| format!("Failed to write to PTY: {}", e))?;

    session
        .pty_handle
        .master_writer
        .flush()
        .map_err(|e| format!("Failed to flush PTY: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn resize_session(
    state: State<'_, SessionManager>,
    session_id: String,
    rows: u16,
    cols: u16,
) -> Result<(), String> {
    let sessions = state.sessions.lock()
        .map_err(|e| format!("Failed to lock sessions: {}", e))?;
    let session = sessions
        .get(&session_id)
        .ok_or_else(|| format!("Session {} not found", session_id))?;

    resize_pty(session.master.as_ref(), rows, cols)
}

#[tauri::command]
pub async fn close_session(
    state: State<'_, SessionManager>,
    session_id: String,
) -> Result<(), String> {
    info!(session_id = %session_id, "Closing session");

    let mut session = state
        .remove(&session_id)
        .ok_or_else(|| format!("Session {} not found", session_id))?;

    // Kill and reap the child process
    session
        .pty_handle
        .child
        .kill()
        .map_err(|e| format!("Failed to kill child: {}", e))?;
    let _ = session.pty_handle.child.wait();

    Ok(())
}

#[tauri::command]
pub async fn list_sessions(state: State<'_, SessionManager>) -> Result<Vec<SessionInfo>, String> {
    Ok(state.list())
}

#[tauri::command]
pub async fn save_profile(profile: crate::ssh::Profile) -> Result<(), String> {
    crate::profiles::save_profile(profile)
}

#[tauri::command]
pub async fn load_profiles() -> Result<Vec<crate::ssh::Profile>, String> {
    crate::profiles::load_profiles()
}

#[tauri::command]
pub async fn delete_profile(name: String) -> Result<(), String> {
    crate::profiles::delete_profile(&name)?;
    // Clean up stored password (ignore errors if none exists)
    let _ = crate::secrets::delete_password(&name);
    Ok(())
}

#[tauri::command]
pub async fn close_all_sessions(state: State<'_, SessionManager>) -> Result<(), String> {
    info!("Closing all sessions");
    let sessions = state.drain_all();
    for mut session in sessions {
        if let Err(e) = session.pty_handle.child.kill() {
            warn!("Failed to kill child: {}", e);
        }
        let _ = session.pty_handle.child.wait();
    }
    Ok(())
}

#[tauri::command]
pub async fn quit_app(app: AppHandle, state: State<'_, SessionManager>) -> Result<(), String> {
    info!("Quitting app — closing all sessions");
    let sessions = state.drain_all();
    for mut session in sessions {
        if let Err(e) = session.pty_handle.child.kill() {
            warn!("Failed to kill child: {}", e);
        }
        let _ = session.pty_handle.child.wait();
    }
    app.exit(0);
    Ok(())
}

#[tauri::command]
pub async fn store_password(profile_name: String, password: String) -> Result<(), String> {
    crate::secrets::store_password(&profile_name, &password)
}

#[tauri::command]
pub async fn has_password(profile_name: String) -> Result<bool, String> {
    crate::secrets::has_password(&profile_name)
}

#[tauri::command]
pub async fn get_password(profile_name: String) -> Result<Option<String>, String> {
    crate::secrets::get_password(&profile_name)
}

#[tauri::command]
pub async fn delete_password(profile_name: String) -> Result<(), String> {
    crate::secrets::delete_password(&profile_name)
}

#[tauri::command]
pub async fn get_settings(
    state: State<'_, std::sync::Mutex<crate::settings::Settings>>,
) -> Result<crate::settings::Settings, String> {
    let settings = state.lock().map_err(|e| format!("Lock error: {e}"))?;
    Ok(settings.clone())
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, std::sync::Mutex<crate::settings::Settings>>,
    terminal: Option<crate::settings::TerminalSettings>,
) -> Result<(), String> {
    let mut settings = state.lock().map_err(|e| format!("Lock error: {e}"))?;
    settings.terminal = terminal;
    crate::settings::save_settings(&settings);
    Ok(())
}
