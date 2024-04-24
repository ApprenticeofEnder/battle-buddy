<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import ApiKeyModal from '../lib/APIKeyModal.svelte';

    // async function greet() {
    //     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //     greetMsg = await invoke('greet', { name });
    // }

    let validApiKey: boolean = true;

    let apiCheckResult: Promise<boolean> = invoke('check_openai_key');

    apiCheckResult.then((result) => {
        validApiKey = result;
    });
</script>

<div>
    <h1>Battle Buddy</h1>
    {#if !validApiKey}
        <ApiKeyModal keyName="OpenAI" keyVarName="OPEN_API_KEY" />
    {/if}
</div>

<style></style>
