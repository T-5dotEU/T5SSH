<script>
  import Terminal from '$lib/terminal/Terminal.svelte';
  import TabBar from '$lib/tabs/TabBar.svelte';
  import ConnectionDialog from '$lib/connection/ConnectionDialog.svelte';
  import ProfileList from '$lib/connection/ProfileList.svelte';
  import { tabStore } from '$lib/tabs/TabStore.svelte.js';

  let showConnectionDialog = $state(false);
  let showProfileList = $state(false);
  let editProfile = $state(null);

  // Show connection dialog on start if no tabs exist
  $effect(() => {
    if (tabStore.tabs.length === 0 && !showConnectionDialog && !showProfileList) {
      showConnectionDialog = true;
    }
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
  }
</script>

<main>
  <TabBar
    onNewTab={() => showConnectionDialog = true}
    onOpenProfiles={() => showProfileList = true}
  />
  <div class="terminals">
    {#each tabStore.tabs as tab (tab.id)}
      <div class="terminal-pane" class:hidden={tabStore.activeTabId !== tab.id}>
        <Terminal
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
