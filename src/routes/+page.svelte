<script>
  import { onMount } from 'svelte';
  import Terminal from '$lib/terminal/Terminal.svelte';
  import TabBar from '$lib/tabs/TabBar.svelte';
  import ConnectionDialog from '$lib/connection/ConnectionDialog.svelte';
  import ProfileList from '$lib/connection/ProfileList.svelte';
  import { tabStore } from '$lib/tabs/TabStore.svelte.js';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';

  let showConnectionDialog = $state(false);
  let showProfileList = $state(false);
  let editProfile = $state(null);
  let terminalRefs = {};
  let everHadTabs = false;

  // Show connection dialog on start; quit app when last tab closed
  $effect(() => {
    if (tabStore.tabs.length > 0) {
      everHadTabs = true;
    } else if (everHadTabs) {
      invoke('quit_app').catch(() => {});
    } else if (!showConnectionDialog && !showProfileList) {
      showConnectionDialog = true;
    }
  });

  onMount(() => {
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

  function handleConnect(sshProfile) {
    tabStore.addTab(sshProfile);
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
</script>

<main>
  <TabBar
    onNewTab={() => showConnectionDialog = true}
    onOpenProfiles={() => showProfileList = true}
    onReconnect={handleReconnect}
  />
  <div class="terminals">
    {#each tabStore.tabs as tab (tab.id)}
      <div class="terminal-pane" class:hidden={tabStore.activeTabId !== tab.id}>
        <Terminal
          bind:this={terminalRefs[tab.id]}
          profile={tab.profile}
          sessionId={tab.sessionId}
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
    initialProfile={editProfile}
  />
{/if}

{#if showProfileList}
  <ProfileList
    onCancel={() => showProfileList = false}
    onEdit={handleProfileEdit}
    onNewConnection={() => { showProfileList = false; showConnectionDialog = true; editProfile = null; }}
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
