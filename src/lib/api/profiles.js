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
