<script lang="ts">
    import { compressionStore } from '../../domain/ui-state-store.svelte.js';
    import { fade } from 'svelte/transition';

    interface Props {
        onconfirm: () => void;
        onback: () => void;
    }

    let { onconfirm, onback }: Props = $props();
</script>

<div class="confirmation-view" transition:fade|global>
    <h2>Confirm Details</h2>
    
    <div class="details">
        <p><strong>Algorithm:</strong> {compressionStore.algorithm}</p>
        <p><strong>Flags:</strong> {compressionStore.flags.length > 0 ? compressionStore.flags.join(', ') : 'None'}</p>
        <p><strong>Input Path:</strong> {compressionStore.inputPath}</p>
        <p><strong>Output Path:</strong> {compressionStore.outputPath}</p>
    </div>

    <div class="actions">
        <button class="back-button" onclick={onback}>Back</button>
        <button class="confirm-button" onclick={onconfirm}>Confirm and Start</button>
    </div>
</div>

<style>
    .confirmation-view {
        padding: 1.5rem;
        border: 1px solid #444;
        background: #1e1e1e;
        border-radius: 8px;
        margin-top: 1rem;
        color: white;
    }
    h2 {
        margin-top: 0;
    }
    .details {
        margin-bottom: 2rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
    .details p {
        margin: 0;
    }
    .actions {
        display: flex;
        gap: 1rem;
    }
    button {
        padding: 0.75rem 1.5rem;
        cursor: pointer;
        border-radius: 4px;
        border: none;
        font-weight: bold;
    }
    .back-button {
        background: #444;
        color: white;
    }
    .back-button:hover {
        background: #555;
    }
    .confirm-button {
        background: #28a745;
        color: white;
        flex: 1;
    }
    .confirm-button:hover {
        background: #218838;
    }
</style>
