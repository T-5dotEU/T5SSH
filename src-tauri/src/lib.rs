mod ipc;
mod profiles;
mod pty;
mod session;
mod settings;
mod ssh;

use session::SessionManager;
use settings::{load_settings, save_settings, Settings, WindowGeometry};
use tauri::Manager;
use tracing_subscriber;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    let initial_settings = load_settings();

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(SessionManager::new())
        .invoke_handler(tauri::generate_handler![
            ipc::create_session,
            ipc::send_input,
            ipc::resize_session,
            ipc::close_session,
            ipc::list_sessions,
            ipc::save_profile,
            ipc::load_profiles,
            ipc::delete_profile,
        ]);

    let saved_geometry = initial_settings.window.clone();

    builder = builder.setup(move |app| {
        let window = app.get_webview_window("main").expect("main window not found");

        if let Some(geo) = saved_geometry {
            use tauri::{LogicalPosition, LogicalSize};
            let _ = window.set_position(LogicalPosition::new(geo.x as f64, geo.y as f64));
            let _ = window.set_size(LogicalSize::new(geo.width as f64, geo.height as f64));
            tracing::info!("Restored window geometry: {}x{} at ({}, {})", geo.width, geo.height, geo.x, geo.y);
        }

        let win = window.clone();
        window.on_window_event(move |event| {
            use tauri::WindowEvent;
            match event {
                WindowEvent::Resized(_) | WindowEvent::Moved(_) | WindowEvent::CloseRequested { .. } => {
                    if let (Ok(pos), Ok(size)) = (win.outer_position(), win.outer_size()) {
                        let geo = WindowGeometry {
                            x: pos.x,
                            y: pos.y,
                            width: size.width,
                            height: size.height,
                        };
                        save_settings(&Settings { window: Some(geo) });
                    }
                }
                _ => {}
            }
        });

        Ok(())
    });

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
