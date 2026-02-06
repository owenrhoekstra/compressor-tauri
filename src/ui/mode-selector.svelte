<script lang="ts">
    import { fade } from 'svelte/transition';

    interface Props {
        onselect: (mode: 'compress' | 'extract') => void;
    }

    let { onselect }: Props = $props();
</script>

<div class="mode-selector" transition:fade>
    <button onclick={() => onselect('compress')}>Compress</button>
    <button onclick={() => onselect('extract')}>Extract</button>
</div>

<style>
    .mode-selector {
        display: flex;
        justify-content: center;
        gap: 2rem;
        margin-top: 4rem;
    }

    button {
        padding: 1.5rem 3rem;
        font-size: 1.5rem;
        cursor: pointer;
        border: 2px solid #007acc;
        background: transparent;
        color: #007acc;
        border-radius: 8px;
        transition: all 0.2s ease;
    }

    button:hover {
        background-color: #007acc;
        color: white;
    }
</style>
