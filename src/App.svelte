<script lang="ts">
    import { Router, Route } from 'svelte-routing';
    import Home from './routes/Home.svelte';
    import VPN from './routes/VPN.svelte';
    import Dashboard from './routes/Dashboard.svelte';

    export let url = '';
</script>

<Router {url}>
    <main class="container">
        <Route path="/dashboard" component={Dashboard} />
        <Route path="/vpn" component={VPN} />
        <Route path="/"><Home /></Route>
    </main></Router
>

<!-- <main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>Click on the Tauri, Vite, and Svelte logos to learn more.</p>

  <div class="row">
    <Greet />
  </div>
</main> -->
