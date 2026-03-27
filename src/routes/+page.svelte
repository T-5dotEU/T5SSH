<script>
  import { onMount } from 'svelte';
  import Terminal from '$lib/terminal/Terminal.svelte';
  import TabBar from '$lib/tabs/TabBar.svelte';
  import ConnectionDialog from '$lib/connection/ConnectionDialog.svelte';
  import ProfileList from '$lib/connection/ProfileList.svelte';
  import SettingsDialog from '$lib/settings/SettingsDialog.svelte';
  import { tabStore } from '$lib/tabs/TabStore.svelte.js';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { getSettings } from '$lib/api/settings.js';

  let showConnectionDialog = $state(false);
  let showProfileList = $state(false);
  let showSettings = $state(false);
  let editProfile = $state(null);
  let terminalRefs = {};
  let everHadTabs = false;
  let terminalSettings = $state(null);

  // Show connection dialog on start; ask before quitting when last tab closed
  $effect(() => {
    if (tabStore.tabs.length > 0) {
      everHadTabs = true;
    } else if (everHadTabs) {
      if (confirm('Last tab closed — quit the app?')) {
        invoke('quit_app').catch(() => {});
      } else {
        everHadTabs = false;
        showConnectionDialog = true;
      }
    } else if (!showConnectionDialog && !showProfileList) {
      showConnectionDialog = true;
    }
  });

  onMount(() => {
    getSettings().then(s => {
      if (s.terminal) terminalSettings = s.terminal;
    }).catch(() => {});

    const unlisten = listen('app:close-requested', () => {
      const activeSessions = tabStore.tabs.filter(t => !t.disconnected);
      if (activeSessions.length === 0) {
        invoke('quit_app');
        return;
      }
      if (confirm(`${activeSessions.length} active session(s) — quit and close all?`)) {
        invoke('quit_app');
      }
    });
    return () => { unlisten.then(fn => fn()); };
  });

  function handleConnect(sshProfile, password, profileName) {
    tabStore.addTab(sshProfile, password, profileName);
    showConnectionDialog = false;
    editProfile = null;
  }

  function handleProfileEdit(profile) {
    showProfileList = false;
    editProfile = profile;
    showConnectionDialog = true;
  }

  function handleExit(sessionId) {
    tabStore.markDisconnected(sessionId);
    const tab = tabStore.tabs.find(t => t.sessionId === sessionId);
    if (tab) delete terminalRefs[tab.id];
  }

  function handleReconnect(tabId) {
    tabStore.markReconnecting(tabId);
    const ref = terminalRefs[tabId];
    if (ref) ref.reconnect();
  }

  function handleSettingsApply(ts) {
    terminalSettings = ts;
  }
</script>

<main>
  <TabBar
    onNewTab={() => showConnectionDialog = true}
    onOpenProfiles={() => showProfileList = true}
    onReconnect={handleReconnect}
    onOpenSettings={() => showSettings = true}
  />
  <div class="terminals">
    {#each tabStore.tabs as tab (tab.id)}
      <div class="terminal-pane" class:hidden={tabStore.activeTabId !== tab.id}>
        <Terminal
          bind:this={terminalRefs[tab.id]}
          profile={tab.profile}
          sessionId={tab.sessionId}
          password={tab.password}
          profileName={tab.profileName}
          {terminalSettings}
          onSessionCreated={(sid) => tabStore.setSessionId(tab.id, sid)}
          onExit={handleExit}
        />
      </div>
    {/each}
  </div>
</main>

{#if showConnectionDialog}
  <ConnectionDialog
    onConnect={handleConnect}
    onCancel={() => { showConnectionDialog = false; editProfile = null; }}
    onOpenSettings={() => showSettings = true}
    initialProfile={editProfile}
    canCancel={tabStore.tabs.length > 0}
  />
{/if}

{#if showProfileList}
  <ProfileList
    onCancel={() => showProfileList = false}
    onEdit={handleProfileEdit}
    onNewConnection={() => { showProfileList = false; showConnectionDialog = true; editProfile = null; }}
  />
{/if}

{#if showSettings}
  <SettingsDialog
    onClose={() => showSettings = false}
    onApply={handleSettingsApply}
  />
{/if}

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
    background-color: #1e1e1e;
    color-scheme: dark;
  }

  :global(::-webkit-scrollbar) {
    width: 16px;
    height: 16px;
  }

  :global(::-webkit-scrollbar-track) {
    background: #2d2d2d;
  }

  :global(::-webkit-scrollbar-thumb) {
    background: #555;
    border-radius: 4px;
  }

  :global(::-webkit-scrollbar-thumb:hover) {
    background: #777;
  }

  :global(select) {
    background: #1e1e1e;
    color: #d4d4d4;
  }

  :global(select option) {
    background: #1e1e1e;
    color: #d4d4d4;
  }

  main {
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .terminals {
    flex: 1;
    position: relative;
    overflow: hidden;
  }

  .terminal-pane {
    position: absolute;
    inset: 0;
  }

  .terminal-pane.hidden {
    visibility: hidden;
  }
</style>
