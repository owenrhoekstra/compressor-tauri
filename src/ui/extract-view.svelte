<script lang="ts">
    import { fade } from 'svelte/transition';

    interface Props {
        onback?: () => void;
    }

    let { onback } = $props<Props>();
</script>

<div class="extract-view" transition:fade={{ global: true }}>
    <h2>Extract Archive</h2>
    <p>The extraction module is currently under development.</p>
    <div class="dummy-box">
        Drop files here (Dummy)
    </div>
</div>

<style>
    .extract-view {
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-top: 2rem;
        text-align: center;
    }

    .dummy-box {
        margin-top: 2rem;
        width: 300px;
        height: 200px;
        border: 2px dashed #ccc;
        display: flex;
        align-items: center;
        justify-content: center;
        color: #999;
        border-radius: 8px;
    }
</style>
