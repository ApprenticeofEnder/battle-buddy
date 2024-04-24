<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';

    async function setKey() {}

    export let keyName = '';
    export let keyVarName = '';

    let key = '';
</script>

<dialog open>
    <h1>Add API Key: {keyName}</h1>

    <form class="row" on:submit|preventDefault={setKey}>
        <input id="api-key-input" placeholder="API Key" bind:value={key} />
        <button type="submit">Save</button>
    </form>
</dialog>

<style></style>
