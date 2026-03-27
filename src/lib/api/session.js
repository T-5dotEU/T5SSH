import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

/**
 * @param {Record<string, any>} profile
 * @param {number} rows
 * @param {number} cols
 * @param {string|null} [password]
 * @param {string|null} [profileName]
 * @returns {Promise<string>}
 */
export async function createSession(profile, rows, cols, password = null, profileName = null) {
  return await invoke('create_session', { profile, rows, cols, password, profileName });
}

/**
 * @param {string} sessionId
 * @param {string} data
 */
export async function sendInput(sessionId, data) {
  const bytes = Array.from(new TextEncoder().encode(data));
  return await invoke('send_input', { sessionId, data: bytes });
}

/**
 * @param {string} sessionId
 * @param {number} rows
 * @param {number} cols
 */
export async function resizeSession(sessionId, rows, cols) {
  return await invoke('resize_session', { sessionId, rows, cols });
}

/** @param {string} sessionId */
export async function closeSession(sessionId) {
  return await invoke('close_session', { sessionId });
}

export async function listSessions() {
  return await invoke('list_sessions');
}

/** @param {(payload: any) => void} callback */
export function onSessionOutput(callback) {
  return listen('session:output', (event) => {
    callback(event.payload);
  });
}

/** @param {(payload: any) => void} callback */
export function onSessionExit(callback) {
  return listen('session:exit', (event) => {
    callback(event.payload);
  });
}

/** @param {(payload: any) => void} callback */
export function onSessionError(callback) {
  return listen('session:error', (event) => {
    callback(event.payload);
  });
}
