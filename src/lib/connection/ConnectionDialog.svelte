<script>
  import { onMount } from 'svelte';
  import { saveProfile, loadProfiles, deleteProfile } from '$lib/api/profiles.js';

  let { onConnect = null, onCancel = null, initialProfile = null, canCancel = true } = $props();

  let savedProfiles = $state([]);
  let name = $state('');
  let host = $state('');
  let port = $state(22);
  let user = $state('');
  let identityFile = $state('');
  let jumpHost = $state('');
  let agentForwarding = $state(false);
  let portForwards = $state([]);

  let saving = $state(false);
  let error = $state('');
  let editingProfile = $state(null);
  let contextMenu = $state(null);
  let connectBtn;
  let formGrid;

  onMount(async () => {
    try {
      savedProfiles = await loadProfiles();
    } catch (e) {
      console.error('Failed to load profiles:', e);
    }
    if (initialProfile) {
      loadIntoForm(initialProfile);
      requestAnimationFrame(() => formGrid?.scrollIntoView({ behavior: 'smooth', block: 'start' }));
    }
  });

  function loadIntoForm(profile) {
    editingProfile = profile.name;
    name = profile.name;
    host = profile.ssh.host;
    port = profile.ssh.port;
    user = profile.ssh.user ?? '';
    identityFile = profile.ssh.identity_file ?? '';
    jumpHost = profile.ssh.jump_host ?? '';
    agentForwarding = profile.ssh.agent_forwarding ?? false;
    portForwards = profile.ssh.port_forwards?.map((f) => ({ ...f })) ?? [];
    requestAnimationFrame(() => connectBtn?.scrollIntoView({ behavior: 'smooth', block: 'nearest' }));
  }

  function clearForm() {
    editingProfile = null;
    name = '';
    host = '';
    port = 22;
    user = '';
    identityFile = '';
    jumpHost = '';
    agentForwarding = false;
    portForwards = [];
    error = '';
  }

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
    if (contextMenu) loadIntoForm(contextMenu.profile);
    closeContextMenu();
  }

  async function contextDelete() {
    if (!contextMenu) return;
    const profileName = contextMenu.profile.name;
    closeContextMenu();
    if (!confirm(`Delete profile "${profileName}"?`)) return;
    try {
      await deleteProfile(profileName);
      savedProfiles = savedProfiles.filter((p) => p.name !== profileName);
      if (editingProfile === profileName) clearForm();
    } catch (err) {
      console.error('Failed to delete profile:', err);
    }
  }

  function addPortForward() {
    portForwards.push({ local_port: 0, remote_host: 'localhost', remote_port: 0 });
  }

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
    if (onConnect) onConnect(buildSshProfile());
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
    saving = true;
    try {
      const profile = { name, ssh: buildSshProfile(), rows: 24, cols: 80 };
      await saveProfile(profile);
      const idx = savedProfiles.findIndex((p) => p.name === name);
      if (idx >= 0) {
        savedProfiles[idx] = profile;
      } else {
        savedProfiles.push(profile);
      }
      savedProfiles = savedProfiles;
      editingProfile = name;
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
    {#if editingProfile && host}
      <div class="profile-banner">
        <span class="banner-text">📋 <strong>{editingProfile}</strong> — {user ? `${user}@` : ''}{host}:{port}</span>
        <button class="btn primary banner-connect" onclick={handleConnect}>▶ Connect</button>
      </div>
    {/if}

    <h2>{editingProfile ? `Edit: ${editingProfile}` : 'New Connection'}</h2>

    {#if savedProfiles.length > 0}
      <div class="saved-profiles">
        <h3>Saved Profiles <span class="hint">(click = load, right-click = options)</span></h3>
        <div class="profile-list">
          {#each savedProfiles as profile}
            <div
              class="profile-item"
              class:editing={editingProfile === profile.name}
              onclick={() => loadIntoForm(profile)}
              oncontextmenu={(e) => handleContextMenu(e, profile)}
              role="button"
              tabindex="0"
              onkeydown={(e) => e.key === 'Enter' && loadIntoForm(profile)}
            >
              <div class="profile-info">
                <span class="profile-name">{profile.name}</span>
                <span class="profile-host">
                  {profile.ssh.user ? `${profile.ssh.user}@` : ''}{profile.ssh.host}:{profile.ssh.port}
                </span>
              </div>
            </div>
          {/each}
        </div>
      </div>
      <hr class="divider">
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
      <input id="cd-identity" type="text" bind:value={identityFile} placeholder="~/.ssh/id_rsa">

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
        <button class="btn secondary" onclick={clearForm}>New</button>
        <button class="btn success" onclick={handleSave} disabled={saving}>
          {saving ? 'Saving...' : '💾 Save'}
        </button>
      {/if}
      <button class="btn primary" onclick={handleConnect} bind:this={connectBtn}>Connect</button>
      <button class="btn success" onclick={handleSaveAndConnect} disabled={saving}>
        {saving ? 'Saving...' : '💾 Save & Connect'}
      </button>
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

  .profile-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 10px 14px;
    background: #1a3a1a;
    border: 1px solid #2d6a30;
    border-radius: 6px;
    margin: -24px -24px 12px -24px;
    padding: 12px 24px;
    position: sticky;
    top: -24px;
    z-index: 10;
  }

  .banner-text {
    font-size: 13px;
    color: #c8e6c9;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .banner-connect {
    flex-shrink: 0;
    padding: 6px 14px;
    font-size: 13px;
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

  .btn.secondary {
    background: #3c3c3c;
    color: #ccc;
  }

  .btn.secondary:hover {
    background: #4a4a4a;
  }

  .btn.save {
    background: #2d6a30;
    color: #fff;
  }

  .btn.save:hover {
    background: #38833c;
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
