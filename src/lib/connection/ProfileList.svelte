<script>
  import { onMount } from 'svelte';
  import { loadProfiles, deleteProfile } from '$lib/api/profiles.js';

  let { onCancel = null, onEdit = null, onNewConnection = null } = $props();

  let profiles = $state([]);
  let loading = $state(true);
  let contextMenu = $state(null);

  onMount(async () => {
    try {
      profiles = await loadProfiles();
    } catch (e) {
      console.error('Failed to load profiles:', e);
    } finally {
      loading = false;
    }
  });

  function handleSelect(profile) {
    if (onEdit) onEdit(profile);
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
    if (contextMenu && onEdit) onEdit(contextMenu.profile);
    closeContextMenu();
  }

  async function contextDelete() {
    if (!contextMenu) return;
    const profileName = contextMenu.profile.name;
    closeContextMenu();
    if (!confirm(`Delete profile "${profileName}"?`)) return;
    try {
      await deleteProfile(profileName);
      profiles = profiles.filter((p) => p.name !== profileName);
    } catch (err) {
      console.error('Failed to delete profile:', err);
    }
  }

  function handleKeydown(e) {
    if (e.key === 'Escape') {
      if (contextMenu) { closeContextMenu(); return; }
      if (onCancel) onCancel();
    }
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div class="overlay" onkeydown={handleKeydown} role="dialog" aria-modal="true">
  <div class="dialog">
    <h2>Saved Profiles <span class="hint">(click = load, right-click = options)</span></h2>

    {#if loading}
      <p class="status">Loading...</p>
    {:else if profiles.length === 0}
      <p class="status">No saved profiles yet.</p>
    {:else}
      <div class="profile-list">
        {#each profiles as profile}
          <div
            class="profile-item"
            onclick={() => handleSelect(profile)}
            oncontextmenu={(e) => handleContextMenu(e, profile)}
            onkeydown={(e) => e.key === 'Enter' && handleSelect(profile)}
            role="button"
            tabindex="0"
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
    {/if}

    <div class="actions">
      <button class="btn" onclick={onCancel}>Close</button>
      <button class="btn success" onclick={() => { if (onNewConnection) onNewConnection(); }}>+ New Connection</button>
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
    width: 400px;
    max-height: 60vh;
    overflow-y: auto;
    overflow-x: hidden;
    color: #d4d4d4;
  }

  h2 {
    margin: 0 0 16px;
    font-size: 18px;
    color: #fff;
  }

  .hint {
    font-size: 11px;
    color: #666;
    font-weight: normal;
  }

  .status {
    color: #888;
    font-size: 13px;
    text-align: center;
    padding: 20px;
  }

  .profile-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .profile-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    background: #1e1e1e;
    border: 1px solid #3c3c3c;
    border-radius: 4px;
    cursor: pointer;
    color: #d4d4d4;
    text-align: left;
    width: 100%;
  }

  .profile-item:hover {
    border-color: #007acc;
    background: #252525;
  }

  .profile-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .profile-name {
    font-size: 14px;
    color: #fff;
  }

  .profile-host {
    font-size: 12px;
    color: #888;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 16px;
  }

  .btn {
    padding: 8px 16px;
    background: #3c3c3c;
    border: none;
    border-radius: 4px;
    color: #ccc;
    font-size: 13px;
    cursor: pointer;
  }

  .btn:hover {
    background: #4a4a4a;
  }

  .btn.success {
    background: #2d6a30;
    color: #fff;
  }

  .btn.success:hover {
    background: #38833c;
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
    cursor: pointer;
    text-align: left;
  }

  .context-item:hover {
    background: #094771;
  }

  .context-item.danger {
    color: #e55;
  }

  .context-item.danger:hover {
    background: #5c2020;
  }
</style>
