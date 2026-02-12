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
        gap: 0.5rem;
    }

    button {
        padding: 0.5rem 1rem;
        cursor: pointer;
    }

    button.selected {
        background-color: #007acc;
        color: white;
    }
</style>