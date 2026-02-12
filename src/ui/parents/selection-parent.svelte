<script lang="ts">
    import { fade } from 'svelte/transition';

    interface Props {
        selected?: string | null;
        onalgorithmSelected?: (algo: string | null) => void;
    }

    let { selected = null, onalgorithmSelected }: Props = $props();

    let selectedAlgorithm = $state<string | null>(null);

    $effect(() => {
        selectedAlgorithm = selected;
    });
    const algorithms = ["zstd", "xz", "7zip", "zpaq", "paq8x"];

    function selectAlgorithm(algo: string) {
        if (selectedAlgorithm === algo) {
            selectedAlgorithm = null;
        } else {
            selectedAlgorithm = algo;
        }
        onalgorithmSelected?.(selectedAlgorithm);
    }
</script>

<div class="algorithm-buttons" transition:fade|global>
    {#each algorithms as algo}
        <button
                onclick={() => selectAlgorithm(algo)}
                class:selected={selectedAlgorithm === algo}
        >
            {algo}
        </button>
    {/each}
</div>

<style>
    .algorithm-buttons {
        display: flex;
        gap: 0.75rem;
        justify-content: center;
        flex-wrap: wrap;
        margin-top: 0.5rem;
    }

    button {
        padding: 0.6rem 1.1rem;
        cursor: pointer;
        border: 1px solid #444;
        background: #2a2a2a;
        color: #ddd;
        border-radius: 6px;
        transition: background-color 150ms ease, color 150ms ease, box-shadow 150ms ease, transform 120ms ease;
    }

    button:hover {
        background-color: #3a3a3a;
        color: #fff;
        box-shadow: 0 2px 8px rgba(0,0,0,0.25);
    }

    button:active {
        transform: translateY(1px);
    }

    button.selected {
        background-color: #007acc;
        color: white;
        border-color: #007acc;
        box-shadow: 0 2px 10px rgba(0,122,204,0.35);
    }
</style>