<script>
  import { saveProfile } from '$lib/api/profiles.js';

  let { onConnect = null, onCancel = null, editProfile = null } = $props();

  let name = $state(editProfile?.name ?? '');
  let host = $state(editProfile?.ssh?.host ?? '');
  let port = $state(editProfile?.ssh?.port ?? 22);
  let user = $state(editProfile?.ssh?.user ?? '');
  let identityFile = $state(editProfile?.ssh?.identity_file ?? '');
  let jumpHost = $state(editProfile?.ssh?.jump_host ?? '');
  let agentForwarding = $state(editProfile?.ssh?.agent_forwarding ?? false);
  let portForwards = $state(
    editProfile?.ssh?.port_forwards?.map((f) => ({ ...f })) ?? []
  );

  let saving = $state(false);
  let error = $state('');

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

  async function handleSaveAndConnect() {
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
      await saveProfile({ name, ssh: buildSshProfile(), rows: 24, cols: 80 });
      if (onConnect) onConnect(buildSshProfile());
    } catch (e) {
      error = `Save failed: ${e}`;
    } finally {
      saving = false;
    }
  }

  function handleKeydown(e) {
    if (e.key === 'Escape' && onCancel) onCancel();
    if (e.key === 'Enter' && !e.shiftKey) handleConnect();
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div class="overlay" onkeydown={handleKeydown} role="dialog" aria-modal="true">
  <div class="dialog">
    <h2>New Connection</h2>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    <div class="form-grid">
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
      <button class="btn secondary" onclick={onCancel}>Cancel</button>
      <button class="btn primary" onclick={handleConnect}>Connect</button>
      <button class="btn save" onclick={handleSaveAndConnect} disabled={saving}>
        {saving ? 'Saving...' : '💾 Save & Connect'}
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
    border: 1px dashed #555;
    color: #aaa;
    padding: 4px 10px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
  }

  .btn-small:hover {
    border-color: #888;
    color: #ccc;
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

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
