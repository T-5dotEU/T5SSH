<script>
  import Terminal from '$lib/terminal/Terminal.svelte';
</script>

<main>
  <Terminal />
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
