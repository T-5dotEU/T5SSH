use keyring::Entry;
use tracing::{info, warn};

const SERVICE: &str = "t5ssh";

fn entry_for(profile_name: &str) -> Result<Entry, String> {
    Entry::new(SERVICE, profile_name).map_err(|e| format!("Keyring error: {e}"))
}

pub fn store_password(profile_name: &str, password: &str) -> Result<(), String> {
    let entry = entry_for(profile_name)?;
    entry
        .set_password(password)
        .map_err(|e| format!("Failed to store password: {e}"))?;
    info!("Stored password for profile '{}'", profile_name);
    Ok(())
}

pub fn get_password(profile_name: &str) -> Result<Option<String>, String> {
    let entry = entry_for(profile_name)?;
    match entry.get_password() {
        Ok(pw) => Ok(Some(pw)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("Failed to get password: {e}")),
    }
}

pub fn delete_password(profile_name: &str) -> Result<(), String> {
    let entry = entry_for(profile_name)?;
    match entry.delete_credential() {
        Ok(()) => {
            info!("Deleted password for profile '{}'", profile_name);
            Ok(())
        }
        Err(keyring::Error::NoEntry) => {
            warn!("No password found for profile '{}' — nothing to delete", profile_name);
            Ok(())
        }
        Err(e) => Err(format!("Failed to delete password: {e}")),
    }
}

pub fn has_password(profile_name: &str) -> Result<bool, String> {
    let entry = entry_for(profile_name)?;
    match entry.get_password() {
        Ok(_) => Ok(true),
        Err(keyring::Error::NoEntry) => Ok(false),
        Err(e) => Err(format!("Failed to check password: {e}")),
    }
}
