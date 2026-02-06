<script lang="ts">
  import ModeSelector from './ui/selector-buttons/mode-selector.svelte';
  import ExtractView from './ui/extract-view.svelte';
  import SelectionParent from './ui/selection-parent.svelte';
  import FlagsSelector from './ui/selector-buttons/flags-selector.svelte';
  import PathSelector from './ui/selector-buttons/path-selector.svelte';
  import ExecutionView from './ui/execution-view.svelte';
  import { fade } from 'svelte/transition';

  let mode = $state<'compress' | 'extract' | null>(null);
  let chosenAlgorithm = $state<string | null>(null);
  let chosenFlags = $state<string[]>([]);
  let paths = $state({ input: '', output: '' });
  let isExecuting = $state(false);

  function handleModeSelect(selectedMode: 'compress' | 'extract') {
    mode = selectedMode;
  }

  function goBack() {
    mode = null;
    chosenAlgorithm = null;
    chosenFlags = [];
    paths = { input: '', output: '' };
    isExecuting = false;
  }

  function handleAlgorithm(algo: string | null) {
    chosenAlgorithm = algo;
    // Reset subsequent tiers
    chosenFlags = [];
    paths = { input: '', output: '' };
    console.log('Selected algorithm:', chosenAlgorithm);
  }

  function handleFlags(flags: string[]) {
    chosenFlags = flags;
  }

  function handlePaths(newPaths: { input: string, output: string }) {
    paths = newPaths;
  }

  function startCompression() {
    isExecuting = true;
  }

  function stopCompression() {
    isExecuting = false;
    // Optionally reset or keep state?
    // User said "all goes away and progress bar comes up", 
    // implying we might want to stay in that view until done or stopped.
  }
</script>

<main>
  <h1>Compression Tool</h1>

  {#if mode === null}
    <ModeSelector onselectMode={handleModeSelect} />
  {:else if mode === 'compress'}
    <div transition:fade={{ global: true }}>
      <button class="back-button" onclick={goBack}>← Back</button>
      
      {#if isExecuting}
        <ExecutionView onstop={stopCompression} />
      {:else}
        <SelectionParent selected={chosenAlgorithm} onalgorithmSelected={handleAlgorithm}/>

        {#if chosenAlgorithm}
          {#key chosenAlgorithm}
            <FlagsSelector
              algorithm={chosenAlgorithm}
              onflagsChanged={handleFlags}
            />
          {/key}

          {#if chosenFlags.length > 0}
            <PathSelector
              onpathsChanged={handlePaths}
              onstart={startCompression}
            />
          {/if}
        {/if}
      {/if}
    </div>
  {:else if mode === 'extract'}
    <div transition:fade={{ global: true }}>
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

  .back-button {
    margin-bottom: 1rem;
    padding: 0.5rem 1rem;
    cursor: pointer;
    background: #f0f0f0;
    border: 1px solid #ccc;
    border-radius: 4px;
    color: black;
  }

  .back-button:hover {
    background: #e0e0e0;
  }
</style>