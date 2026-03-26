<script>
  import { onMount } from 'svelte';
  import { Terminal } from '@xterm/xterm';
  import { FitAddon } from '@xterm/addon-fit';
  import '@xterm/xterm/css/xterm.css';
  import { createSession, sendInput, resizeSession, closeSession, onSessionOutput, onSessionExit, onSessionError } from '$lib/api/session.js';

  let { profile = null, sessionId = $bindable(null), onExit = null } = $props();

  let terminalDiv;
  let terminal;
  let fitAddon;
  let unlistenOutput;
  let unlistenExit;
  let unlistenError;

  async function connect() {
    if (!profile) return;

    const rows = terminal.rows;
    const cols = terminal.cols;

    try {
      sessionId = await createSession(profile, rows, cols);

      unlistenOutput = await onSessionOutput((payload) => {
        if (payload.session_id === sessionId) {
          terminal.write(new Uint8Array(payload.data));
        }
      });

      unlistenExit = await onSessionExit((payload) => {
        if (payload.session_id === sessionId) {
          terminal.writeln('\r\n\x1b[33m[Session ended]\x1b[0m');
          if (onExit) onExit(sessionId);
        }
      });

      unlistenError = await onSessionError((payload) => {
        if (payload.session_id === sessionId) {
          terminal.writeln('\r\n\x1b[31m[Session error]\x1b[0m');
        }
      });

      terminal.onData((data) => {
        if (sessionId) {
          sendInput(sessionId, data);
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
    fitAddon.fit();

    const handleResize = () => {
      fitAddon.fit();
      if (sessionId) {
        resizeSession(sessionId, terminal.rows, terminal.cols);
      }
    };
    window.addEventListener('resize', handleResize);

    connect();

    return () => {
      window.removeEventListener('resize', handleResize);
      if (unlistenOutput) unlistenOutput();
      if (unlistenExit) unlistenExit();
      if (unlistenError) unlistenError();
      if (sessionId) closeSession(sessionId).catch(() => {});
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
