<script>
  import { onMount } from 'svelte';
  import { Terminal } from '@xterm/xterm';
  import { FitAddon } from '@xterm/addon-fit';
  import '@xterm/xterm/css/xterm.css';
  import { createSession, sendInput, resizeSession, closeSession, onSessionOutput, onSessionExit, onSessionError } from '$lib/api/session.js';

  let { profile = null, sessionId = $bindable(null), onExit = null, onSessionCreated = null } = $props();

  let terminalDiv;
  let terminal;
  let fitAddon;
  let unlistenOutput;
  let unlistenExit;
  let unlistenError;
  let currentSessionId = null;

  function cleanupListeners() {
    if (unlistenOutput) { unlistenOutput(); unlistenOutput = null; }
    if (unlistenExit) { unlistenExit(); unlistenExit = null; }
    if (unlistenError) { unlistenError(); unlistenError = null; }
  }

  async function connect() {
    if (!profile) return;

    const rows = terminal.rows;
    const cols = terminal.cols;

    try {
      currentSessionId = await createSession(profile, rows, cols);
      sessionId = currentSessionId;
      if (onSessionCreated) onSessionCreated(currentSessionId);

      terminal.focus();

      unlistenOutput = await onSessionOutput((payload) => {
        if (payload.session_id === currentSessionId) {
          terminal.write(new Uint8Array(payload.data));
        }
      });

      unlistenExit = await onSessionExit((payload) => {
        if (payload.session_id === currentSessionId) {
          terminal.writeln('\r\n\x1b[33m[Session ended]\x1b[0m');
          if (onExit) onExit(currentSessionId);
        }
      });

      unlistenError = await onSessionError((payload) => {
        if (payload.session_id === currentSessionId) {
          terminal.writeln('\r\n\x1b[31m[Session error]\x1b[0m');
          if (onExit) onExit(currentSessionId);
        }
      });

      terminal.onData((data) => {
        if (currentSessionId) {
          sendInput(currentSessionId, data);
        }
      });

    } catch (e) {
      terminal.writeln(`\r\n\x1b[31mConnection failed: ${e}\x1b[0m`);
      if (onExit) onExit(currentSessionId);
    }
  }

  export function reconnect() {
    cleanupListeners();
    currentSessionId = null;
    sessionId = null;
    terminal.reset();
    terminal.writeln('\x1b[36mReconnecting...\x1b[0m\r\n');
    connect();
  }

  onMount(() => {
    terminal = new Terminal({
      cursorBlink: true,
      fontSize: 14,
      fontFamily: 'Menlo, Monaco, "Courier New", monospace',
      theme: {
        background: '#1e1e1e',
        foreground: '#d4d4d4',
        cursor: '#d4d4d4',
      },
    });

    fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.open(terminalDiv);

    // Ctrl+Shift+C/V: handle copy/paste directly in xterm's key handler
    terminal.attachCustomKeyEventHandler((e) => {
      if (e.type !== 'keydown') return true;
      if (e.ctrlKey && e.shiftKey && e.key === 'C') {
        const sel = terminal.getSelection();
        if (sel) navigator.clipboard.writeText(sel).catch(() => {});
        return false;
      }
      if (e.ctrlKey && e.shiftKey && e.key === 'V') {
        navigator.clipboard.readText().then((text) => {
          if (text && currentSessionId) sendInput(currentSessionId, text);
        }).catch(() => {});
        return false;
      }
      return true;
    });

    // Sync PTY size whenever xterm resizes
    terminal.onResize(({ cols, rows }) => {
      if (currentSessionId) {
        resizeSession(currentSessionId, rows, cols);
      }
    });

    requestAnimationFrame(() => {
      fitAddon.fit();
      connect();
    });

    const handleResize = () => fitAddon.fit();
    window.addEventListener('resize', handleResize);

    const observer = new ResizeObserver(() => {
      if (terminalDiv.offsetParent !== null) {
        fitAddon.fit();
        terminal.focus();
      }
    });
    observer.observe(terminalDiv);

    return () => {
      observer.disconnect();
      window.removeEventListener('resize', handleResize);
      cleanupListeners();
      if (currentSessionId) closeSession(currentSessionId).catch(() => {});
      terminal.dispose();
    };
  });
</script>

<div class="terminal-container" bind:this={terminalDiv}></div>

<style>
  .terminal-container {
    width: 100%;
    height: 100%;
  }
</style>
