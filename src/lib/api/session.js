import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export async function createSession(profile, rows, cols) {
  return await invoke('create_session', { profile, rows, cols });
}

export async function sendInput(sessionId, data) {
  const bytes = Array.from(new TextEncoder().encode(data));
  return await invoke('send_input', { sessionId, data: bytes });
}

export async function resizeSession(sessionId, rows, cols) {
  return await invoke('resize_session', { sessionId, rows, cols });
}

export async function closeSession(sessionId) {
  return await invoke('close_session', { sessionId });
}

export async function listSessions() {
  return await invoke('list_sessions');
}

export function onSessionOutput(callback) {
  return listen('session:output', (event) => {
    callback(event.payload);
  });
}

export function onSessionExit(callback) {
  return listen('session:exit', (event) => {
    callback(event.payload);
  });
}

export function onSessionError(callback) {
  return listen('session:error', (event) => {
    callback(event.payload);
  });
}
