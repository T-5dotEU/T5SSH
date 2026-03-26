mod ipc;
mod pty;
mod session;
mod ssh;

use session::SessionManager;
use tracing_subscriber;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(SessionManager::new())
        .invoke_handler(tauri::generate_handler![
            ipc::create_session,
            ipc::send_input,
            ipc::resize_session,
            ipc::close_session,
            ipc::list_sessions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
