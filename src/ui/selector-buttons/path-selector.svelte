<script lang="ts">
    import { fade } from 'svelte/transition';
    import { open } from '@tauri-apps/plugin-dialog';

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

    async function selectInputFile() {
        const selected = await open({
            multiple: false,
            directory: false,
        });
        if (selected) {
            inputPath = selected;
        }
    }

    async function selectInputFolder() {
        const selected = await open({
            multiple: false,
            directory: true,
        });
        if (selected) {
            inputPath = selected;
        }
    }

    async function selectOutputDirectory() {
        const selected = await open({
            multiple: false,
            directory: true,
        });
        if (selected) {
            outputPath = selected;
        }
    }
</script>

<div class="path-selector" transition:fade|global>
    <div class="input-group">
        <label for="input-file">Input (File or Folder):</label>
        <div class="input-row">
            <input id="input-file" type="text" bind:value={inputPath} placeholder="Select input..." />
            <button type="button" class="browse-button" onclick={selectInputFile}>File</button>
            <button type="button" class="browse-button" onclick={selectInputFolder}>Folder</button>
        </div>
    </div>

    <div class="input-group">
        <label for="output-file">Output Folder:</label>
        <div class="input-row">
            <input id="output-file" type="text" bind:value={outputPath} placeholder="Select output destination..." />
            <button type="button" class="browse-button" onclick={selectOutputDirectory}>Browse</button>
        </div>
    </div>

    {#if canStart}
        <button class="start-button" onclick={onstart} transition:fade>
            Review and Confirm
        </button>
    {/if}
</div>

<style>
    .path-selector {
        margin-top: 1.5rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        max-width: 500px;
        width: 100%;
        margin-left: auto;
        margin-right: auto;
    }

    .input-group {
        display: flex;
        flex-direction: column;
        gap: 0.35rem;
        width: 100%;
    }

    .input-row {
        display: flex;
        gap: 0.5rem;
        width: 100%;
    }

    label {
        font-size: 0.95rem;
        color: #ddd;
    }

    input {
        padding: 0.6rem 0.7rem;
        border: 1px solid #555;
        border-radius: 6px;
        background: #2a2a2a;
        color: #eee;
        width: 100%;
        transition: border-color 150ms ease, box-shadow 150ms ease;
    }

    input:focus {
        outline: none;
        border-color: #007acc;
        box-shadow: 0 0 0 3px rgba(0,122,204,0.25);
    }

    .browse-button {
        padding: 0.6rem 1rem;
        background-color: #444;
        color: white;
        border: 1px solid #555;
        border-radius: 6px;
        cursor: pointer;
        font-size: 0.9rem;
        transition: background-color 150ms ease, box-shadow 150ms ease;
        white-space: nowrap;
    }

    .browse-button:hover {
        background-color: #555;
        box-shadow: 0 2px 5px rgba(0,0,0,0.2);
    }

    .browse-button:active {
        background-color: #666;
    }

    .start-button {
        margin-top: 0.5rem;
        padding: 0.8rem 1.2rem;
        background-color: #28a745;
        color: white;
        border: none;
        border-radius: 6px;
        cursor: pointer;
        font-weight: bold;
        transition: background-color 150ms ease, box-shadow 150ms ease, transform 120ms ease;
        align-self: center;
    }

    .start-button:hover {
        background-color: #218838;
        box-shadow: 0 2px 10px rgba(40,167,69,0.35);
    }

    .start-button:active {
        transform: translateY(1px);
    }
</style>
