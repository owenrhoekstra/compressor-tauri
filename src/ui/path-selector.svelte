<script lang="ts">
    import { fade } from 'svelte/transition';

    interface Props {
        onpathsChanged?: (paths: { input: string, output: string }) => void;
        onstart?: () => void;
    }

    let { onpathsChanged, onstart }: Props = $props();

    let inputPath = $state("");
    let outputPath = $state("");

    $effect(() => {
        onpathsChanged?.({ input: inputPath, output: outputPath });
    });

    let canStart = $derived(inputPath.trim() !== "" && outputPath.trim() !== "");
</script>

<div class="path-selector" transition:fade>
    <div class="input-group">
        <label for="input-file">Input File:</label>
        <input id="input-file" type="text" bind:value={inputPath} placeholder="Select input file..." />
    </div>

    <div class="input-group">
        <label for="output-file">Output File:</label>
        <input id="output-file" type="text" bind:value={outputPath} placeholder="Select output destination..." />
    </div>

    {#if canStart}
        <button class="start-button" onclick={onstart} transition:fade>
            Start Compression
        </button>
    {/if}
</div>

<style>
    .path-selector {
        margin-top: 1.5rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        max-width: 400px;
    }

    .input-group {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    input {
        padding: 0.5rem;
        border: 1px solid #ccc;
        border-radius: 4px;
    }

    .start-button {
        margin-top: 1rem;
        padding: 0.75rem;
        background-color: #28a745;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-weight: bold;
    }

    .start-button:hover {
        background-color: #218838;
    }
</style>
