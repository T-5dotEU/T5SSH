import { invoke } from '@tauri-apps/api/core';

export async function saveProfile(profile) {
  return await invoke('save_profile', { profile });
}

export async function loadProfiles() {
  return await invoke('load_profiles');
}

export async function deleteProfile(name) {
  return await invoke('delete_profile', { name });
}

export async function storePassword(profileName, password) {
  return await invoke('store_password', { profileName, password });
}

export async function hasPassword(profileName) {
  return await invoke('has_password', { profileName });
}

export async function deletePassword(profileName) {
  return await invoke('delete_password', { profileName });
}
