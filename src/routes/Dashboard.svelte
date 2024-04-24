<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import ApiKeyModal from '../lib/APIKeyModal.svelte';
    import ApiKeys from '../lib/enums/ApiKeys';

    // async function greet() {
    //     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //     greetMsg = await invoke('greet', { name });
    // }

    let validApiKey: boolean = true;

    let apiCheckResult: Promise<boolean> = invoke('check_api_key', {
        keyVarName: ApiKeys.OPENAI_API_KEY,
    });

    apiCheckResult.then((result) => {
        validApiKey = result;
        console.log(validApiKey);
    });
</script>

<div>
    <h1>Battle Buddy</h1>
    {#if !validApiKey}
        <ApiKeyModal keyName="OpenAI" keyVarName={ApiKeys.OPENAI_API_KEY} />
    {/if}
</div>

<style></style>
