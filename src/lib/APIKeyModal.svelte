<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';

    async function setKey() {
        await invoke('set_api_key', {
            keyVarName,
            key,
        });
        dialog.close();
    }

    export let keyName = '';
    export let keyVarName = '';

    let key = '';
    let dialog: HTMLDialogElement;
</script>

<dialog bind:this={dialog} open>
    <h1>Add API Key: {keyName}</h1>

    <form class="row" on:submit|preventDefault={setKey}>
        <input id="api-key-input" placeholder="API Key" bind:value={key} />
        <button type="submit">Save</button>
    </form>
    <button
        on:click={() => {
            dialog.close();
        }}>Cancel</button
    >
</dialog>

<style></style>
