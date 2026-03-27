import { invoke } from '@tauri-apps/api/core';

export async function getSettings() {
  return await invoke('get_settings');
}

export async function updateSettings(terminal) {
  return await invoke('update_settings', { terminal });
}
