<script lang="ts">
    import { fade } from 'svelte/transition';

    interface Props {
        onstop?: () => void;
    }

    let { onstop }: Props = $props();

    let progress = $state(0);

    $effect(() => {
        const interval = setInterval(() => {
            if (progress < 100) {
                progress += 1;
            } else {
                clearInterval(interval);
            }
        }, 100);

        return () => clearInterval(interval);
    });
</script>

<div class="execution-view" transition:fade|global>
    <h2>Compressing...</h2>
    <div class="progress-container">
        <div class="progress-bar" style="width: {progress}%"></div>
    </div>
    <p>{progress}% Complete</p>

    <button class="stop-button" onclick={onstop}>
        Stop
    </button>
</div>

<style>
    .execution-view {
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-top: 2rem;
        width: 100%;
    }

    .progress-container {
        width: 100%;
        max-width: 500px;
        height: 20px;
        background-color: #eee;
        border-radius: 10px;
        overflow: hidden;
        margin-bottom: 0.5rem;
    }

    .progress-bar {
        height: 100%;
        background-color: #007acc;
        transition: width 0.1s ease;
    }

    .stop-button {
        margin-top: 1.5rem;
        padding: 0.5rem 2rem;
        background-color: #dc3545;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    .stop-button:hover {
        background-color: #c82333;
    }
</style>
