#!/usr/bin/env node

import { createReadStream } from 'fs';
import { basename, extname } from 'path';
import { createInterface } from 'readline';

function processInput(input: string) {
	const rl = createInterface(createReadStream(input));

	rl.on('line', line => {
		// Process `line` here
	});
	rl.on('close', () => {
		console.log(`Results for ${input}:`);
		// Report results for this input
	});
}

processInput(`${basename(process.argv[1], extname(process.argv[1]))}.input`);
