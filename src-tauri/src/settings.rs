use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    #[serde(default)]
    pub window: Option<WindowGeometry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

fn settings_path() -> Result<PathBuf, String> {
    let config_dir = dirs::config_dir().ok_or("Could not determine config directory")?;
    let dir = config_dir.join("ultrassh");
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create config dir: {e}"))?;
    }
    Ok(dir.join("settings.json"))
}

pub fn load_settings() -> Settings {
    match settings_path() {
        Ok(path) if path.exists() => {
            match fs::read_to_string(&path) {
                Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
                Err(e) => {
                    error!("Failed to read settings: {e}");
                    Settings::default()
                }
            }
        }
        _ => Settings::default(),
    }
}

pub fn save_settings(settings: &Settings) {
    match settings_path() {
        Ok(path) => {
            match serde_json::to_string_pretty(settings) {
                Ok(json) => {
                    let tmp = path.with_extension("json.tmp");
                    if let Err(e) = fs::write(&tmp, &json) {
                        error!("Failed to write settings tmp: {e}");
                    } else if let Err(e) = fs::rename(&tmp, &path) {
                        error!("Failed to rename settings: {e}");
                    } else {
                        info!("Settings saved");
                    }
                }
                Err(e) => error!("Failed to serialize settings: {e}"),
            }
        }
        Err(e) => error!("Failed to get settings path: {e}"),
    }
}
