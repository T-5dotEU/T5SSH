use crate::ssh::Profile;
use std::fs;
use std::path::PathBuf;
use tracing::{error, info};

fn profiles_path() -> Result<PathBuf, String> {
    let config_dir = dirs::config_dir().ok_or("Could not determine config directory")?;
    let dir = config_dir.join("ultrassh");
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create config dir: {e}"))?;
    }
    Ok(dir.join("profiles.json"))
}

pub fn load_profiles() -> Result<Vec<Profile>, String> {
    let path = profiles_path()?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let data = fs::read_to_string(&path).map_err(|e| format!("Failed to read profiles: {e}"))?;
    let profiles: Vec<Profile> =
        serde_json::from_str(&data).map_err(|e| format!("Failed to parse profiles: {e}"))?;
    Ok(profiles)
}

pub fn save_profiles(profiles: &[Profile]) -> Result<(), String> {
    let path = profiles_path()?;
    let data = serde_json::to_string_pretty(profiles)
        .map_err(|e| format!("Failed to serialize profiles: {e}"))?;
    fs::write(&path, data).map_err(|e| format!("Failed to write profiles: {e}"))?;
    info!("Saved {} profiles to {:?}", profiles.len(), path);
    Ok(())
}

pub fn save_profile(profile: Profile) -> Result<(), String> {
    let mut profiles = load_profiles()?;
    if let Some(existing) = profiles.iter_mut().find(|p| p.name == profile.name) {
        *existing = profile;
    } else {
        profiles.push(profile);
    }
    save_profiles(&profiles)
}

pub fn delete_profile(name: &str) -> Result<(), String> {
    let mut profiles = load_profiles()?;
    let before = profiles.len();
    profiles.retain(|p| p.name != name);
    if profiles.len() == before {
        return Err(format!("Profile '{}' not found", name));
    }
    save_profiles(&profiles)
}
