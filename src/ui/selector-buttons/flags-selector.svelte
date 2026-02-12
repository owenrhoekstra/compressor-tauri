<script lang="ts">
    import { fade } from 'svelte/transition';

    interface Props {
        algorithm: string;
        onflagsChanged?: (flags: string[]) => void;
    }

    interface FlagOption {
        value: string;
        label: string;
        children?: string[];
        input?: {
            placeholder: string;
            prefix?: string;
        };
        isPlaceholder?: boolean;
        childPrefix?: string;
        childSuffix?: string;
    }

    let { algorithm, onflagsChanged }: Props = $props();

    let selectedFlags = $state<string[]>([]);

    const flagsMap: Record<string, FlagOption[]> = {
        zstd: [
            {
                value: "level",
                label: "Compression Level",
                isPlaceholder: true,
                input: {
                    placeholder: "0 – 19",
                    prefix: "-"
                }
            },
            {
              value: "ultra",
              label: "Ultra Compression Levels",
                children: ["20", "21", "22"],
                childPrefix: "ultra-",
                isPlaceholder: true
            },
            {
                value: "-T",
                label: "# of Threads",
                isPlaceholder: true,
                input: {
                    placeholder: "0 = auto",
                    prefix: "-T"
                }
            },
        ],
        xz: [
            {
                value: "level",
                label: "Compression Level",
                children: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
                isPlaceholder: true,
                childPrefix: "-"
            },
            {
                value: "extreme",
                label: "Extreme Mode",
                children: ["On", "Off"],
                isPlaceholder: true
            },
            {
                value: "threads",
                label: "# of Threads",
                isPlaceholder: true,
                input: {
                    placeholder: "0 = auto",
                    prefix: "-T"
                }
            },
            {
                value: "lzma2",
                label: "Dictionary Size",
                children: ["16M", "32M", "64M", "128M", "256M", "512M", "1G", "2G"],
                isPlaceholder: true,
                childPrefix: "lzma2[dict=",
                childSuffix: "]"
            },
            {
                value: "-C",
                label: "Checksum",
                children: ["crc32", "crc64", "sha256"],
                isPlaceholder: true,
                childPrefix: "-C "
            }
        ],

        "7zip": [
            {
                value: "-mx",
                label: "Compression Level",
                children: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
                isPlaceholder: true
            },
            {
                value: "-p",
                label: "Password",
                input: {
                    placeholder: "Password",
                    prefix: "-p"
                }
            },
            {
                value: "-ms=",
                label: "Solid Compression",
                children: ["On", "Off"],
                isPlaceholder: true,
            },
            {
                value: "-mmt",
                label: "# of Threads",
                input: {
                    placeholder: "0 = auto",
                    prefix: "-nmt"
                }
            },
            {
                value: "md",
                label: "Dictionary Size",
                children: ["16M", "32M", "64M", "128M", "256M", "512M", "1G", "2G", "4G"],

            }

        ],
        "zpaq": [

        ],
        "paq8x": [
            {
                value: "level",
                label: "Compression Level",
                children: ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12"],
                isPlaceholder: true,
                childPrefix: "-"
            },
            { value: "-L", label: "Enable LSTM" },
            { value: "-A", label: "Adaptive Learning" },
            { value: "-v", label: "Verbose" }
        ]
    };

    let openMenus = $state<Record<string, boolean>>({});

    function toggleMenu(flag: string) {
        openMenus[flag] = !openMenus[flag];
    }

    let availableFlags = $derived(flagsMap[algorithm] || []);

    const processedFlags = $derived.by(() => {
        let result = [...selectedFlags];

        if (algorithm === 'xz') {
            const hasExtreme = result.includes('extremeOn');
            const levelFlag = result.find(f => /^-\d$/.test(f));

            // Clean up internal markers for dropdowns
            result = result.filter(f => f !== 'extremeOn' && f !== 'extremeOff');

            if (hasExtreme) {
                if (levelFlag) {
                    result = result.map(f => f === levelFlag ? f + 'e' : f);
                } else {
                    result.push('-e');
                }
            }

            // Threads handling - default to -T0 if none specified
            if (!result.some(f => f.startsWith('-T'))) {
                result.push('-T0');
            }
        }

        if (algorithm === 'zstd') {
            const ultraFlag = result.find(f => f.startsWith('ultra-'));
            if (ultraFlag) {
                const level = ultraFlag.split('-')[1];
                // Remove all normal compression levels and the ultra marker
                result = result.filter(f => !f.startsWith('ultra-') && !/^-(\d+)$/.test(f));
                result.push('-' + level);
                result.push('--ultra');
            }
        }

        return result;
    });

    function toggleFlag(flag: string) {
        if (selectedFlags.includes(flag)) {
            selectedFlags = selectedFlags.filter(f => f !== flag);
        } else {
            selectedFlags = [...selectedFlags, flag];
        }
        onflagsChanged?.(processedFlags);
    }

    function toggleSubFlag(parent: FlagOption, child: string) {
        // For subflags, we often want to replace any existing subflag from the same parent
        // if the parent only allows one selection (like Dictionary Size).
        // For now, let's just handle it as a toggle but with the correct prefix.
        
        const prefix = parent.childPrefix ?? parent.value;
        const suffix = parent.childSuffix ?? "";
        const fullFlag = prefix + child + suffix;
        
        // If it's a exclusive group (most dropdowns are), remove other options from same parent
        if (parent.children) {
            const otherSubFlags = parent.children.map(c => (parent.childPrefix ?? parent.value) + c + (parent.childSuffix ?? ""));
            selectedFlags = selectedFlags.filter(f => !otherSubFlags.includes(f) || f === fullFlag);
        }

        toggleFlag(fullFlag);
    }

    function handleInput(e: Event, flag: FlagOption) {
        const target = e.target as HTMLInputElement;
        const prefix = flag.input?.prefix ?? '';
        
        // Replace existing flag with same prefix
        selectedFlags = selectedFlags.filter(f => !f.startsWith(prefix));
        
        if (target.value.trim() !== "") {
            selectedFlags = [...selectedFlags, prefix + target.value];
        }
        
        onflagsChanged?.(processedFlags);
    }

    $effect(() => {
        algorithm;
        openMenus = {};
        selectedFlags = [];
    });
</script>

<div class="flags-selector" transition:fade|global>
    <h3>Flags for {algorithm}</h3>
    <div class="flag-buttons">
        {#each availableFlags as flag}
            <div class="flag-group">
                <button
                        onclick={() => {
                        if (flag.children || flag.input) {
                            toggleMenu(flag.value);
                        }
                        if (!flag.isPlaceholder) {
                            toggleFlag(flag.value);
                        }
                    }}
                        class:selected={
                            selectedFlags.includes(flag.value) || 
                            (flag.children && flag.children.some(child => selectedFlags.includes((flag.childPrefix ?? flag.value) + child + (flag.childSuffix ?? "")))) ||
                            (flag.input && selectedFlags.some(f => f.startsWith(flag.input.prefix) && (flag.input.prefix !== '-' || /^-(\d+)$/.test(f))))
                        }
                >
                    {flag.label}
                </button>

                {#if openMenus[flag.value]}
                    <div class="dropdown">

                        {#if flag.children}
                            {#each flag.children as child}
                                <button
                                        class="subflag"
                                        onclick={() => toggleSubFlag(flag, child)}
                                        class:selected={selectedFlags.includes((flag.childPrefix ?? flag.value) + child + (flag.childSuffix ?? ""))}
                                >
                                    {child}
                                </button>
                            {/each}
                        {/if}

                        {#if flag.input}
                            <input
                                    class="subinput"
                                    type="text"
                                    placeholder={flag.input.placeholder}
                                    oninput={(e) => handleInput(e, flag)}
                            />
                        {/if}

                    </div>
                {/if}

            </div>
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
    .flag-group {
        position: relative;
    }

    .dropdown {
        display: flex;
        flex-direction: column;
        margin-top: 0.25rem;
        background: #1e1e1e;
        border: 1px solid #444;
        padding: 0.25rem;
    }

    .subflag {
        font-size: 0.85rem;
    }
    .subinput {
        padding: 0.4rem;
        font-size: 0.85rem;
    }
</style>
