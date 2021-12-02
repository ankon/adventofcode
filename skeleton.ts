#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

function processInput(input: string): Promise<void> {
	const rl = createInterface(createReadStream(input));

	return new Promise((resolve, reject) => {
		rl.on('line', line => {
			// Process `line` here
		});
		rl.on('error', err => {
			reject(err);
		});
		rl.on('close', () => {
			console.log(`Results for ${input}:`);
			// Report results for this input

			resolve();
		});
	});
}

async function main(inputFiles: string[]) {
	for (const inputFile of inputFiles) {
		await processInput(inputFile);
	}
}

const INPUT_SPECS = [
	'-example',
	'',
];

main(INPUT_SPECS.map(inputSpec => `${basename(process.argv[1], extname(process.argv[1]))}${inputSpec}.input`)).catch(err => console.error(err));
