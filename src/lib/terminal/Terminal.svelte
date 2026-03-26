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

  async function connect() {
    if (!profile) return;

    const rows = terminal.rows;
    const cols = terminal.cols;

    try {
      currentSessionId = await createSession(profile, rows, cols);
      sessionId = currentSessionId;
      if (onSessionCreated) onSessionCreated(currentSessionId);

      // Auto-focus terminal so password prompts can be typed immediately
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
        }
      });

      terminal.onData((data) => {
        if (currentSessionId) {
          sendInput(currentSessionId, data);
        }
      });

    } catch (e) {
      terminal.writeln(`\r\n\x1b[31mConnection failed: ${e}\x1b[0m`);
    }
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

    // Sync PTY size whenever xterm resizes (covers fit, window resize, tab switch)
    terminal.onResize(({ cols, rows }) => {
      if (currentSessionId) {
        resizeSession(currentSessionId, rows, cols);
      }
    });

    // Delay initial fit so the container has its final layout dimensions
    requestAnimationFrame(() => {
      fitAddon.fit();
      connect();
    });

    const handleResize = () => fitAddon.fit();
    window.addEventListener('resize', handleResize);

    // Re-fit and focus when this terminal pane becomes visible (tab switch)
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
      if (unlistenOutput) unlistenOutput();
      if (unlistenExit) unlistenExit();
      if (unlistenError) unlistenError();
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
