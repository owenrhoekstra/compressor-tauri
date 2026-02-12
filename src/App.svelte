<script lang="ts">
  import ModeSelector from './ui/selector-buttons/mode-selector.svelte';
  import ExtractView from './ui/views/extract-view.svelte';
  import SelectionParent from './ui/parents/selection-parent.svelte';
  import FlagsSelector from './ui/selector-buttons/flags-selector.svelte';
  import PathSelector from './ui/selector-buttons/path-selector.svelte';
  import ExecutionView from './ui/views/execution-view.svelte';
  import ConfirmationView from './ui/views/confirmation-view.svelte';
  import { compressionStore } from './domain/ui-state-store.svelte.js';
  import { startCompression } from './domain/compression-api';
  import { fade } from 'svelte/transition';

  let mode = $state<'compress' | 'extract' | null>(null);
  let isExecuting = $state(false);
  let isConfirming = $state(false);

  function handleModeSelect(selectedMode: 'compress' | 'extract') {
    mode = selectedMode;
  }

  function goBack() {
    mode = null;
    compressionStore.reset();
    isExecuting = false;
    isConfirming = false;
  }

  function handleAlgorithm(algo: string | null) {
    compressionStore.setAlgorithm(algo);
  }

  function handleFlags(flags: string[]) {
    compressionStore.setFlags(flags);
  }

  function handlePaths(newPaths: { input: string, output: string }) {
    compressionStore.setPaths(newPaths.input, newPaths.output);
  }

  function goToConfirmation() {
    isConfirming = true;
  }

  async function handleConfirm() {
    isConfirming = false;
    isExecuting = true;
    await startCompression(
      compressionStore.algorithm,
      compressionStore.flags,
      compressionStore.inputPath,
      compressionStore.outputPath
    );
  }

  function handleBackFromConfirm() {
    isConfirming = false;
  }

  function stopCompression() {
    isExecuting = false;
  }
</script>

<main>
  <h1>Compression Tool</h1>

  {#if mode === null}
    <ModeSelector onselectMode={handleModeSelect} />
  {:else if mode === 'compress'}
    <div transition:fade|global>
      <button class="back-button" onclick={goBack}>← Back</button>
      
      {#if isExecuting}
        <ExecutionView onstop={stopCompression} />
      {:else if isConfirming}
        <ConfirmationView 
          onconfirm={handleConfirm}
          onback={handleBackFromConfirm}
        />
      {:else}
        <SelectionParent 
          selected={compressionStore.algorithm} 
          onalgorithmSelected={handleAlgorithm}
        />

        {#if compressionStore.algorithm}
          {#key compressionStore.algorithm}
            <FlagsSelector
              algorithm={compressionStore.algorithm}
              onflagsChanged={handleFlags}
            />
          {/key}

          <PathSelector
            onpathsChanged={handlePaths}
            onstart={goToConfirmation}
          />
        {/if}
      {/if}
    </div>
  {:else if mode === 'extract'}
    <div transition:fade|global>
      <button class="back-button" onclick={goBack}>← Back</button>
      <ExtractView />
    </div>
  {/if}
</main>

<style>
  main {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
  }

  h1 {
    text-align: center;
    margin: 0 0 1rem 0;
  }

  .back-button {
    margin-bottom: 1rem;
    padding: 0.5rem 1rem;
    cursor: pointer;
    background: #f0f0f0;
    border: 1px solid #ccc;
    border-radius: 4px;
    color: black;
    transition: background-color 150ms ease, box-shadow 150ms ease, transform 120ms ease;
  }

  .back-button:hover {
    background: #e0e0e0;
    box-shadow: 0 2px 8px rgba(0,0,0,0.15);
  }

  .back-button:active {
    transform: translateY(1px);
  }
</style>