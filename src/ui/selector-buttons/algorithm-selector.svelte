<script lang="ts">
    let selectedAlgorithm = $state<string | null>(null);

    const algorithms = ["xz", "7zip", "lpaq", "paq8x"];
</script>

<div class="algorithm-buttons">
    {#each algorithms as algo}
        <button
                onclick={() => selectedAlgorithm = algo}
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
        padding: 0
    }
    </style>