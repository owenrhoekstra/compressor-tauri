export class CompressionStore {
    algorithm = $state<string | null>(null);
    flags = $state<string[]>([]);
    inputPath = $state('');
    outputPath = $state('');

    setAlgorithm(algo: string | null) {
        this.algorithm = algo;
        this.flags = [];
        this.inputPath = '';
        this.outputPath = '';
    }

    setFlags(flags: string[]) {
        this.flags = flags;
    }

    setPaths(input: string, output: string) {
        this.inputPath = input;
        this.outputPath = output;
    }

    reset() {
        this.algorithm = null;
        this.flags = [];
        this.inputPath = '';
        this.outputPath = '';
    }
}

export const compressionStore = new CompressionStore();
