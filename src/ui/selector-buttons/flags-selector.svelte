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
            {
                value: "style",
                label: "Output Style",
                children: ["Multi-File", "Single-File"],
                isPlaceholder: true
            },
            {
                value: "--long",
                label: "Long Distance Matching",
                children: ["27", "28", "29", "30", "31"],
                isPlaceholder: true,
                childPrefix: "--long="
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
                isPlaceholder: true,
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
                value: "threads",
                label: "# of Threads",
                isPlaceholder: true,
                input: {
                    placeholder: "0 = auto",
                    prefix: "-mmt"
                }
            },
            {
                value: "-md",
                label: "Dictionary Size",
                isPlaceholder: true,
                children: ["16M", "32M", "64M", "128M", "256M", "512M", "1G", "2G", "4G"],

            }

        ],
        "zpaq": [
            {
                value: "compress",
                label: "Compression Level",
                children: ["1", "2", "3", "4", "5"],
                isPlaceholder: true,
                childPrefix: "-m"
            },
            {
                value: "threads",
                label: "# of Threads",
                isPlaceholder: true,
                input: {
                    placeholder: "0 = auto",
                    prefix: "-t"
                }
            },
            {
                value: "password",
                label: "Password",
                isPlaceholder: true,
                input: {
                    placeholder: "Password",
                    prefix: "-key "
                }
            },
            {
                value: "verbose",
                label: "Verbose",
                children: ["On", "Off"],
                childPrefix: "-s",
                isPlaceholder: true
            }
        ],
        "paq8x": [
            {
                value: "level",
                label: "Compression Level",
                isPlaceholder: true,
                input: {
                    placeholder: "1 - 12, 0z, 0L",
                    prefix: "-"
                }
            },
            {
                value: "-L",
                label: "Enable LSTM"
            },
            {
                value: "-T",
                label: "Pre-Train LSTM on Text"
            },
            {
                value: "-E",
                label: "Pre-Train LSTM on Binaries"
            },
            {
                value: "-A",
                label: "Adaptive Learning"
            },
            {
                value: "-v",
                label: "Verbose"
            }
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
            if (result.includes('styleMulti-File')) {
                result.push('-r');
            }
            result = result.filter(f => !f.startsWith('style'));

            const ultraFlag = result.find(f => f.startsWith('ultra-'));
            if (ultraFlag) {
                const level = ultraFlag.split('-')[1];
                // Remove all normal compression levels and the ultra marker
                result = result.filter(f => !f.startsWith('ultra-') && !/^-(\d+)$/.test(f));
                result.push('-' + level);
                result.push('--ultra');
            }
        }

        if (algorithm === 'zpaq') {
            // Map verbose On/Off to numeric values appended to -s
            const hasVerboseOn = result.includes('-sOn');
            const hasVerboseOff = result.includes('-sOff');
            if (hasVerboseOn || hasVerboseOff) {
                result = result.filter(f => f !== '-sOn' && f !== '-sOff');
                result.push(hasVerboseOn ? '-s1' : '-s0');
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
                            (flag.input && selectedFlags.some(f => f.startsWith(flag.input?.prefix ?? "") && (flag.input?.prefix !== '-' || /^-(\d+)$/.test(f))))
                        }
                >
                    {flag.label}
                </button>

                {#if openMenus[flag.value]}
                    <div class="dropdown" transition:fade>

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
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .flags-selector h3 {
        margin: 0 0 0.5rem 0;
        text-align: center;
    }

    .flag-buttons {
        display: flex;
        gap: 0.6rem;
        row-gap: 0.6rem;
        flex-wrap: wrap;
        justify-content: center;
    }

    button {
        padding: 0.55rem 1rem;
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

    .flag-group {
        position: relative;
    }

    .dropdown {
        position: absolute;
        top: calc(100% + 8px);
        left: 50%;
        transform: translateX(-50%);
        display: flex;
        flex-direction: column;
        background: #1e1e1e;
        border: 1px solid #444;
        padding: 0.4rem;
        border-radius: 8px;
        box-shadow: 0 8px 20px rgba(0,0,0,0.45);
        z-index: 10;
        min-width: 180px;
    }

    .subflag {
        font-size: 0.9rem;
        padding: 0.4rem 0.6rem;
        text-align: left;
    }

    .subinput {
        padding: 0.45rem 0.5rem;
        font-size: 0.9rem;
        border-radius: 6px;
        border: 1px solid #555;
        background: #2a2a2a;
        color: #eee;
    }
</style>
