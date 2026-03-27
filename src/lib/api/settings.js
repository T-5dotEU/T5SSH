import { invoke } from '@tauri-apps/api/core';

/** @returns {Promise<{terminal?: {font_family?: string, font_size?: number, color_scheme?: string, scrollback_lines?: number}, window?: any}>} */
export async function getSettings() {
  return await invoke('get_settings');
}

/** @param {{font_family?: string|null, font_size?: number|null, color_scheme?: string|null, scrollback_lines?: number|null}} terminal */
export async function updateSettings(terminal) {
  return await invoke('update_settings', { terminal });
}
