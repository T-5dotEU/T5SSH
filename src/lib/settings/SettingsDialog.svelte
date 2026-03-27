<script>
  import { getSettings, updateSettings } from '$lib/api/settings.js';
  import { colorSchemes } from '$lib/settings/colorSchemes.js';
  import { onMount } from 'svelte';

  let { onClose = null, onApply = null } = $props();

  const fonts = [
    { label: 'Menlo', value: 'Menlo, Monaco, "Courier New", monospace' },
    { label: 'Cascadia Code', value: '"Cascadia Code", Menlo, monospace' },
    { label: 'Fira Code', value: '"Fira Code", monospace' },
    { label: 'JetBrains Mono', value: '"JetBrains Mono", monospace' },
    { label: 'Source Code Pro', value: '"Source Code Pro", monospace' },
    { label: 'Ubuntu Mono', value: '"Ubuntu Mono", monospace' },
    { label: 'Consolas', value: 'Consolas, "Courier New", monospace' },
    { label: 'Courier New', value: '"Courier New", monospace' },
  ];

  let fontFamily = $state(fonts[0].value);
  let fontSize = $state(14);
  let colorScheme = $state('dark');
  let scrollbackLines = $state(10000);
  let saving = $state(false);

  onMount(async () => {
    try {
      const settings = await getSettings();
      if (settings.terminal) {
        if (settings.terminal.font_family) fontFamily = settings.terminal.font_family;
        if (settings.terminal.font_size) fontSize = settings.terminal.font_size;
        if (settings.terminal.color_scheme) colorScheme = settings.terminal.color_scheme;
        if (settings.terminal.scrollback_lines) scrollbackLines = settings.terminal.scrollback_lines;
      }
    } catch (e) {
      console.error('Failed to load settings:', e);
    }
  });

  function currentTheme() {
    return colorSchemes[/** @type {keyof colorSchemes} */ (colorScheme)] || colorSchemes.dark;
  }

  async function handleSave() {
    saving = true;
    try {
      const terminal = {
        font_family: fontFamily || null,
        font_size: fontSize ? Number(fontSize) : null,
        color_scheme: colorScheme || null,
        scrollback_lines: scrollbackLines ? Number(scrollbackLines) : null,
      };
      await updateSettings(terminal);
      if (onApply) onApply(terminal);
      if (onClose) onClose();
    } catch (e) {
      console.error('Failed to save settings:', e);
    } finally {
      saving = false;
    }
  }

  /** @param {KeyboardEvent} e */
  function handleKeydown(e) {
    if (e.key === 'Escape' && onClose) onClose();
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div class="overlay" onkeydown={handleKeydown} role="dialog" aria-modal="true">
  <div class="dialog">
    <h2>Settings</h2>

    <div class="section">
      <h3>Terminal Appearance</h3>

      <div class="form-grid">
        <label for="s-font">Font</label>
        <select id="s-font" bind:value={fontFamily}>
          {#each fonts as f}
            <option value={f.value}>{f.label}</option>
          {/each}
        </select>

        <label for="s-fontsize">Font Size</label>
        <input id="s-fontsize" type="number" bind:value={fontSize} min="8" max="36">

        <label for="s-scheme">Color Scheme</label>
        <select id="s-scheme" bind:value={colorScheme}>
          <option value="dark">Dark (Konsole)</option>
          <option value="classic">Classic (VS Code)</option>
        </select>

        <label for="s-scrollback">Scrollback Lines</label>
        <input id="s-scrollback" type="number" bind:value={scrollbackLines} min="1000" max="100000" step="1000">
      </div>

      <div class="preview" style="background: {currentTheme().background}; font-family: {fontFamily}; font-size: {fontSize}px;">
        <span style="color: {currentTheme().foreground}">user@host:~$ </span><span style="color: {currentTheme().green}">ls</span> <span style="color: {currentTheme().cyan}">-la</span>
        <br>
        <span style="color: {currentTheme().blue}">drwxr-xr-x</span> <span style="color: {currentTheme().foreground}">2 user user 4096</span> <span style="color: {currentTheme().yellow}">Documents/</span>
      </div>
    </div>

    <div class="actions">
      <button class="btn cancel" onclick={onClose}>Cancel</button>
      <button class="btn primary" onclick={handleSave} disabled={saving}>
        {saving ? 'Saving...' : 'Save'}
      </button>
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 150;
  }

  .dialog {
    background: #2d2d2d;
    border: 1px solid #555;
    border-radius: 8px;
    padding: 24px;
    width: 440px;
    max-height: 80vh;
    overflow-y: auto;
    color: #d4d4d4;
  }

  h2 {
    margin: 0 0 16px;
    font-size: 18px;
    color: #fff;
  }

  h3 {
    margin: 0 0 12px;
    font-size: 14px;
    color: #ccc;
  }

  .section {
    margin-bottom: 16px;
  }

  .form-grid {
    display: grid;
    grid-template-columns: 120px 1fr;
    gap: 8px 12px;
    align-items: center;
  }

  label {
    font-size: 13px;
    color: #aaa;
  }

  select {
    background: #1e1e1e;
    border: 1px solid #3c3c3c;
    border-radius: 4px;
    padding: 6px 8px;
    color: #d4d4d4;
    font-size: 13px;
    appearance: auto;
  }

  select option {
    background: #1e1e1e;
    color: #d4d4d4;
  }

  select:focus {
    outline: none;
    border-color: #007acc;
  }

  input[type="number"] {
    background: #1e1e1e;
    border: 1px solid #3c3c3c;
    border-radius: 4px;
    padding: 6px 8px;
    color: #d4d4d4;
    font-size: 13px;
  }

  input:focus,
  select:focus {
    outline: none;
    border-color: #007acc;
  }

  .preview {
    margin-top: 12px;
    padding: 12px;
    border: 1px solid #3c3c3c;
    border-radius: 4px;
    min-height: 40px;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 16px;
  }

  .btn {
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 13px;
  }

  .btn.cancel {
    background: #3c3c3c;
    color: #d4d4d4;
  }

  .btn.cancel:hover {
    background: #4a4a4a;
  }

  .btn.primary {
    background: #007acc;
    color: #fff;
  }

  .btn.primary:hover {
    background: #0098ff;
  }
</style>
