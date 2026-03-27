use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub window: Option<WindowGeometry>,
    #[serde(default = "default_terminal_settings")]
    pub terminal: Option<TerminalSettings>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window: None,
            terminal: Some(TerminalSettings::default()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSettings {
    #[serde(default)]
    pub font_family: Option<String>,
    #[serde(default)]
    pub font_size: Option<u32>,
    #[serde(default)]
    pub color_scheme: Option<String>,
    #[serde(default)]
    pub scrollback_lines: Option<u32>,
}

impl Default for TerminalSettings {
    fn default() -> Self {
        Self {
            font_family: Some("Menlo, Monaco, \"Courier New\", monospace".to_string()),
            font_size: Some(14),
            color_scheme: Some("dark".to_string()),
            scrollback_lines: Some(10000),
        }
    }
}

fn default_terminal_settings() -> Option<TerminalSettings> {
    Some(TerminalSettings::default())
}

fn settings_path() -> Result<PathBuf, String> {
    let config_dir = dirs::config_dir().ok_or("Could not determine config directory")?;
    let dir = config_dir.join("t5ssh");
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create config dir: {e}"))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&dir, fs::Permissions::from_mode(0o700));
        }
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
                    } else {
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            let _ = fs::set_permissions(&tmp, fs::Permissions::from_mode(0o600));
                        }
                        if let Err(e) = fs::rename(&tmp, &path) {
                            error!("Failed to rename settings: {e}");
                        } else {
                            info!("Settings saved");
                        }
                    }
                }
                Err(e) => error!("Failed to serialize settings: {e}"),
            }
        }
        Err(e) => error!("Failed to get settings path: {e}"),
    }
}
