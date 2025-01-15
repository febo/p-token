#!/usr/bin/env zx
import 'zx/globals';
import { existsSync } from 'fs';
import { cliArguments, workingDirectory } from './utils.mjs';

// Git repository for the SPL Token program.
const SPL_TOKEN_GIT = "https://github.com/solana-program/token.git";
// Directory where the SPL Token program is cloned. Since it is not meant to be
// checked in, it is cloned in the target directory.
const SPL_TOKEN_DIR = path.join(workingDirectory, 'target', 'spl-token');
// Directory where the fixtures are generated. They get generated everytime the
// `SPL_TOKEN_GIT` repository is cloned to make sure they are up to date.
const FIXTURES_DIR = path.join(workingDirectory, 'target', 'spl-token', 'fixtures');
// Path to the Cargo.toml file of the fixtures CLI program.
const FIXTURES_MANIFEST = path.join(workingDirectory, 'fixtures', 'Cargo.toml');
// Directory where the program binary is found.
const OUTPUT_DIR = path.join(workingDirectory, 'target', 'deploy');
// Directory where the CLI executale is found.
const CLI_OUTPUT_DIR = path.join(workingDirectory, 'target', 'release');

const [command, ...args] = cliArguments();

switch (command) {
    case 'clean':
        await clean();
        break;
    case 'run':
        await run(args);
        break;
    default:
        throw new Error(`Unknown command: ${command}`);
}

async function clean() {
    await $`rm -rf ${SPL_TOKEN_DIR}`;
}

async function run(args) {
    // On first run (or CI), clone the SPL Token program and generate the fixtures.
    // This allows re-runing the fixtures without having to run the tests again. To
    // force re-generating the fixtures, delete the `SPL_TOKEN_DIR` directory.
    if (!existsSync(SPL_TOKEN_DIR)) {
        await $`mkdir ${SPL_TOKEN_DIR}`;
        await $`git clone ${SPL_TOKEN_GIT} ${SPL_TOKEN_DIR}`;

        cd(SPL_TOKEN_DIR);
        // TODO: this can be removed once the mollusk checks PR is merged.
        await $`git switch febo/mollusk-checks`;
        
        await $`EJECT_FUZZ_FIXTURES=${FIXTURES_DIR} cargo test-sbf --features mollusk-svm/fuzz --test processor`;
    }

    cd(workingDirectory);
    // Make sure that the program is up to date.
    await $`pnpm programs:build`;
    // Builds the fixtures CLI program.
    await $`cargo build --manifest-path ${FIXTURES_MANIFEST} --release`;
    // Run the fixtures.
    await $`SBF_OUT_DIR="${OUTPUT_DIR}" ${CLI_OUTPUT_DIR}/fixtures --directory ${FIXTURES_DIR} ${args ? args.join(' ') : ''}`;
}
