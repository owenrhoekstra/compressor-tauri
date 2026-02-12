<script lang="ts">
    import { fade } from 'svelte/transition';
    import { listen } from '@tauri-apps/api/event';
    import { onMount } from 'svelte';
    import { compressionStore } from '../../domain/ui-state-store.svelte.js';
    import { startCompression } from '../../domain/compression-api';

    interface Props {
        onstop?: () => void;
    }

    let { onstop }: Props = $props();

    let progress = $state(0);
    let statusMessage = $state('Initializing...');
    let errorMessage = $state<string | null>(null);
    let isFinished = $state(false);

    interface CompressionEvent {
        algorithm: string;
        event_type: 'progress' | 'status' | 'error' | 'success';
        message: string;
        percentage: number | null;
    }

    onMount(() => {
        const unlisten = listen<CompressionEvent>('compression-event', (event) => {
            const { event_type, message, percentage } = event.payload;
            console.log('Received event:', event.payload);
            
            if (event_type === 'error') {
                errorMessage = message;
            } else if (event_type === 'success') {
                statusMessage = message;
                progress = 100;
                isFinished = true;
            } else {
                statusMessage = message;
                if (percentage !== null) {
                    progress = percentage;
                }
            }
        });

        // Start compression after setting up listener
        startCompression(
            compressionStore.algorithm,
            compressionStore.flags,
            compressionStore.inputPath,
            compressionStore.outputPath
        ).catch(e => {
            errorMessage = String(e);
        });

        return () => {
            unlisten.then(f => f());
        };
    });
</script>

<div class="execution-view" transition:fade|global>
    {#if errorMessage}
        <div class="error-container">
            <h3>Error</h3>
            <div class="error-message">
                {errorMessage}
            </div>
            <button class="close-button" onclick={onstop}>
                Close
            </button>
        </div>
    {:else}
        <h2>{isFinished ? 'Compression Complete' : 'Compressing...'}</h2>
        <div class="progress-container">
            <div class="progress-bar" style="width: {progress}%"></div>
        </div>
        <div class="progress-text">
            <span>{progress.toFixed(1)}% Complete</span>
        </div>
        
        <div class="status-box">
            <p class="status-message">{statusMessage}</p>
        </div>

        {#if !isFinished}
            <button class="stop-button" onclick={onstop}>
                Stop
            </button>
        {:else}
            <button class="finish-button" onclick={onstop}>
                Done
            </button>
        {/if}
    {/if}
</div>

<style>
    .execution-view {
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-top: 2rem;
        width: 100%;
        max-width: 600px;
        margin-left: auto;
        margin-right: auto;
    }

    .progress-container {
        width: 100%;
        height: 20px;
        background-color: #eee;
        border-radius: 10px;
        overflow: hidden;
        margin-bottom: 0.5rem;
        border: 1px solid #ddd;
    }

    .progress-bar {
        height: 100%;
        background-color: #007acc;
        transition: width 0.3s ease;
    }

    .progress-text {
        font-weight: bold;
        margin-bottom: 1.5rem;
    }

    .status-box {
        width: 100%;
        background: #f8f9fa;
        border: 1px solid #e9ecef;
        border-radius: 4px;
        padding: 1rem;
        margin-bottom: 1.5rem;
        min-height: 60px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .status-message {
        margin: 0;
        font-family: monospace;
        font-size: 0.9rem;
        text-align: center;
        word-break: break-all;
    }

    .stop-button, .finish-button, .close-button {
        padding: 0.6rem 2.5rem;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-weight: bold;
        transition: background-color 0.2s;
    }

    .stop-button {
        background-color: #dc3545;
        color: white;
    }

    .stop-button:hover {
        background-color: #c82333;
    }

    .finish-button {
        background-color: #28a745;
        color: white;
    }

    .finish-button:hover {
        background-color: #218838;
    }

    .close-button {
        background-color: #6c757d;
        color: white;
    }

    .close-button:hover {
        background-color: #5a6268;
    }

    .error-container {
        width: 100%;
        padding: 2rem;
        background-color: #fff5f5;
        border: 2px solid #feb2b2;
        border-radius: 8px;
        text-align: center;
    }

    .error-container h3 {
        color: #c53030;
        margin-top: 0;
    }

    .error-message {
        background: white;
        border: 1px solid #feb2b2;
        padding: 1rem;
        border-radius: 4px;
        margin-bottom: 1.5rem;
        font-family: monospace;
        text-align: left;
        white-space: pre-wrap;
        word-break: break-all;
        max-height: 200px;
        overflow-y: auto;
    }
</style>
