<script>
  import Terminal from '$lib/terminal/Terminal.svelte';

  const testProfile = {
    host: 'localhost',
    port: 22,
    user: null,
    identity_file: null,
    jump_host: null,
    port_forwards: [],
    agent_forwarding: false,
  };
</script>

<main>
  <Terminal profile={testProfile} />
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
  }
</style>
