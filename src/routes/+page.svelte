<script>
  import { onMount } from 'svelte';
  import Terminal from '$lib/terminal/Terminal.svelte';
  import TabBar from '$lib/tabs/TabBar.svelte';
  import { tabStore } from '$lib/tabs/TabStore.svelte.js';

  onMount(() => {
    if (tabStore.tabs.length === 0) {
      tabStore.addTab({
        host: 'localhost',
        port: 22,
        user: null,
        identity_file: null,
        jump_host: null,
        port_forwards: [],
        agent_forwarding: false,
      });
    }
  });

  function handleExit(sessionId) {
    tabStore.markDisconnected(sessionId);
  }
</script>

<main>
  <TabBar />
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
