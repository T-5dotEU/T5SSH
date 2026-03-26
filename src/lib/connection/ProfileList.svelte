<script>
  import { onMount } from 'svelte';
  import { loadProfiles, deleteProfile } from '$lib/api/profiles.js';

  let { onSelect = null, onCancel = null } = $props();

  let profiles = $state([]);
  let loading = $state(true);

  onMount(async () => {
    try {
      profiles = await loadProfiles();
    } catch (e) {
      console.error('Failed to load profiles:', e);
    } finally {
      loading = false;
    }
  });

  async function handleDelete(e, name) {
    e.stopPropagation();
    try {
      await deleteProfile(name);
      profiles = profiles.filter((p) => p.name !== name);
    } catch (err) {
      console.error('Failed to delete profile:', err);
    }
  }

  function handleSelect(profile) {
    if (onSelect) onSelect(profile.ssh);
  }

  function handleKeydown(e) {
    if (e.key === 'Escape' && onCancel) onCancel();
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div class="overlay" onkeydown={handleKeydown} role="dialog" aria-modal="true">
  <div class="dialog">
    <h2>Saved Profiles</h2>

    {#if loading}
      <p class="status">Loading...</p>
    {:else if profiles.length === 0}
      <p class="status">No saved profiles yet.</p>
    {:else}
      <div class="profile-list">
        {#each profiles as profile}
          <div class="profile-item" onclick={() => handleSelect(profile)} onkeydown={(e) => e.key === 'Enter' && handleSelect(profile)} role="button" tabindex="0">
            <div class="profile-info">
              <span class="profile-name">{profile.name}</span>
              <span class="profile-host">
                {profile.ssh.user ? `${profile.ssh.user}@` : ''}{profile.ssh.host}:{profile.ssh.port}
              </span>
            </div>
            <button class="btn-delete" onclick={(e) => handleDelete(e, profile.name)}>×</button>
          </div>
        {/each}
      </div>
    {/if}

    <div class="actions">
      <button class="btn" onclick={onCancel}>Close</button>
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
    width: 400px;
    max-height: 60vh;
    overflow-y: auto;
    color: #d4d4d4;
  }

  h2 {
    margin: 0 0 16px;
    font-size: 18px;
    color: #fff;
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

  .btn-delete {
    background: none;
    border: none;
    color: #666;
    font-size: 18px;
    cursor: pointer;
    padding: 0 4px;
  }

  .btn-delete:hover {
    color: #e44;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
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
</style>
