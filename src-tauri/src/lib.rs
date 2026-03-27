mod ipc;
mod profiles;
mod pty;
mod secrets;
mod session;
mod settings;
mod ssh;

use session::SessionManager;
use settings::{load_settings, save_settings, Settings, WindowGeometry};
use tauri::{Emitter, Manager, WebviewWindow};
use tracing_subscriber;

fn save_window_geometry(window: &WebviewWindow) {
    if let (Ok(pos), Ok(size)) = (window.outer_position(), window.outer_size()) {
        let geo = WindowGeometry {
            x: pos.x,
            y: pos.y,
            width: size.width,
            height: size.height,
        };
        save_settings(&Settings { window: Some(geo) });
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    let initial_settings = load_settings();

    let mut builder = tauri::Builder::default()
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
            ipc::close_all_sessions,
            ipc::quit_app,
            ipc::store_password,
            ipc::has_password,
            ipc::delete_password,
        ]);

    let saved_geometry = initial_settings.window.clone();

    builder = builder.setup(move |app| {
        let window = match app.get_webview_window("main") {
            Some(w) => w,
            None => {
                tracing::error!("main window not found");
                return Ok(());
            }
        };

        if let Some(geo) = saved_geometry {
            use tauri::{LogicalPosition, LogicalSize};
            let _ = window.set_position(LogicalPosition::new(geo.x as f64, geo.y as f64));
            let _ = window.set_size(LogicalSize::new(geo.width as f64, geo.height as f64));
            tracing::info!("Restored window geometry: {}x{} at ({}, {})", geo.width, geo.height, geo.x, geo.y);
        }

        let win_geo = window.clone();
        let win_close = window.clone();
        window.on_window_event(move |event| {
            use tauri::WindowEvent;
            match event {
                WindowEvent::Resized(_) | WindowEvent::Moved(_) => {
                    save_window_geometry(&win_geo);
                }
                WindowEvent::CloseRequested { api, .. } => {
                    save_window_geometry(&win_geo);
                    api.prevent_close();
                    let _ = win_close.emit("app:close-requested", ());
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
