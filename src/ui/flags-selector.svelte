<script lang="ts">
    import { fade } from 'svelte/transition';

    interface Props {
        algorithm: string;
        onflagsChanged?: (flags: string[]) => void;
    }

    let { algorithm, onflagsChanged }: Props = $props();

    let selectedFlags = $state<string[]>([]);

    const flagsMap: Record<string, string[]> = {
        "xz": ["-0", "-1", "-6", "-9", "--extreme"],
        "7zip": ["-mx1", "-mx5", "-mx9"],
        "lpaq": ["1", "2", "3", "4"],
        "paq8x": ["-0", "-1", "-2", "-3"]
    };

    let availableFlags = $derived(flagsMap[algorithm] || []);

    function toggleFlag(flag: string) {
        if (selectedFlags.includes(flag)) {
            selectedFlags = selectedFlags.filter(f => f !== flag);
        } else {
            selectedFlags = [...selectedFlags, flag];
        }
        onflagsChanged?.(selectedFlags);
    }
</script>

<div class="flags-selector" transition:fade>
    <h3>Flags for {algorithm}</h3>
    <div class="flag-buttons">
        {#each availableFlags as flag}
            <button
                onclick={() => toggleFlag(flag)}
                class:selected={selectedFlags.includes(flag)}
            >
                {flag}
            </button>
        {/each}
    </div>
</div>

<style>
    .flags-selector {
        margin-top: 1rem;
    }
    .flag-buttons {
        display: flex;
        gap: 0.5rem;
        flex-wrap: wrap;
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
