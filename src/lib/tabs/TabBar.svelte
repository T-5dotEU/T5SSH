<script>
  import { tabStore } from './TabStore.svelte.js';
  import { closeSession } from '$lib/api/session.js';

  function switchTab(id) {
    tabStore.setActiveTab(id);
  }

  function addTab() {
    const defaultProfile = {
      host: 'localhost',
      port: 22,
      user: null,
      identity_file: null,
      jump_host: null,
      port_forwards: [],
      agent_forwarding: false,
    };
    tabStore.addTab(defaultProfile);
  }

  async function removeTab(e, id) {
    e.stopPropagation();
    const tab = tabStore.getTab(id);
    if (tab && tab.sessionId) {
      await closeSession(tab.sessionId).catch(() => {});
    }
    tabStore.removeTab(id);
  }
</script>

<div class="tab-bar">
  {#each tabStore.tabs as tab (tab.id)}
    <button
      class="tab"
      class:active={tabStore.activeTabId === tab.id}
      class:disconnected={tab.disconnected}
      onclick={() => switchTab(tab.id)}
    >
      <span class="tab-label">{tab.label}</span>
      <span class="tab-close" onclick={(e) => removeTab(e, tab.id)}>×</span>
    </button>
  {/each}
  <button class="tab add-tab" onclick={addTab}>+</button>
</div>

<style>
  .tab-bar {
    display: flex;
    background: #252525;
    border-bottom: 1px solid #3c3c3c;
    height: 36px;
    flex-shrink: 0;
    overflow-x: auto;
    user-select: none;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 12px;
    background: #2d2d2d;
    border: none;
    border-right: 1px solid #3c3c3c;
    color: #999;
    font-size: 13px;
    cursor: pointer;
    white-space: nowrap;
    min-width: 0;
  }

  .tab:hover {
    background: #333;
    color: #ccc;
  }

  .tab.active {
    background: #1e1e1e;
    color: #d4d4d4;
    border-bottom: 2px solid #007acc;
  }

  .tab.disconnected .tab-label {
    opacity: 0.5;
    font-style: italic;
  }

  .tab-close {
    font-size: 16px;
    line-height: 1;
    opacity: 0.5;
  }

  .tab-close:hover {
    opacity: 1;
    color: #e04040;
  }

  .add-tab {
    color: #888;
    font-size: 18px;
    padding: 0 14px;
    background: transparent;
    border-right: none;
  }

  .add-tab:hover {
    color: #d4d4d4;
    background: #333;
  }
</style>
