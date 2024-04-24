<script lang="ts">
    import { navigate } from 'svelte-routing';

    export let to = '';
</script>

<button
    on:click={() => {
        navigate(to);
    }}><slot /></button
>

<style></style>
