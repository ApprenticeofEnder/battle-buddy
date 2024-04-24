<script lang="ts">
    import { Link } from 'svelte-routing';
    import NavButton from '../lib/NavButton.svelte';
</script>

<div>
    <h1>Battle Buddy</h1>
    <div class="row">
        <NavButton to="/vpn">VPN</NavButton>
        <NavButton to="/dashboard">Local</NavButton>
    </div>
</div>

<style></style>
