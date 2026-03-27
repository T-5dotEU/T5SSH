<script>
  import { onMount } from 'svelte';
  import { saveProfile, loadProfiles, deleteProfile } from '$lib/api/profiles.js';
  import { storePassword, hasPassword, getPassword, deletePassword } from '$lib/api/profiles.js';

  import { open } from '@tauri-apps/plugin-dialog';

  let { onConnect = null, onCancel = null, onOpenSettings = null, initialProfile = null, canCancel = true } = $props();

  /** @type {any[]} */
  let savedProfiles = $state([]);
  let name = $state('');
  let host = $state('');
  let port = $state(22);
  let user = $state('');
  let identityFile = $state('');
  let jumpHost = $state('');
  let agentForwarding = $state(false);
  /** @type {any[]} */
  let portForwards = $state([]);
  let password = $state('');
  let showPassword = $state(false);
  let savePasswordChecked = $state(false);
  /** @type {Record<string, boolean>} */
  let profilePasswordFlags = $state({});

  /** @type {any} */
  let formSnapshot = $state(null);

  let saving = $state(false);
  let error = $state('');
  let successMsg = $state('');
  /** @type {string|null} */
  let editingProfile = $state(null);
  /** @type {{x: number, y: number, profile: any}|null} */
  let contextMenu = $state(null);
  /** @type {HTMLButtonElement} */
  let connectBtn;
  /** @type {HTMLDivElement} */
  let formGrid;

  onMount(async () => {
    try {
      savedProfiles = await loadProfiles();
      // Check which profiles have stored passwords
      for (const p of savedProfiles) {
        try {
          profilePasswordFlags[p.name] = await hasPassword(p.name);
        } catch (_) { /* ignore */ }
      }
      profilePasswordFlags = profilePasswordFlags;
    } catch (e) {
      console.error('Failed to load profiles:', e);
    }
    if (initialProfile) {
      loadIntoForm(initialProfile);
      requestAnimationFrame(() => formGrid?.scrollIntoView({ behavior: 'smooth', block: 'start' }));
    }
  });

  /** @param {any} profile */
  async function loadIntoForm(profile) {
    editingProfile = profile.name;
    name = profile.name;
    host = profile.ssh.host;
    port = profile.ssh.port;
    user = profile.ssh.user ?? '';
    identityFile = profile.ssh.identity_file ?? '';
    jumpHost = profile.ssh.jump_host ?? '';
    agentForwarding = profile.ssh.agent_forwarding ?? false;
    portForwards = profile.ssh.port_forwards?.map((/** @type {any} */ f) => ({ ...f })) ?? [];
    showPassword = false;
    savePasswordChecked = !!profilePasswordFlags[profile.name];
    // Load stored password from keyring
    if (profilePasswordFlags[profile.name]) {
      try {
        password = (await getPassword(profile.name)) ?? '';
      } catch (_) {
        password = '';
      }
    } else {
      password = '';
    }
    snapshotForm();
    requestAnimationFrame(() => connectBtn?.scrollIntoView({ behavior: 'smooth', block: 'nearest' }));
  }

  function snapshotForm() {
    formSnapshot = { name, host, port, user, identityFile, jumpHost, agentForwarding, portForwards: JSON.stringify(portForwards), password, savePasswordChecked };
  }

  function isFormDirty() {
    if (!formSnapshot) return name || host || user || identityFile || jumpHost || password;
    return name !== formSnapshot.name || host !== formSnapshot.host || port !== formSnapshot.port ||
      user !== formSnapshot.user || identityFile !== formSnapshot.identityFile ||
      jumpHost !== formSnapshot.jumpHost || agentForwarding !== formSnapshot.agentForwarding ||
      JSON.stringify(portForwards) !== formSnapshot.portForwards ||
      password !== formSnapshot.password || savePasswordChecked !== formSnapshot.savePasswordChecked;
  }

  function confirmDiscardChanges() {
    return !isFormDirty() || confirm('Das aktuelle Profil hat ungespeicherte Änderungen. Änderungen verwerfen?');
  }

  function clearForm(skipConfirm = false) {
    if (!skipConfirm && !confirmDiscardChanges()) return;
    editingProfile = null;
    name = '';
    host = '';
    port = 22;
    user = '';
    identityFile = '';
    jumpHost = '';
    agentForwarding = false;
    portForwards = [];
    password = '';
    showPassword = false;
    savePasswordChecked = false;
    error = '';
    formSnapshot = null;
  }

  /** @param {MouseEvent} e @param {any} profile */
  function handleContextMenu(e, profile) {
    e.preventDefault();
    e.stopPropagation();
    let x = e.clientX;
    let y = e.clientY;
    if (x > window.innerWidth - 150) x = window.innerWidth - 150;
    if (y > window.innerHeight - 80) y = window.innerHeight - 80;
    contextMenu = { x, y, profile };
  }

  function closeContextMenu() {
    contextMenu = null;
  }

  function contextEdit() {
    if (contextMenu) {
      if (!confirmDiscardChanges()) { closeContextMenu(); return; }
      loadIntoForm(contextMenu.profile);
    }
    closeContextMenu();
  }

  async function contextDelete() {
    if (!contextMenu) return;
    const profileName = contextMenu.profile.name;
    closeContextMenu();
    if (!confirm(`Delete profile "${profileName}"?`)) return;
    try {
      await deleteProfile(profileName);
      await deletePassword(profileName).catch(() => {});
      savedProfiles = savedProfiles.filter((p) => p.name !== profileName);
      delete profilePasswordFlags[profileName];
      profilePasswordFlags = profilePasswordFlags;
      if (editingProfile === profileName) clearForm(true);
    } catch (err) {
      console.error('Failed to delete profile:', err);
    }
  }

  function addPortForward() {
    portForwards.push({ local_port: 0, remote_host: 'localhost', remote_port: 0 });
  }

  /** @param {number} idx */
  function removePortForward(idx) {
    portForwards.splice(idx, 1);
  }

  function buildSshProfile() {
    return {
      host,
      port: Number(port),
      user: user || null,
      identity_file: identityFile || null,
      jump_host: jumpHost || null,
      port_forwards: portForwards
        .filter((f) => f.local_port > 0 && f.remote_port > 0)
        .map((f) => ({
          local_port: Number(f.local_port),
          remote_host: f.remote_host,
          remote_port: Number(f.remote_port),
        })),
      agent_forwarding: agentForwarding,
    };
  }

  function handleConnect() {
    if (!host) {
      error = 'Host is required';
      return;
    }
    error = '';
    if (onConnect) {
      const pw = password || null;
      const pname = name || null;
      onConnect(buildSshProfile(), pw, pname);
    }
  }

  async function handleSave() {
    if (!host) {
      error = 'Host is required';
      return;
    }
    if (!name) {
      error = 'Profile name is required to save';
      return;
    }
    error = '';
    const nameChanged = editingProfile && editingProfile !== name;
    let doRename = false;

    if (nameChanged) {
      const choice = confirm(
        `Profile name changed from "${editingProfile}" to "${name}".\n\nOK = Rename existing profile\nCancel = Save as new profile`
      );
      doRename = choice;
    }

    saving = true;
    try {
      const profile = { name, ssh: buildSshProfile(), rows: 24, cols: 80 };

      // Rename: delete old profile on disk and clean up old password
      if (doRename && editingProfile) {
        await deleteProfile(editingProfile).catch(() => {});
        await deletePassword(editingProfile).catch(() => {});
        delete profilePasswordFlags[editingProfile];
      }

      await saveProfile(profile);

      // Handle password storage
      if (savePasswordChecked && password) {
        await storePassword(name, password);
        profilePasswordFlags[name] = true;
      } else if (!savePasswordChecked) {
        await deletePassword(name).catch(() => {});
        profilePasswordFlags[name] = false;
      }
      profilePasswordFlags = profilePasswordFlags;

      if (doRename) {
        // Replace old profile entry with the renamed one
        const oldIdx = savedProfiles.findIndex((p) => p.name === editingProfile);
        if (oldIdx >= 0) {
          savedProfiles[oldIdx] = profile;
        } else {
          savedProfiles.push(profile);
        }
      } else {
        const idx = savedProfiles.findIndex((p) => p.name === name);
        if (idx >= 0) {
          savedProfiles[idx] = profile;
        } else {
          savedProfiles.push(profile);
        }
      }
      savedProfiles = savedProfiles;
      const oldName = editingProfile;
      editingProfile = name;
      snapshotForm();
      successMsg = `✓ Profile "${name}" saved` + (doRename ? ` (renamed from "${oldName}")` : '') + (savePasswordChecked ? ' (with password)' : '');
      setTimeout(() => successMsg = '', 2500);

      // Scroll new profile into view
      requestAnimationFrame(() => {
        const el = document.querySelector('.profile-item.editing');
        el?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
      });
    } catch (e) {
      error = `Save failed: ${e}`;
    } finally {
      saving = false;
    }
  }

  async function handleSaveAndConnect() {
    await handleSave();
    if (!error) handleConnect();
  }

  /** @param {any} profile */
  function handleProfileDblClick(profile) {
    if (!confirmDiscardChanges()) return;
    loadIntoForm(profile);
    requestAnimationFrame(() => handleConnect());
  }

  /** @param {KeyboardEvent} e */
  function handleKeydown(e) {
    if (e.key === 'Escape') {
      if (contextMenu) { closeContextMenu(); return; }
      if (canCancel && onCancel) onCancel();
    }
    if (e.key === 'Enter' && !e.shiftKey) handleConnect();
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div class="overlay" onkeydown={handleKeydown} role="dialog" aria-modal="true">
  <div class="dialog">
    <h2>{editingProfile ? `Edit: ${editingProfile}` : 'New Connection'}</h2>

    {#if savedProfiles.length > 0}
      <div class="saved-profiles">
        <h3>Saved Profiles <span class="hint">(click = load, double-click = connect, right-click = options)</span></h3>
        <div class="profile-list">
          {#each savedProfiles as profile}
            <div
              class="profile-item"
              class:editing={editingProfile === profile.name}
              onclick={() => { if (confirmDiscardChanges()) loadIntoForm(profile); }}
              ondblclick={() => handleProfileDblClick(profile)}
              oncontextmenu={(e) => handleContextMenu(e, profile)}
              role="button"
              tabindex="0"
              onkeydown={(e) => e.key === 'Enter' && confirmDiscardChanges() && loadIntoForm(profile)}
            >
              <div class="profile-info">
                <span class="profile-name">{profile.name}</span>
                <span class="profile-host">
                  {profile.ssh.user ? `${profile.ssh.user}@` : ''}{profile.ssh.host}:{profile.ssh.port}
                  {#if profilePasswordFlags[profile.name]}
                    <span class="pw-indicator" title="Password stored">🔑</span>
                  {/if}
                </span>
              </div>
            </div>
          {/each}
        </div>
      </div>
      <hr class="divider">
    {/if}

    {#if successMsg}
      <div class="success-msg">{successMsg}</div>
    {/if}

    {#if error}
      <div class="error">{error}</div>
    {/if}

    <div class="form-grid" bind:this={formGrid}>
      <label for="cd-name">Profile Name</label>
      <input id="cd-name" type="text" bind:value={name} placeholder="My Server (optional)">

      <label for="cd-host">Host</label>
      <input id="cd-host" type="text" bind:value={host} placeholder="example.com" autofocus>

      <label for="cd-port">Port</label>
      <input id="cd-port" type="number" bind:value={port} min="1" max="65535">

      <label for="cd-user">Username</label>
      <input id="cd-user" type="text" bind:value={user} placeholder="(current user)">

      <label for="cd-identity">Identity File</label>
      <div class="identity-row">
        <input id="cd-identity" type="text" bind:value={identityFile} placeholder="~/.ssh/id_rsa">
        <button type="button" class="btn-icon browse-btn" title="Browse..." onclick={async () => { const f = await open({ title: 'Select Identity File', directory: false, multiple: false }); if (f) identityFile = /** @type {string} */ (f); }}>📂</button>
      </div>

      <label for="cd-password">Password</label>
      <div class="password-row">
        <input id="cd-password" type="password" bind:value={password} placeholder={savePasswordChecked && !password ? '(stored in keyring)' : '(optional)'} autocomplete="off">
        <button type="button" class="btn-icon toggle-pw" class:showing={showPassword} onclick={(e) => { const inp = /** @type {HTMLInputElement|null} */ (document.getElementById('cd-password')); if (inp) { inp.type = inp.type === 'password' ? 'text' : 'password'; showPassword = inp.type === 'text'; } }} title={showPassword ? 'Hide' : 'Show'}>
          {#if showPassword}
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94"/><path d="M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19"/><line x1="1" y1="1" x2="23" y2="23"/></svg>
          {:else}
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
          {/if}
        </button>
      </div>

      <label for="cd-save-pw" class="save-pw-label">Save Password</label>
      <div class="save-pw-row">
        <input id="cd-save-pw" type="checkbox" bind:checked={savePasswordChecked}>
        <span class="save-pw-hint">Store in system keyring</span>
      </div>

      <label for="cd-jump">Jump Host</label>
      <input id="cd-jump" type="text" bind:value={jumpHost} placeholder="bastion.example.com">

      <label for="cd-agent">Agent Forwarding</label>
      <input id="cd-agent" type="checkbox" bind:checked={agentForwarding}>
    </div>

    <div class="section">
      <h3>Port Forwards</h3>
      {#each portForwards as fwd, i}
        <div class="port-forward-row">
          <input type="number" bind:value={fwd.local_port} placeholder="Local" min="1" max="65535">
          <span>→</span>
          <input type="text" bind:value={fwd.remote_host} placeholder="Remote host">
          <span>:</span>
          <input type="number" bind:value={fwd.remote_port} placeholder="Port" min="1" max="65535">
          <button class="btn-icon" onclick={() => removePortForward(i)}>×</button>
        </div>
      {/each}
      <button class="btn-small" onclick={addPortForward}>+ Add Forward</button>
    </div>

    <div class="actions">
      {#if canCancel}
        <button class="btn cancel" onclick={onCancel}>Cancel</button>
      {/if}
      {#if editingProfile}
        <button class="btn secondary" onclick={() => clearForm()}>New</button>
        <button class="btn success" onclick={handleSave} disabled={saving}>
          {saving ? 'Saving...' : '💾 Save'}
        </button>
      {/if}
      <button class="btn success" onclick={handleSaveAndConnect} disabled={saving}>
        {saving ? 'Saving...' : '💾 Save & Connect'}
      </button>
      <button class="btn primary" onclick={handleConnect} bind:this={connectBtn}>Connect</button>
      {#if onOpenSettings && !canCancel}
        <button class="btn settings" onclick={onOpenSettings} title="Settings">⚙</button>
      {/if}
    </div>
  </div>
</div>

{#if contextMenu}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="context-overlay" onclick={closeContextMenu} oncontextmenu={(e) => { e.preventDefault(); closeContextMenu(); }}>
    <div class="context-menu" style="left: {contextMenu.x}px; top: {contextMenu.y}px;">
      <button class="context-item" onclick={contextEdit}>✏️ Edit</button>
      <button class="context-item danger" onclick={contextDelete}>🗑️ Delete</button>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    overflow: hidden;
  }

  .dialog {
    background: #2d2d2d;
    border: 1px solid #555;
    border-radius: 8px;
    padding: 24px;
    width: 480px;
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
    margin: 0 0 8px;
    font-size: 14px;
    color: #ccc;
  }

  .hint {
    font-size: 11px;
    color: #666;
    font-weight: normal;
  }

  .saved-profiles {
    margin-bottom: 8px;
  }

  .profile-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-height: 180px;
    overflow-y: auto;
  }

  .profile-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    background: #1e1e1e;
    border: 1px solid #3c3c3c;
    border-radius: 4px;
    cursor: pointer;
    color: #d4d4d4;
  }

  .profile-item:hover {
    border-color: #007acc;
    background: #252525;
  }

  .profile-item.editing {
    border-color: #2d6a30;
    background: #1e2e1e;
  }

  .profile-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .profile-name {
    font-size: 13px;
    color: #fff;
  }

  .profile-host {
    font-size: 11px;
    color: #888;
  }

  .divider {
    border: none;
    border-top: 1px solid #3c3c3c;
    margin: 12px 0;
  }

  .success-msg {
    background: #1a3a1a;
    border: 1px solid #2d6a30;
    padding: 8px 12px;
    border-radius: 4px;
    margin-bottom: 12px;
    font-size: 13px;
    color: #a5d6a7;
  }

  .error {
    background: #5c2020;
    border: 1px solid #a33;
    padding: 8px 12px;
    border-radius: 4px;
    margin-bottom: 12px;
    font-size: 13px;
  }

  .form-grid {
    display: grid;
    grid-template-columns: 120px 1fr;
    gap: 8px 12px;
    align-items: center;
    margin-bottom: 16px;
  }

  label {
    font-size: 13px;
    color: #aaa;
  }

  input[type="text"],
  input[type="number"] {
    background: #1e1e1e;
    border: 1px solid #555;
    border-radius: 4px;
    padding: 6px 8px;
    color: #d4d4d4;
    font-size: 13px;
  }

  input[type="text"]:focus,
  input[type="number"]:focus {
    outline: none;
    border-color: #007acc;
  }

  input[type="checkbox"] {
    justify-self: start;
  }

  .password-row {
    display: flex;
    gap: 4px;
  }

  .identity-row {
    display: flex;
    gap: 4px;
  }

  .identity-row input {
    flex: 1;
    background: #1e1e1e;
    border: 1px solid #555;
    border-radius: 4px;
    padding: 6px 8px;
    color: #d4d4d4;
    font-size: 13px;
  }

  .identity-row input:focus {
    outline: none;
    border-color: #007acc;
  }

  .browse-btn {
    color: #aaa;
    font-size: 16px;
    flex-shrink: 0;
    width: 28px;
  }

  .browse-btn:hover {
    color: #fff;
  }

  .password-row input {
    flex: 1;
    background: #1e1e1e;
    border: 1px solid #555;
    border-radius: 4px;
    padding: 6px 8px;
    color: #d4d4d4;
    font-size: 13px;
  }

  .password-row input:focus {
    outline: none;
    border-color: #007acc;
  }

  .toggle-pw {
    color: #aaa;
    font-size: 14px;
    flex-shrink: 0;
    width: 28px;
  }

  .toggle-pw.showing {
    color: #e74c3c;
  }

  .save-pw-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .save-pw-hint {
    font-size: 11px;
    color: #888;
  }

  .pw-indicator {
    font-size: 10px;
    margin-left: 4px;
  }

  .section {
    margin-bottom: 16px;
  }

  .port-forward-row {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 6px;
  }

  .port-forward-row input[type="number"] {
    width: 70px;
  }

  .port-forward-row input[type="text"] {
    flex: 1;
  }

  .port-forward-row span {
    color: #888;
  }

  .btn-icon {
    background: none;
    border: none;
    color: #e44;
    font-size: 18px;
    cursor: pointer;
    padding: 0 4px;
  }

  .btn-small {
    background: none;
    border: 1px dashed #2d6a30;
    color: #4ec9b0;
    padding: 4px 10px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
  }

  .btn-small:hover {
    border-color: #38833c;
    background: rgba(45, 106, 48, 0.15);
    color: #6fd8a8;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 8px;
  }

  .btn {
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    font-size: 13px;
    cursor: pointer;
  }

  .btn.primary {
    background: #007acc;
    color: #fff;
  }

  .btn.primary:hover {
    background: #0098ff;
  }

  .btn.settings {
    background: #3c3c3c;
    color: #ccc;
    margin-left: auto;
    font-size: 16px;
    padding: 8px 10px;
  }

  .btn.settings:hover {
    background: #4a4a4a;
    color: #fff;
  }

  .btn.secondary {
    background: #3c3c3c;
    color: #ccc;
  }

  .btn.secondary:hover {
    background: #4a4a4a;
  }

  .btn.success {
    background: #2d6a30;
    color: #fff;
  }

  .btn.success:hover {
    background: #38833c;
  }

  .btn.cancel {
    background: transparent;
    border: 1px solid #c33;
    color: #e55;
  }

  .btn.cancel:hover {
    background: rgba(204, 51, 51, 0.15);
    border-color: #e55;
    color: #f77;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .context-overlay {
    position: fixed;
    inset: 0;
    z-index: 200;
  }

  .context-menu {
    position: fixed;
    background: #2d2d2d;
    border: 1px solid #555;
    border-radius: 6px;
    padding: 4px 0;
    min-width: 140px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    z-index: 201;
  }

  .context-item {
    display: block;
    width: 100%;
    padding: 8px 16px;
    background: none;
    border: none;
    color: #d4d4d4;
    font-size: 13px;
    text-align: left;
    cursor: pointer;
  }

  .context-item:hover {
    background: #094771;
    color: #fff;
  }

  .context-item.danger {
    color: #e55;
  }

  .context-item.danger:hover {
    background: #5c2020;
    color: #f88;
  }
</style>
