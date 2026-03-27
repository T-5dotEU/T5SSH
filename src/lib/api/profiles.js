import { invoke } from '@tauri-apps/api/core';

/** @param {Record<string, any>} profile */
export async function saveProfile(profile) {
  return await invoke('save_profile', { profile });
}

/** @returns {Promise<any[]>} */
export async function loadProfiles() {
  return await invoke('load_profiles');
}

/** @param {string} name */
export async function deleteProfile(name) {
  return await invoke('delete_profile', { name });
}

/**
 * @param {string} profileName
 * @param {string} password
 */
export async function storePassword(profileName, password) {
  return await invoke('store_password', { profileName, password });
}

/** @param {string} profileName @returns {Promise<boolean>} */
export async function hasPassword(profileName) {
  return await invoke('has_password', { profileName });
}

/** @param {string} profileName @returns {Promise<string|null>} */
export async function getPassword(profileName) {
  return await invoke('get_password', { profileName });
}

/** @param {string} profileName */
export async function deletePassword(profileName) {
  return await invoke('delete_password', { profileName });
}
